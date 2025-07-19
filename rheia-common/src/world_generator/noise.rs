
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

use bracket_lib::noise::{FastNoise, FractalType, NoiseType};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

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
