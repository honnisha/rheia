use crate::{
    chunks::{
        block_position::ChunkBlockPosition,
        chunk_data::{BlockDataInfo, ChunkData, ChunkSectionData},
        chunk_position::ChunkPosition,
    },
    default_blocks_ids::BlockID,
    CHUNK_SIZE, VERTICAL_SECTIONS,
};
use bracket_lib::random::RandomNumberGenerator;
use bracket_noise::prelude::*;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use super::traits::IWorldGenerator;

// FRACTAL_TYPE
// - FBM (Fractal Brownian Motion) — классический вариант, когда октавы просто складываются.
//   Даёт мягкие, естественные шумы (например, ландшафты, облака).
//
// - Billow — похоже на FBM, но шум становится «пухлым», волнистым.
//   Часто используется для генерации холмистых форм, где пики становятся более округлыми.
//
// - RigidMulti — жёсткий фрактал. Формирует резкие, «зубчатые» структуры.
//   Применяется для генерации гор с острыми пиками, скал.

// FRACTAL_OCTAVES
// Количество октав — сколько раз шум повторяется на разных масштабах.
// Чем больше октав, тем более детализированным будет результат.
// - Например: octaves = 1 — просто Перлин-шум.
// - octaves = 5 — к базовому шуму добавляется ещё 4 слоя шума с разной детализацией.

// FRACTAL_GAIN
// Коэффициент усиления амплитуды для каждой следующей октавы.
// Он управляет тем, насколько каждая следующая октава влияет на итоговый шум.
// - Если gain близко к 1.0 — октавы сильно влияют.
// - Если gain меньше 0.5 — каждая следующая октава заметно слабее предыдущей.

// FRACTAL_LACUNARITY
// Коэффициент увеличения частоты для каждой следующей октавы.
// - Если lacunarity = 2.0 — каждая новая октава в 2 раза «мельче» по частоте.
// - Высокое значение даёт больше мелких деталей.

// FREQUENCY
// Базовая частота шума. Определяет, насколько «часто» повторяются волны шума.
// - Маленькая частота (frequency = 0.01) — большие плавные формы.
// - Большая частота (frequency = 1.0) — много мелких деталей.

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CFractalType {
    Fbm,
    Billow,
    RigidMulti,
}

impl Default for CFractalType {
    fn default() -> Self {
        Self::Fbm
    }
}

impl CFractalType {
    pub fn orig(&self) -> FractalType {
        match *self {
            CFractalType::Fbm => FractalType::FBM,
            CFractalType::Billow => FractalType::Billow,
            CFractalType::RigidMulti => FractalType::RigidMulti,
        }
    }
}

#[serde_inline_default]
#[derive(Default, Serialize, Deserialize)]
pub struct Noise {
    #[serde_inline_default(CFractalType::Fbm)]
    pub fractal_type: CFractalType,
    #[serde_inline_default(4)]
    pub fractal_octaves: i32,
    #[serde_inline_default(0.5)]
    pub fractal_gain: f32,
    #[serde_inline_default(2.0)]
    pub fractal_lacunarity: f32,
    #[serde_inline_default(0.03)]
    pub frequency: f32,
    #[serde_inline_default(1.0)]
    pub miltiplier: f32,
}

pub struct GeneratedNoise {
    noise: FastNoise,
    miltiplier: f32,
}

impl GeneratedNoise {
    pub fn get_noise(&self, x: f32, y: f32) -> f32 {
        (self.noise.get_noise(x, y) * self.miltiplier).max(0.0).min(1.0)
    }
}

impl Noise {
    pub fn generate(&self, seed: u64) -> GeneratedNoise {
        let mut noise = FastNoise::seeded(seed);
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(self.fractal_type.orig());
        noise.set_fractal_octaves(self.fractal_octaves);
        noise.set_fractal_gain(self.fractal_gain);
        noise.set_fractal_lacunarity(self.fractal_lacunarity);
        noise.set_frequency(self.frequency);
        GeneratedNoise { noise, miltiplier: self.miltiplier.clone() }
    }
}

#[serde_inline_default]
#[derive(Default, Serialize, Deserialize)]
pub struct WorldGeneratorSettings {
    surface_noise: Noise,
    #[serde_inline_default(10.0)]
    surface_noise_multiplier: f32,

    river_noise: Noise,
    #[serde_inline_default(0.1)]
    river_threshold: f32,
    #[serde_inline_default(3.0)]
    river_miltiplier: f32,

    stream_noise: Noise,
    #[serde_inline_default(0.1)]
    stream_threshold: f32,
    #[serde_inline_default(3.0)]
    stream_miltiplier: f32,
}

pub struct WorldGenerator {
    surface_noise: GeneratedNoise,
    river_noise: GeneratedNoise,
    stream_noise: GeneratedNoise,
    settings: WorldGeneratorSettings,
}

impl IWorldGenerator for WorldGenerator {
    type Error = String;

    fn create(seed: Option<u64>, settings: WorldGeneratorSettings) -> Result<Self, Self::Error> {
        let seed = match seed {
            Some(s) => s,
            None => {
                let mut rng = RandomNumberGenerator::new();
                rng.next_u64()
            }
        };

        Ok(Self {
            surface_noise: settings.surface_noise.generate(seed),
            river_noise: settings.river_noise.generate(seed),
            stream_noise: settings.stream_noise.generate(seed),
            settings: settings,
        })
    }

    fn generate_chunk_data(&self, chunk_position: &ChunkPosition) -> ChunkData {
        let mut chunk_data: ChunkData = Default::default();
        for y in 0..VERTICAL_SECTIONS {
            let chunk_section = self.generate_section_data(&chunk_position, y);
            chunk_data.push_section(chunk_section);
        }
        chunk_data
    }
}

impl WorldGenerator {
    fn generate_section_data(&self, chunk_position: &ChunkPosition, vertical_index: usize) -> ChunkSectionData {
        let mut section_data: ChunkSectionData = Default::default();

        for x in 0_u8..(CHUNK_SIZE as u8) {
            for z in 0_u8..(CHUNK_SIZE as u8) {
                let x_map = x as f32 + (chunk_position.x as f32 * CHUNK_SIZE as f32);
                let z_map = z as f32 + (chunk_position.z as f32 * CHUNK_SIZE as f32);
                let height =
                    self.surface_noise.get_noise(x_map, z_map) * self.settings.surface_noise_multiplier + 60_f32;

                // Множитель для рек, превращающий их в реки
                let river_noise = self.river_noise.get_noise(x_map, z_map);

                // Реки
                let stream_noise = (self.stream_noise.get_noise(x_map, z_map) - 1.0).abs();
                let stream_mut = 1.0 + match stream_noise > self.settings.stream_threshold {
                    true => stream_noise * self.settings.stream_miltiplier,
                    false => 0.0,
                };

                for y in 0_u8..(CHUNK_SIZE as u8) {
                    let pos = ChunkBlockPosition::new(x, y, z);

                    let y_global = y as f32 + (vertical_index as f32 * CHUNK_SIZE as f32);

                    if height > y_global * stream_mut {
                        section_data.insert(&pos, BlockDataInfo::create(BlockID::Grass.id(), None));

                        if river_noise > self.settings.river_threshold {
                            section_data.insert(&pos, BlockDataInfo::create(BlockID::Stone.id(), None));
                        }

                        if stream_noise > self.settings.stream_threshold {
                            section_data.insert(&pos, BlockDataInfo::create(BlockID::Sand.id(), None));
                        }
                    }
                }
            }
        }
        return section_data;
    }
}

#[cfg(test)]
mod tests {
    use super::{WorldGenerator, WorldGeneratorSettings};
    use crate::world_generator::traits::IWorldGenerator;

    #[test]
    fn test_generation_stream() {
        let settings: WorldGeneratorSettings = serde_yaml::from_str("").unwrap();
        let generator = WorldGenerator::create(Some(40), settings).unwrap();

        for y in 0..50 {
            for x in 0..50 {
                let n = generator.stream_noise.get_noise(x as f32 * 10.0, y as f32 * 10.0);
                if n > 0.1 {
                    print!("1 ");
                } else {
                    print!("0 ");
                }
            }
            println!("");
        }

        let river = generator.stream_noise.get_noise(100.0, 100.0);
        assert_eq!(river, 1.0);
    }
}
