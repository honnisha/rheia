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

const GROUND_LEVEL: f32 = 60.0;

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
    #[serde_inline_default(None)]
    pub powf: Option<f32>,
}

pub struct GeneratedNoise {
    noise: FastNoise,
    miltiplier: f32,
    powf: Option<f32>,
}

impl GeneratedNoise {
    pub fn get_noise(&self, x: f32, y: f32) -> f32 {
        let mut r = (self.noise.get_noise(x, y) * self.miltiplier).max(0.0).min(1.0);
        r = match self.powf {
            Some(p) => r.powf(p),
            None => r,
        };
        r
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
        GeneratedNoise {
            noise,
            miltiplier: self.miltiplier.clone(),
            powf: self.powf,
        }
    }
}

#[serde_inline_default]
#[derive(Default, Serialize, Deserialize)]
pub struct WorldGeneratorSettings {
    surface_noise: Noise,
    #[serde_inline_default(10.0)]
    surface_multiplier: f32,

    river_noise: Noise,
    #[serde_inline_default(10.0)]
    river_multiplier: f32,

    stream_noise: Noise,
    #[serde_inline_default(10.0)]
    stream_multiplier: f32,

    #[serde_inline_default(5.0)]
    sand_threshold: f32,
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
                    self.surface_noise.get_noise(x_map, z_map) * self.settings.surface_multiplier + GROUND_LEVEL;

                // Множитель для рек, превращающий их в реки
                let river_noise = self.river_noise.get_noise(x_map, z_map);

                // Реки
                let stream_noise = self.stream_noise.get_noise(x_map, z_map);
                let stream = (stream_noise * (1.0 + river_noise)) * self.settings.stream_multiplier;

                for y in 0_u8..(CHUNK_SIZE as u8) {
                    let pos = ChunkBlockPosition::new(x, y, z);

                    let y_global = y as f32 + (vertical_index as f32 * CHUNK_SIZE as f32);

                    if y_global < height - stream {
                        section_data.insert(&pos, BlockDataInfo::create(BlockID::Grass.id(), None));

                        if stream > self.settings.sand_threshold {
                            section_data.insert(&pos, BlockDataInfo::create(BlockID::Sand.id(), None));
                        }
                    }
                }
            }
        }
        return section_data;
    }
}
