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
enum CFractalType {
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
    fn orig(&self) -> FractalType {
        match *self {
            CFractalType::Fbm => FractalType::FBM,
            CFractalType::Billow => FractalType::Billow,
            CFractalType::RigidMulti => FractalType::RigidMulti,
        }
    }
}

#[serde_inline_default]
#[derive(Default, Serialize, Deserialize)]
pub struct WorldGeneratorSettings {
    #[serde_inline_default(2)]
    surface_fractal_octaves: i32,
    #[serde_inline_default(2.0)]
    surface_fractal_gain: f32,
    #[serde_inline_default(1.1)]
    surface_fractal_lacunarity: f32,
    #[serde_inline_default(0.05)]
    surface_frequency: f32,
    #[serde_inline_default(10.0)]
    surface_noise_multiplier: f32,

    #[serde_inline_default(CFractalType::Fbm)]
    river_fractal_type: CFractalType,
    #[serde_inline_default(4)]
    river_fractal_octaves: i32,
    #[serde_inline_default(0.5)]
    river_fractal_gain: f32,
    #[serde_inline_default(2.0)]
    river_fractal_lacunarity: f32,
    #[serde_inline_default(0.03)]
    river_frequency: f32,
    #[serde_inline_default(0.3)]
    river_threshold: f32,

    #[serde_inline_default(CFractalType::Fbm)]
    stream_fractal_type: CFractalType,
    #[serde_inline_default(4)]
    stream_fractal_octaves: i32,
    #[serde_inline_default(0.5)]
    stream_fractal_gain: f32,
    #[serde_inline_default(2.0)]
    stream_fractal_lacunarity: f32,
    #[serde_inline_default(0.03)]
    stream_frequency: f32,
    #[serde_inline_default(0.3)]
    stream_threshold: f32,
    #[serde_inline_default(3.0)]
    stream_miltiplier: f32,
}

pub struct WorldGenerator {
    surface_noise: FastNoise,
    river_noise: FastNoise,
    stream_noise: FastNoise,
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

        let mut surface_noise = FastNoise::seeded(seed);
        surface_noise.set_noise_type(NoiseType::PerlinFractal);
        surface_noise.set_fractal_type(FractalType::FBM);
        surface_noise.set_fractal_octaves(settings.surface_fractal_octaves);
        surface_noise.set_fractal_gain(settings.surface_fractal_gain);
        surface_noise.set_fractal_lacunarity(settings.surface_fractal_lacunarity);
        surface_noise.set_frequency(settings.surface_frequency);

        let mut river_noise = FastNoise::seeded(seed);
        river_noise.set_noise_type(NoiseType::PerlinFractal);
        river_noise.set_fractal_type(settings.river_fractal_type.orig());
        river_noise.set_fractal_octaves(settings.river_fractal_octaves);
        river_noise.set_fractal_gain(settings.river_fractal_gain);
        river_noise.set_fractal_lacunarity(settings.river_fractal_lacunarity);
        river_noise.set_frequency(settings.river_frequency);

        let mut stream_noise = FastNoise::seeded(seed);
        stream_noise.set_noise_type(NoiseType::PerlinFractal);
        stream_noise.set_fractal_type(settings.stream_fractal_type.orig());
        stream_noise.set_fractal_octaves(settings.stream_fractal_octaves);
        stream_noise.set_fractal_gain(settings.stream_fractal_gain);
        stream_noise.set_fractal_lacunarity(settings.stream_fractal_lacunarity);
        stream_noise.set_frequency(settings.stream_frequency);

        Ok(Self {
            surface_noise,
            river_noise,
            stream_noise,
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
                let river = self.river_noise.get_noise(x_map, z_map);
                let river_mut = 1.0 + (river - self.settings.river_threshold).max(0.0);

                // Реки
                let stream = self.stream_noise.get_noise(x_map, z_map);
                let stream = (stream - self.settings.stream_threshold).max(0.0) * self.settings.stream_miltiplier * river_mut;

                for y in 0_u8..(CHUNK_SIZE as u8) {
                    let pos = ChunkBlockPosition::new(x, y, z);

                    let y_global = y as f32 + (vertical_index as f32 * CHUNK_SIZE as f32);

                    if height > y_global - stream {
                        section_data.insert(&pos, BlockDataInfo::create(BlockID::Grass.id(), None));
                    }
                    if stream > 1.0 {
                        section_data.insert(&pos, BlockDataInfo::create(BlockID::Sand.id(), None));
                    }
                }
            }
        }
        return section_data;
    }
}
