static PSIZE: usize = 2048;
static PMASK: u32 = 2047;

/// Implementation of OpenSimplex 2 Noise, the smooth variant ("SuperSimplex")
/// based on the most recent version available at the repository https://github.com/KdotJPG/OpenSimplex2
/// developed by KdotJPG, who originally released the OpenSimplex Noise under a Public Domain license.
///
/// This is a port of the Java / C# code from the linked repository.
///
pub struct OpenSimplexNoise {
    /// The seed to start rng from
    pub seed: i64,
    /// Array of unsigned short entries (1-dimensional?)
    perm: Vec<u16>,
    // Array of 2-dimensional float values (x, y)
    // permGrad2: Vec<(f64, f64)>,
}

impl OpenSimplexNoise {
    /// Constructor
    pub fn new(seed: i64) -> Self {
        let mut perm = Vec::new();

        let mut source: Vec<u16> = Vec::with_capacity(PSIZE);
        for index in 0..PSIZE {
            source[index] = index as u16;
        }

        for i in (0..PSIZE).rev() {
            let seed = seed * 6364136223846793005 + 1442695040888963407;
            let mut r: i32 = (seed as i32 + 31) % (i as i32 + 1);
            if r < 0 {
                r += i as i32 + 1;
            }

            perm[i] = source[r as usize];
            source[r as usize] = source[i];
        }

        Self {
            seed,
            perm,
        }
    }
}
