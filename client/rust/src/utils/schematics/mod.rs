use fastnbt::{ByteArray, IntArray, Value};
use flate2::read::GzDecoder;
use ndshape::ConstShape;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::{collections::HashMap, io::Read, path::PathBuf};

use crate::world::chunks::chunk_info::{ChunkInfo, ChunkShape};
use crate::world::{blocks::minecraft_types::block_type_from_minecraft_name, chunks::block_info::BlockInfo};

// https://github.com/SpongePowered/Schematic-Specification

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(non_snake_case)]
pub struct Metadata {
    name: Option<String>,
    author: Option<String>,
    WEOffsetX: Option<i32>,
    WEOffsetY: Option<i32>,
    WEOffsetZ: Option<i32>,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockEntity {
    pos: IntArray,
    id: String,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SchemData {
    version: u32,
    data_version: u32,
    pub width: u32,
    pub height: u32,
    pub length: u32,
    pub offset: Option<IntArray>,
    palette: HashMap<String, Value>,
    block_entities: Vec<BlockEntity>,
    block_data: ByteArray,
    metadata: Option<Metadata>,
}

impl SchemData {
    pub fn remap_palette(&self) -> HashMap<i64, BlockInfo> {
        let mut result = HashMap::new();
        for p in &self.palette {
            let block_name = match parse_block_id(p.0) {
                Some(e) => e,
                _ => {
                    println!("Block \"{}\" regex error", p.0);
                    continue;
                }
            };

            let block_type = match block_type_from_minecraft_name(&block_name) {
                Some(e) => e,
                _ => {
                    println!("Minecraft block \"{}\" not found", block_name);
                    continue;
                }
            };

            result.insert(p.1.as_i64().unwrap(), BlockInfo::new(block_type));
        }
        result
    }

    pub fn get_local_offset(&self) -> (i32, i32, i32) {
        match self.metadata.as_ref() {
            Some(m) => (
                match m.WEOffsetX {
                    Some(e) => e,
                    _ => 0_i32,
                },
                match m.WEOffsetY {
                    Some(e) => e,
                    _ => 0_i32,
                },
                match m.WEOffsetZ {
                    Some(e) => e,
                    _ => 0_i32,
                },
            ),
            _ => (0_i32, 0_i32, 0_i32),
        }
    }
}

pub fn load_schem_data(path: &PathBuf) -> Result<SchemData, String> {
    let now = Instant::now();
    let filename = path.clone().into_os_string().into_string().unwrap();
    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Cant open file \"{}\": {}", filename, e).into()),
    };

    let mut decoder = GzDecoder::new(file);
    let mut bytes = vec![];
    match decoder.read_to_end(&mut bytes) {
        Ok(u) => u,
        Err(e) => return Err(format!("Cant read \"{}\": {}", filename, e).into()),
    };

    match fastnbt::from_bytes(&bytes) {
        Ok(d) => {
            println!("Schem {:?} loaded in {:.2?}", path, now.elapsed());
            Ok(d)
        }
        Err(e) => return Err(format!("fastnbt::from_bytes file error \"{}\": {}", filename, e).into()),
    }
}

pub fn parse_block_id(block_id: &String) -> Option<&str> {
    let re = Regex::new(r"[a-z]+:([a-zA-Z_]+)(?:\[[a-zA-Z0-9=,]+\])?").unwrap();
    let caps = re.captures(block_id).unwrap();
    match caps.get(1) {
        Some(e) => Some(e.as_str()),
        _ => None,
    }
}

pub fn convert_schem_to_blockinfo(anchor: &[i32; 3], schem: &SchemData) -> HashMap<[i32; 3], HashMap<u32, BlockInfo>> {
    let now = Instant::now();
    let palette_map = schem.remap_palette();

    let mut result = HashMap::new();

    let offset = schem.get_local_offset();

    let mut index = 0_u32;
    let mut i = 0;
    let mut value;
    let mut varint_length;

    while i < schem.block_data.len() {
        value = 0_i64;
        varint_length = 0_i64;

        loop {
            value |= (schem.block_data[i] as i64 & 127_i64) << (varint_length * 7_i64);
            varint_length += 1;
            if varint_length > 5 {
                panic!("VarInt too big (probably corrupted data)");
            }
            if (schem.block_data[i] as i64 & 128) != 128 {
                i += 1;
                break;
            }
            i += 1;
        }
        // index = (y * length + z) * width + x
        let _y = index / (schem.width * schem.length);
        let _z = (index % (schem.width * schem.length)) / schem.width;
        let _x = (index % (schem.width * schem.length)) % schem.width;

        let x = anchor[0] + (_x as i32 + offset.0);
        let y = anchor[1] + (_y as i32 + offset.1);
        let z = anchor[2] + (_z as i32 + offset.2);

        if let Some(e) = palette_map.get(&(value as i64)) {
            let global_pos = [x, y, z];
            let chunk_pos = ChunkInfo::get_chunk_pos_by_global(&global_pos);
            let chunk = result.entry(chunk_pos).or_insert(HashMap::new());

            let local_pos = ChunkInfo::get_chunk_local_pos_from_global(&global_pos);
            let i = ChunkShape::linearize(local_pos);

            chunk.insert(i, e.clone());
        }
        index += 1;
    }
    println!("convert_schem_to_blockinfo: completed in {:.2?}", now.elapsed());
    result
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use fastnbt::Value;

    use crate::world::blocks::blocks_storage::BlockType;
    use crate::world::chunks::block_info::BlockInfo;

    use super::{convert_schem_to_blockinfo, load_schem_data};

    #[test]
    fn test_load_schem_data() {
        let mut path = env::current_dir().unwrap().clone();
        path.push("tests");
        path.push("large.schem");

        let schem_data_result = load_schem_data(&path);
        assert_eq!(schem_data_result.is_ok(), true, "error: {:?}", schem_data_result.err());

        let schem_data = schem_data_result.unwrap();
        let palette_map = schem_data.remap_palette();

        assert_eq!(schem_data.version, 2_u32);
        assert_eq!(schem_data.data_version, 2865_u32);
        assert_eq!(schem_data.width, 315_u32);
        assert_eq!(schem_data.height, 151_u32);
        assert_eq!(schem_data.length, 195_u32);

        assert_eq!(schem_data.offset.is_some(), true);
        let offset = schem_data.offset.unwrap();
        assert_eq!(offset.len(), 3_usize);
        assert_eq!((offset[0], offset[1], offset[2]), (-129_i32, -64_i32, -48_i32));

        assert_eq!(schem_data.block_data.len(), 9535329_usize);

        assert_eq!(schem_data.palette.len(), 842_usize);
        assert_eq!(palette_map[&0], BlockInfo::new(BlockType::Air));

        assert_eq!(schem_data.block_entities.len(), 3416_usize);

        let block_entry = schem_data.block_entities.get(3300).unwrap();
        assert_eq!(block_entry.id, "minecraft:sign");
        assert_eq!(block_entry.pos.len(), 3_usize);
        assert_eq!(
            (block_entry.pos[0], block_entry.pos[1], block_entry.pos[2]),
            (114_i32, 59_i32, 93_i32)
        );
        let mut extra: HashMap<String, Value> = HashMap::new();
        extra.insert("Text1".into(), Value::String("{\"text\":\"\"}".into()));
        extra.insert("Text3".into(), Value::String("{\"text\":\"\"}".into()));
        extra.insert("Text2".into(), Value::String("{\"text\":\"\"}".into()));
        extra.insert("Text4".into(), Value::String("{\"text\":\"\"}".into()));
        extra.insert("Color".into(), Value::String("black".into()));
        extra.insert("GlowingText".into(), Value::Byte(0));
        assert_eq!(block_entry.other, extra);
    }

    #[test]
    fn test_convert() {
        let mut path = env::current_dir().unwrap().clone();
        path.push("tests");
        path.push("small.schem");

        let schem_data_result = load_schem_data(&path);
        let schem = schem_data_result.unwrap();
        let modify_data = convert_schem_to_blockinfo(&[0_i32, 80_i32, 0_i32], &schem);
        assert_eq!(modify_data.len(), 72171_usize);
        //assert_eq!(modify_data[&[-20_i32, 110_i32, 31_i32]], BlockInfo::new(BlockType::Air));
    }
}
