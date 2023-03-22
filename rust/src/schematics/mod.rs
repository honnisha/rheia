use fastnbt::{IntArray, Value};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read, path::PathBuf};

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
    width: u32,
    height: u32,
    length: u32,
    offset: Option<IntArray>,
    block_entities: Vec<BlockEntity>,
}

pub fn load_schem_data(path: &PathBuf) -> Result<SchemData, String> {
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
        Ok(d) => Ok(d),
        Err(e) => {
            return Err(format!("fastnbt::from_bytes file error \"{}\": {}", filename, e).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use fastnbt::Value;

    use super::load_schem_data;

    #[test]
    fn test_load_schem_data() {
        let mut path = env::current_dir().unwrap().clone();
        path.push("tests");
        path.push("test.schem");

        let schem_data_result = load_schem_data(&path);
        assert_eq!(
            schem_data_result.is_ok(),
            true,
            "error: {:?}",
            schem_data_result.err()
        );

        let schem_data = schem_data_result.unwrap();
        assert_eq!(schem_data.version, 2_u32);
        assert_eq!(schem_data.data_version, 2865_u32);
        assert_eq!(schem_data.width, 315_u32);
        assert_eq!(schem_data.height, 151_u32);
        assert_eq!(schem_data.length, 195_u32);

        assert_eq!(schem_data.offset.is_some(), true);
        let offset = schem_data.offset.unwrap();
        assert_eq!(offset.len(), 3_usize);

        assert_eq!(schem_data.block_entities.len(), 3416_usize);

        let block = schem_data.block_entities.get(3300).unwrap();
        assert_eq!(block.id, "minecraft:sign");
        assert_eq!(block.pos.len(), 3_usize);
        assert_eq!(
            (block.pos[0], block.pos[1], block.pos[2]),
            (114_i32, 59_i32, 93_i32)
        );
        let mut extra: HashMap<String, Value> = HashMap::new();
        extra.insert("Text1".into(), Value::String("{\"text\":\"\"}".into()));
        extra.insert("Text3".into(), Value::String("{\"text\":\"\"}".into()));
        extra.insert("Text2".into(), Value::String("{\"text\":\"\"}".into()));
        extra.insert("Text4".into(), Value::String("{\"text\":\"\"}".into()));
        extra.insert("Color".into(), Value::String("black".into()));
        extra.insert("GlowingText".into(), Value::Byte(0));
        assert_eq!(block.other, extra);
    }
}
