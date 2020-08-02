const PSIZE: usize = 2048;
const PMASK: u32 = 2047;

const N2: f64 = 0.05481866495625118;

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
    permGrad2: Vec<(f64, f64)>,
}

impl OpenSimplexNoise {
    /// Constructs a new instance based on the given seed
    pub fn new(seed: i64) -> Self {
        let mut perm = Vec::with_capacity(PSIZE);
        let mut permGrad2 = Vec::with_capacity(PSIZE);

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
            permGrad2,
        }
    }

    /// 2D SuperSimplex noise, standard lattice orientation
    ///
    /// returns a noise value between -1..+1
    ///
    pub fn noise2(x: f64, y: f64) -> f64 {
        0.0
    }

    /// 2D SuperSimplex noise, with Y pointing down the main diagonal.
    /// Might be better for a 2D sandbox style game, where Y is vertical.
    /// Probably slightly less optimal for heightmaps or continent maps.
    ///
    /// returns a noise value between -1..+1
    ///
    pub fn noise2_x_before_y(x: f64, y: f64) -> f64 {
        0.0
    }

    /// 2D SuperSimplex noise base.
    /// Lookup table implementation inspired by DigitalShadow.
    ///
    /// It is based on the refactored implementation to improve performance,
    /// see https://gist.github.com/digitalshadow/134a3a02b67cecd72181.
    ///
    /// returns a value between -1..+1
    ///
    fn noise2_base(xs: f64, ys: f64) -> f64 {
        0.0
    }

    /// Looks up entry from gradient table and returns pair (x, y) floats
    fn gradients_2d(index: usize) -> (f64, f64) {
        (0.0, 0.0)
    }
}

// For now this macro helps to initialize the gradients tables once for use.
// There is a RFC to introduce logic to initialize static variables via `once_cell`.
// For details on the progress, see the discussion https://github.com/rust-lang/rfcs/pull/2788
// Until then the [lazy_static](https://docs.rs/lazy_static/1.4.0/lazy_static/) crate is used to initialize
// the lookup tables similar to the Java static initializer.
//
lazy_static! {
    static ref GRADIENTS_2D: Vec<(f64, f64)> = {
        let mut grad2 = vec![
            (0.130526192220052,  0.99144486137381),
            (0.38268343236509,   0.923879532511287),
            (0.608761429008721,  0.793353340291235),
            (0.793353340291235,  0.608761429008721),
            (0.923879532511287,  0.38268343236509),
            (0.99144486137381,   0.130526192220051),
            (0.99144486137381,  -0.130526192220051),
            (0.923879532511287, -0.38268343236509),
            (0.793353340291235, -0.60876142900872),
            (0.608761429008721, -0.793353340291235),
            (0.38268343236509,  -0.923879532511287),
            (0.130526192220052, -0.99144486137381),
            (0.130526192220052, -0.99144486137381),
            (0.38268343236509,  -0.923879532511287),
            (0.608761429008721, -0.793353340291235),
            (0.793353340291235, -0.608761429008721),
            (0.923879532511287, -0.38268343236509),
            (0.99144486137381,  -0.130526192220052),
            (0.99144486137381,   0.130526192220051),
            (0.923879532511287,  0.38268343236509),
            (0.793353340291235,  0.608761429008721),
            (0.608761429008721,  0.793353340291235),
            (0.38268343236509,   0.923879532511287),
            (0.130526192220052,  0.99144486137381)
        ];

        for i in 0..grad2.len() {
            grad2[i].0 /= N2;
            grad2[i].1 /= N2;
        }

        let mut gradients = Vec::with_capacity(PSIZE);
        for i in 0..PSIZE {
            gradients[i] = grad2[i & grad2.len()];
        }
        gradients
    };
}
