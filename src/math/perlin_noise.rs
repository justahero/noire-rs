use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Gradian function with hash
fn grad(hash: i32, x: f64, y: f64, z: f64) -> f64 {
    match 0xF & hash {
        0x0 =>  x + y,
        0x1 => -x + y,
        0x2 =>  x - y,
        0x3 => -x - y,
        0x4 =>  x + z,
        0x5 => -x + z,
        0x6 =>  x - z,
        0x7 => -x - z,
        0x8 =>  y + z,
        0x9 => -y + z,
        0xA =>  y - z,
        0xB => -y - z,
        0xC =>  y + x,
        0xD => -y + z,
        0xE =>  y - x,
        _ => -y - z,
    }
}

/// Fades the given value
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Linear interpolation between a..b
fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a + x * (b - a)
}

/// Implementation of Perlin Noise
/// Based on: https://mrl.nyu.edu/~perlin/noise/ and https://gist.github.com/Flafla2/f0260a861be0ebdeef76
///
/// The algorithm is using the rand crate with rand_chacha rng to
/// use a seed based random generator. See the official book for details:
/// https://rust-random.github.io/book/guide-start.html
///
pub struct PerlinNoise {
    /// The seeded random generator
    pub seed: u64,
    /// A table of randomly generated values
    table: Vec<i32>,
    /// If set, defines repetition of Perlin Noise pattern
    repeat: Option<u32>,
    /// The number of octaves to smooth noise functions
    octaves: u16,
    /// The amplitude fall off
    ampl_falloff: f64,
}

/// Generate a table with random values
/// If fills a Vec with size number of random values between 0.0 and 1.0
///
fn generate_table(rng: &mut ChaCha8Rng, size: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(2 * size);

    // generate random numbers between 0..size
    for i in 0..size {
        let value = rng.gen_range(0, size) as i32;
        result.push(value);
    }

    // repeat these numbers to avoid overflow
    for i in 0..size {
        let value = result[i];
        result.push(value);
    }

    result
}

impl PerlinNoise {
    /// Generates a new Perlin Noise set from given seed
    pub fn new(seed: u64) -> Self {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let table = generate_table(&mut rng, 256);

        PerlinNoise {
            seed,
            table,
            repeat: None,
            octaves: 4,
            ampl_falloff: 0.5,
        }
    }

    /// Generates a 1-dimensional random number
    /// Returns a value between 0..1
    pub fn gen1(&self, x: f64) -> f64 {
        self.perlin_3d(x, 0.0, 0.0)
    }

    /// Generates a random number from 2 components
    /// Returns a value between 0..1
    pub fn gen2(&self, x: f64, y: f64) -> f64 {
        self.perlin_3d(x, y, 0.0)
    }

    /// Generates a new random noise number based on three components
    /// Returns a value between 0..1
    pub fn gen3(&self, x: f64, y: f64, z: f64) -> f64 {
        self.perlin_3d(x, y, z)
    }

    /// The main function to generate a noise value from three components
    ///
    fn perlin_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value  = 0.0;

        for _i in 0..self.octaves {
            total += self.perlin(x * frequency, y * frequency, z * frequency) * amplitude;

            max_value += amplitude;
            amplitude *= self.ampl_falloff;
            frequency *= 2.0;
        }

        total / max_value
    }

    fn inc(&self, num: i32) -> i32 {
        num + 1
    }

    /// The function to calculate a noise value
    /// Returns a value between -1..+1
    fn perlin(&self, x: f64, y: f64, z: f64) -> f64 {
        let xi = (x.floor() as i32) & 255;
        let yi = (y.floor() as i32) & 255;
        let zi = (z.floor() as i32) & 255;

        let aaa: i32 = self.p(self.p(self.p(    xi ) +               yi ) +               zi );
        let aba: i32 = self.p(self.p(self.p(    xi ) + self.inc(yi)) +               zi );
        let aab: i32 = self.p(self.p(self.p(    xi ) +               yi ) + self.inc(zi));
        let abb: i32 = self.p(self.p(self.p(    xi ) + self.inc(yi)) + self.inc(zi));
        let baa: i32 = self.p(self.p(self.p(self.inc(xi)) +     yi ) +     zi );
        let bba: i32 = self.p(self.p(self.p(self.inc(xi)) + self.inc(yi)) +     zi );
        let bab: i32 = self.p(self.p(self.p(self.inc(xi)) +     yi ) + self.inc(zi));
        let bbb: i32 = self.p(self.p(self.p(self.inc(xi)) + self.inc(yi)) + self.inc(zi));

        let x = x - x.floor();
        let y = y - y.floor();
        let z = z - z.floor();

        let u = fade(x);
        let v = fade(y);
        let w = fade(z);

        let x1 = lerp(
            grad(aaa, x      , y    , z),
            grad(baa, x - 1.0, y    , z), u);
        let x2 = lerp(
            grad(aba, x      , y - 1.0, z),
            grad(bba, x - 1.0, y - 1.0, z), u);

        let y1 = lerp(x1, x2, v);

        let x1 = lerp(
            grad(aab, x      , y    , z - 1.0),
            grad(bab, x - 1.0, y    , z - 1.0), u);
        let x2 = lerp(
            grad(abb, x      , y - 1.0, z - 1.0),
            grad(bbb, x - 1.0, y - 1.0, z - 1.0), u);
        let y2 = lerp(x1, x2, v);

        (lerp(y1, y2, w) + 1.0) / 2.0
    }

    /// lookup into permutation table
    fn p(&self, index: i32) -> i32 {
        self.table[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::PerlinNoise;

    #[test]
    fn generates_value() {
        let perlin = PerlinNoise::new(0);
        assert!(perlin.gen1(5.1) > 0.0);
    }
}
