const PSIZE: usize = 2048;
const PMASK: u32 = 2047;

const N2: f64 = 0.05481866495625118;
const N3: f64 = 0.2781926117527186;
const N4: f64 = 0.11127401889945551;

#[inline(always)]
fn fastFloor(x: f64) -> i32 {
    let xi = x.floor() as i32;
    if x < xi as f64 {
        xi - 1
    } else {
        xi
    }
}

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
    /// Array of 2-dimensional float values (x, y)
    permGrad2: Vec<(f64, f64)>,
    /// Array of 3-dimensional float values (x, y, z)
    permGrad3: Vec<(f64, f64, f64)>,
}

impl OpenSimplexNoise {
    /// Constructs a new instance based on the given seed
    pub fn new(seed: i64) -> Self {
        let mut perm = Vec::with_capacity(PSIZE);
        let mut permGrad2 = Vec::with_capacity(PSIZE);
        let mut permGrad3 = Vec::with_capacity(PSIZE);

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
            permGrad2[i] = GRADIENTS_2D[perm[i] as usize];
            permGrad3[i] = GRADIENTS_3D[perm[i] as usize];
            source[r as usize] = source[i];
        }

        Self {
            seed,
            perm,
            permGrad2,
            permGrad3,
        }
    }

    /// 2D SuperSimplex noise, standard lattice orientation
    ///
    /// returns a noise value between -1..+1
    ///
    pub fn noise2(&self, x: f64, y: f64) -> f64 {
        // Get points for A2* lattice
        let s = 0.366025403784439 * (x + y);
        let xs = x + s;
        let ys = y + s;

        self.noise2_base(xs, ys)
    }

    /// 2D SuperSimplex noise, with Y pointing down the main diagonal.
    /// Might be better for a 2D sandbox style game, where Y is vertical.
    /// Probably slightly less optimal for heightmaps or continent maps.
    ///
    /// returns a noise value between -1..+1
    ///
    pub fn noise2_x_before_y(&self, x: f64, y: f64) -> f64 {
        // Skew transform and rotation baked into one.
        let xx = x * 0.7071067811865476;
        let yy = y * 1.224744871380249;

        self.noise2_base(yy + xx, yy - xx)
    }

    /// 2D SuperSimplex noise base.
    /// Lookup table implementation inspired by DigitalShadow.
    ///
    /// It is based on the refactored implementation to improve performance,
    /// see https://gist.github.com/digitalshadow/134a3a02b67cecd72181.
    ///
    /// returns a value between -1..+1
    ///
    fn noise2_base(&self, xs: f64, ys: f64) -> f64 {
        // Get base points and offsets
        let xsb = fastFloor(xs);
        let ysb = fastFloor(ys);

        let xsi = xs - xsb as f64;
        let ysi = ys - ysb as f64;

        // Index to point list
        let a = (xsi + ysi) as i32;
        let index =
            (a << 2) |
            ((xsi - ysi / 2.0 + 1.0 - a as f64 / 2.0) as i32) << 3 |
            ((ysi - xsi / 2.0 + 1.0 - a as f64 / 2.0) as i32) << 4;

        let ssi = (xsi + ysi) * -0.211324865405187;
        let xi = xsi + ssi;
        let yi = ysi + ssi;

        // Point contributions
        let mut value = 0.0;
        for i in 0..4 {
            let c = &LOOKUP_2D[(index + i) as usize];

            let dx = xi + c.dx;
            let dy = yi + c.dy;
            let mut attn = 2.0 / 3.0 - dx * dx - dy * dy;
            if attn <= 0.0 {
                continue;
            }

            let pxm = (xsb + c.xsv) & PMASK as i32;
            let pym = (ysb + c.ysv) & PMASK as i32;
            let grad = self.permGrad2[(self.perm[pxm as usize] ^ pym as u16) as usize];
            let extrapolation = grad.0 * dx + grad.1 * dy;

            attn *= attn;
            value += attn * attn * extrapolation;
        }

        value
    }

    /// 3D Re-oriented 8-point BCC noise, classic orientation
    /// Proper substitute for what 3D SuperSimplex "should" be,
    /// in light of Forbidden Formulae.
    /// Use noise3_XYBeforeZ or noise3_XZBeforeY instead, wherever appropriate.
    pub fn noise3_classic(&self, x: f64, y: f64, z: f64) -> f64 {
        // Re-orient the cubic lattices via rotation, to produce the expected look on cardinal planar slices.
        // If texturing objects that don't tend to have cardinal plane faces, you could even remove this.
        // Orthonormal rotation. Not a skew transform.
        let r = (2.0 / 3.0) * (x + y + z);
        let xr = r - x;
        let yr = r - y;
        let zr = r - z;

        // Evaluate both lattices to form a BCC lattice.
        self.noise3_bcc(xr, yr, zr)
    }

    /// 3D Re-oriented 8-point BCC noise, with better visual isotropy in (X, Y).
    /// Recommended for 3D terrain and time-varied animations.
    /// The Z coordinate should always be the "different" coordinate in your use case.
    /// If Y is vertical in world coordinates, call noise3_XYBeforeZ(x, z, Y) or use noise3_XZBeforeY.
    /// If Z is vertical in world coordinates, call noise3_XYBeforeZ(x, y, Z).
    /// For a time varied animation, call noise3_XYBeforeZ(x, y, T).
    pub fn noise3_xy_before_z(&self, x: f64, y: f64, z: f64) -> f64 {
        // Re-orient the cubic lattices without skewing, to make X and Y triangular like 2D.
        // Orthonormal rotation. Not a skew transform.
        let xy = x + y;
        let s2 = xy * -0.211324865405187;
        let zz = z * 0.577350269189626;
        let xr = x + s2 - zz;
        let yr = y + s2 - zz;
        let zr = xy * 0.577350269189626 + zz;

        // Evaluate both lattices to form a BCC lattice.
        self.noise3_bcc(xr, yr, zr)
    }

    /// 3D Re-oriented 8-point BCC noise, with better visual isotropy in (X, Z).
    /// Recommended for 3D terrain and time-varied animations.
    /// The Y coordinate should always be the "different" coordinate in your use case.
    /// If Y is vertical in world coordinates, call noise3_XZBeforeY(x, Y, z).
    /// If Z is vertical in world coordinates, call noise3_XZBeforeY(x, Z, y) or use noise3_XYBeforeZ.
    /// For a time varied animation, call noise3_XZBeforeY(x, T, y) or use noise3_XYBeforeZ.
    pub fn noise3_xz_before_y(&self, x: f64, y: f64, z: f64) -> f64 {
        // Re-orient the cubic lattices without skewing, to make X and Z triangular like 2D.
        // Orthonormal rotation. Not a skew transform.
        let xz = x + z;
        let s2 = xz * -0.211324865405187;
        let yy = y * 0.577350269189626;
        let xr = x + s2 - yy;
        let zr = z + s2 - yy;
        let yr = xz * 0.577350269189626 + yy;

        // Evaluate both lattices to form a BCC lattice.
        self.noise3_bcc(xr, yr, zr)
    }

    /// Generate overlapping cubic lattices for 3D Re-oriented BCC noise.
    /// Lookup table implementation inspired by DigitalShadow.
    /// It was actually faster to narrow down the points in the loop itself,
    /// than to build up the index with enough info to isolate 8 points.
    fn noise3_bcc(&self, xr: f64, yr: f64, zr: f64) -> f64 {
        // Get base and offsets inside cube of first lattice.
        let xrb = fastFloor(xr);
        let yrb = fastFloor(yr);
        let zrb = fastFloor(zr);
        let xri = xr - xrb as f64;
        let yri = yr - yrb as f64;
        let zri = zr - zrb as f64;

        // Identify which octant of the cube we're in. This determines which cell
        // in the other cubic lattice we're in, and also narrows down one point on each.
        let xht = (xri + 0.5).floor() as i32;
        let yht = (yri + 0.5).floor() as i32;
        let zht = (zri + 0.5).floor() as i32;
        let index = (xht << 0) | (yht << 1) | (zht << 2);

        // Point contributions
        let mut value: f64 = 0.0;
        let mut c = &LOOKUP_3D[index as usize];

        loop {
            let dxr = xri + c.dxr;
            let dyr = yri + c.dyr;
            let dzr = zri + c.dzr;
            let mut attn = 0.75 - dxr * dxr - dyr * dyr - dzr * dzr;

            if attn < 0.0 {
                c = match &c.next_on_failure {
                    Some(c) => &c,
                    None => break,
                };
            } else {
                let pxm = (xrb + c.xrv) & PMASK as i32;
                let pym = (yrb + c.yrv) & PMASK as i32;
                let pzm = (zrb + c.zrv) & PMASK as i32;

                let index = self.perm[(self.perm[pxm as usize] as i32 ^ pym) as usize];
                let grad = self.permGrad3[(index as i32 ^ pzm) as usize];

                let extrapolation = grad.0 * dxr + grad.1 * dyr + grad.2 * dzr;

                attn *= attn;
                value += attn * attn * extrapolation;

                c = match &c.next_on_success {
                    Some(c) => c,
                    None => break,
                };
            }
        }

        value
    }
}

#[derive(Copy, Clone)]
struct LatticePoint2D {
    pub xsv: i32,
    pub ysv: i32,
    pub dx: f64,
    pub dy: f64,
}

impl LatticePoint2D {
    pub fn new(xsv: i32, ysv: i32) -> Self {
        let ssv = (xsv * ysv) as f64 * -0.211324865405187;
        Self {
            xsv,
            ysv,
            dx: -xsv as f64 - ssv,
            dy: -ysv as f64 - ssv,
        }
    }
}

#[derive(Clone)]
struct LatticePoint3D {
    pub xrv: i32,
    pub yrv: i32,
    pub zrv: i32,
    pub dxr: f64,
    pub dyr: f64,
    pub dzr: f64,
    pub next_on_failure: Option<Box<LatticePoint3D>>,
    pub next_on_success: Option<Box<LatticePoint3D>>,
}

impl LatticePoint3D {
    pub fn new(xrv: i32, yrv: i32, zrv: i32, lattice: i32) -> Self {
        let dxr = -xrv as f64 + lattice as f64 * 0.5;
        let dyr = -yrv as f64 + lattice as f64 * 0.5;
        let dzr = -zrv as f64 + lattice as f64 * 0.5;
        let xrv = xrv + lattice * 1024;
        let yrv = yrv + lattice * 1024;
        let zrv = zrv + lattice * 1024;

        Self {
            dxr,
            dyr,
            dzr,
            xrv,
            yrv,
            zrv,
            next_on_failure: None,
            next_on_success: None,
        }
    }
}

#[derive(Copy, Clone)]
struct LatticePoint4D {
    pub xsv: i32,
    pub ysv: i32,
    pub zsv: i32,
    pub wsv: i32,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub dw: f64,
}

impl LatticePoint4D {
    pub fn new(xsv: i32, ysv: i32, zsv: i32, wsv: i32) -> Self {
        let xsv = xsv;
        let ysv = ysv;
        let zsv = zsv;
        let wsv = wsv;
        let ssv: f64 = (xsv + ysv + zsv + wsv) as f64 * -0.138196601125011;
        let dx = -xsv as f64 - ssv;
        let dy = -ysv as f64 - ssv;
        let dz = -zsv as f64 - ssv;
        let dw = -wsv as f64 - ssv;

        Self {
            xsv,
            ysv,
            zsv,
            wsv,
            dx,
            dy,
            dz,
            dw,
        }
    }
}


// For now this macro helps to initialize the gradients tables once for use.
// There is a RFC to introduce logic to initialize static variables via `once_cell`.
// For details on the progress, see the discussion https://github.com/rust-lang/rfcs/pull/2788
// Until then the [lazy_static](https://docs.rs/lazy_static/1.4.0/lazy_static/) crate is used to initialize
// the lookup tables similar to the Java static initializer.
//
lazy_static! {
    static ref LOOKUP_2D: Vec<LatticePoint2D> = {
        let mut table = Vec::with_capacity(32);

        for i in 0..8 {
            let i1; let j1; let i2; let j2;

            if (i & 1) == 0 {
                if (i & 2) == 0 { i1 = -1; j1 =  0; } else { i1 = 1; j1 = 0; }
                if (i & 4) == 0 { i2 =  0; j2 = -1; } else { i2 = 0; j2 = 1; }
            } else {
                if (i & 2) != 0 { i1 = 2; j1 = 1; } else { i1 = 0; j1 = 1; }
                if (i & 4) != 0 { i2 = 1; j2 = 2; } else { i2 = 1; j2 = 0; }
            }

            table.push(LatticePoint2D::new(0, 0));
            table.push(LatticePoint2D::new(1, 1));
            table.push(LatticePoint2D::new(i1, j1));
            table.push(LatticePoint2D::new(i2, j2));
        }

        table
    };

    static ref LOOKUP_3D: Vec<LatticePoint3D> = {
        let mut table = Vec::with_capacity(8);

        for i in 0..8 {
            let i1; let j1; let k1; let i2; let j2; let k2;

            i1 = (i >> 0) & 1; j1 = (i >> 1) & 1; k1 = (i >> 2) & 1;
            i2 = i1 ^ 1; j2 = j1 ^ 1; k2 = k1 ^ 1;

            // The two points within this octant, one from each of the two cubic half-lattices.
            let mut c0 = LatticePoint3D::new(i1, j1, k1, 0);
            let mut c1 = LatticePoint3D::new(i1 + i2, j1 + j2, k1 + k2, 1);

            // (1, 0, 0) vs (0, 1, 1) away from octant.
            let mut c2 = LatticePoint3D::new(i1 ^ 1, j1, k1, 0);
            let mut c3 = LatticePoint3D::new(i1, j1 ^ 1, k1 ^ 1, 0);

            // (1, 0, 0) vs (0, 1, 1) away from octant, on second half-lattice.
            let mut c4 = LatticePoint3D::new(i1 + (i2 ^ 1), j1 + j2, k1 + k2, 1);
            let mut c5 = LatticePoint3D::new(i1 + i2, j1 + (j2 ^ 1), k1 + (k2 ^ 1), 1);

            // (0, 1, 0) vs (1, 0, 1) away from octant.
            let mut c6 = LatticePoint3D::new(i1, j1 ^ 1, k1, 0);
            let mut c7 = LatticePoint3D::new(i1 ^ 1, j1, k1 ^ 1, 0);

            // (0, 1, 0) vs (1, 0, 1) away from octant, on second half-lattice.
            let mut c8 = LatticePoint3D::new(i1 + i2, j1 + (j2 ^ 1), k1 + k2, 1);
            let mut c9 = LatticePoint3D::new(i1 + (i2 ^ 1), j1 + j2, k1 + (k2 ^ 1), 1);

            // (0, 0, 1) vs (1, 1, 0) away from octant.
            let mut cA = LatticePoint3D::new(i1, j1, k1 ^ 1, 0);
            let mut cB = LatticePoint3D::new(i1 ^ 1, j1 ^ 1, k1, 0);

            // (0, 0, 1) vs (1, 1, 0) away from octant, on second half-lattice.
            let mut cC = LatticePoint3D::new(i1 + i2, j1 + j2, k1 + (k2 ^ 1), 1);
            let mut cD = LatticePoint3D::new(i1 + (i2 ^ 1), j1 + (j2 ^ 1), k1 + k2, 1);

            // First two points are guaranteed.
            c0.next_on_failure = Some(Box::new(c1.clone()));
            c0.next_on_success = Some(Box::new(c1.clone()));
            c1.next_on_failure = Some(Box::new(c2.clone()));
            c1.next_on_success = Some(Box::new(c2.clone()));

            // If c2 is in range, then we know c3 and c4 are not.
            c2.next_on_failure = Some(Box::new(c3.clone()));
            c2.next_on_success = Some(Box::new(c5.clone()));
            c3.next_on_failure = Some(Box::new(c4.clone()));
            c3.next_on_success = Some(Box::new(c4.clone()));

            // If c4 is in range, then we know c5 is not.
            c4.next_on_failure = Some(Box::new(c5.clone()));
            c4.next_on_success = Some(Box::new(c6.clone()));
            c5.next_on_failure = Some(Box::new(c6.clone()));
            c5.next_on_success = Some(Box::new(c6.clone()));

            // If c6 is in range, then we know c7 and c8 are not.
            c6.next_on_failure = Some(Box::new(c7.clone()));
            c6.next_on_success = Some(Box::new(c9.clone()));
            c7.next_on_failure = Some(Box::new(c8.clone()));
            c7.next_on_success = Some(Box::new(c8.clone()));

            // If c8 is in range, then we know c9 is not.
            c8.next_on_failure = Some(Box::new(c9.clone()));
            c8.next_on_success = Some(Box::new(cA.clone()));
            c9.next_on_failure = Some(Box::new(cA.clone()));
            c9.next_on_success = Some(Box::new(cA.clone()));

            // If cA is in range, then we know cB and cC are not.
            cA.next_on_failure = Some(Box::new(cB.clone()));
            cA.next_on_success = Some(Box::new(cD.clone()));
            cB.next_on_failure = Some(Box::new(cC.clone()));
            cB.next_on_success = Some(Box::new(cC.clone()));

            // If cC is in range, then we know cD is not.
            cC.next_on_failure = Some(Box::new(cD.clone()));
            cC.next_on_success = None;
            cD.next_on_failure = None;
            cD.next_on_success = None;

            table.push(c0);
        }

        table
    };

    /// Gradients lookup table for 2 dimensions
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
            gradients.push(grad2[i % grad2.len()]);
        }
        gradients
    };

    /// Gradients lookup table for 3 dimensions
    static ref GRADIENTS_3D: Vec<(f64, f64, f64)> = {
        let mut grad3 = vec![
            (-2.22474487139,      -2.22474487139,      -1.0),
            (-2.22474487139,      -2.22474487139,       1.0),
            (-3.0862664687972017, -1.1721513422464978,  0.0),
            (-1.1721513422464978, -3.0862664687972017,  0.0),
            (-2.22474487139,      -1.0,                -2.22474487139),
            (-2.22474487139,       1.0,                -2.22474487139),
            (-1.1721513422464978,  0.0,                -3.0862664687972017),
            (-3.0862664687972017,  0.0,                -1.1721513422464978),
            (-2.22474487139,      -1.0,                 2.22474487139),
            (-2.22474487139,       1.0,                 2.22474487139),
            (-3.0862664687972017,  0.0,                 1.1721513422464978),
            (-1.1721513422464978,  0.0,                 3.0862664687972017),
            (-2.22474487139,       2.22474487139,      -1.0),
            (-2.22474487139,       2.22474487139,       1.0),
            (-1.1721513422464978,  3.0862664687972017,  0.0),
            (-3.0862664687972017,  1.1721513422464978,  0.0),
            (-1.0,                -2.22474487139,      -2.22474487139),
            ( 1.0,                -2.22474487139,      -2.22474487139),
            ( 0.0,                -3.0862664687972017, -1.1721513422464978),
            ( 0.0,                -1.1721513422464978, -3.0862664687972017),
            (-1.0,                -2.22474487139,       2.22474487139),
            ( 1.0,                -2.22474487139,       2.22474487139),
            ( 0.0,                -1.1721513422464978,  3.0862664687972017),
            ( 0.0,                -3.0862664687972017,  1.1721513422464978),
            (-1.0,                 2.22474487139,      -2.22474487139),
            ( 1.0,                 2.22474487139,      -2.22474487139),
            ( 0.0,                 1.1721513422464978, -3.0862664687972017),
            ( 0.0,                 3.0862664687972017, -1.1721513422464978),
            (-1.0,                 2.22474487139,       2.22474487139),
            ( 1.0,                 2.22474487139,       2.22474487139),
            ( 0.0,                 3.0862664687972017,  1.1721513422464978),
            ( 0.0,                 1.1721513422464978,  3.0862664687972017),
            ( 2.22474487139,      -2.22474487139,      -1.0),
            ( 2.22474487139,      -2.22474487139,       1.0),
            ( 1.1721513422464978, -3.0862664687972017,  0.0),
            ( 3.0862664687972017, -1.1721513422464978,  0.0),
            ( 2.22474487139,      -1.0,                -2.22474487139),
            ( 2.22474487139,       1.0,                -2.22474487139),
            ( 3.0862664687972017,  0.0,                -1.1721513422464978),
            ( 1.1721513422464978,  0.0,                -3.0862664687972017),
            ( 2.22474487139,      -1.0,                 2.22474487139),
            ( 2.22474487139,       1.0,                 2.22474487139),
            ( 1.1721513422464978,  0.0,                 3.0862664687972017),
            ( 3.0862664687972017,  0.0,                 1.1721513422464978),
            ( 2.22474487139,       2.22474487139,      -1.0),
            ( 2.22474487139,       2.22474487139,       1.0),
            ( 3.0862664687972017,  1.1721513422464978,  0.0),
            ( 1.1721513422464978,  3.0862664687972017,  0.0)
        ];

        for i in 0..grad3.len() {
            grad3[i].0 /= N3;
            grad3[i].1 /= N3;
            grad3[i].2 /= N3;
        }

        let mut gradients = Vec::with_capacity(PSIZE);
        for i in 0..PSIZE {
            gradients.push(grad3[i % grad3.len()]);
        }

        gradients
    };

    static ref GRADIENTS_4D: Vec<(f64, f64, f64, f64)> = {
        let mut grad4 = vec![
            (-0.753341017856078,    -0.37968289875261624,  -0.37968289875261624,  -0.37968289875261624),
            (-0.7821684431180708,   -0.4321472685365301,   -0.4321472685365301,    0.12128480194602098),
            (-0.7821684431180708,   -0.4321472685365301,    0.12128480194602098,  -0.4321472685365301),
            (-0.7821684431180708,    0.12128480194602098,  -0.4321472685365301,   -0.4321472685365301),
            (-0.8586508742123365,   -0.508629699630796,     0.044802370851755174,  0.044802370851755174),
            (-0.8586508742123365,    0.044802370851755174, -0.508629699630796,     0.044802370851755174),
            (-0.8586508742123365,    0.044802370851755174,  0.044802370851755174, -0.508629699630796),
            (-0.9982828964265062,   -0.03381941603233842,  -0.03381941603233842,  -0.03381941603233842),
            (-0.37968289875261624,  -0.753341017856078,    -0.37968289875261624,  -0.37968289875261624),
            (-0.4321472685365301,   -0.7821684431180708,   -0.4321472685365301,    0.12128480194602098),
            (-0.4321472685365301,   -0.7821684431180708,    0.12128480194602098,  -0.4321472685365301),
            ( 0.12128480194602098,  -0.7821684431180708,   -0.4321472685365301,   -0.4321472685365301),
            (-0.508629699630796,    -0.8586508742123365,    0.044802370851755174,  0.044802370851755174),
            ( 0.044802370851755174, -0.8586508742123365,   -0.508629699630796,     0.044802370851755174),
            ( 0.044802370851755174, -0.8586508742123365,    0.044802370851755174, -0.508629699630796),
            (-0.03381941603233842,  -0.9982828964265062,   -0.03381941603233842,  -0.03381941603233842),
            (-0.37968289875261624,  -0.37968289875261624,  -0.753341017856078,    -0.37968289875261624),
            (-0.4321472685365301,   -0.4321472685365301,   -0.7821684431180708,    0.12128480194602098),
            (-0.4321472685365301,    0.12128480194602098,  -0.7821684431180708,   -0.4321472685365301),
            ( 0.12128480194602098,  -0.4321472685365301,   -0.7821684431180708,   -0.4321472685365301),
            (-0.508629699630796,     0.044802370851755174, -0.8586508742123365,    0.044802370851755174),
            ( 0.044802370851755174, -0.508629699630796,    -0.8586508742123365,    0.044802370851755174),
            ( 0.044802370851755174,  0.044802370851755174, -0.8586508742123365,   -0.508629699630796),
            (-0.03381941603233842,  -0.03381941603233842,  -0.9982828964265062,   -0.03381941603233842),
            (-0.37968289875261624,  -0.37968289875261624,  -0.37968289875261624,  -0.753341017856078),
            (-0.4321472685365301,   -0.4321472685365301,    0.12128480194602098,  -0.7821684431180708),
            (-0.4321472685365301,    0.12128480194602098,  -0.4321472685365301,   -0.7821684431180708),
            ( 0.12128480194602098,  -0.4321472685365301,   -0.4321472685365301,   -0.7821684431180708),
            (-0.508629699630796,     0.044802370851755174,  0.044802370851755174, -0.8586508742123365),
            ( 0.044802370851755174, -0.508629699630796,     0.044802370851755174, -0.8586508742123365),
            ( 0.044802370851755174,  0.044802370851755174, -0.508629699630796,    -0.8586508742123365),
            (-0.03381941603233842,  -0.03381941603233842,  -0.03381941603233842,  -0.9982828964265062),
            (-0.6740059517812944,   -0.3239847771997537,   -0.3239847771997537,    0.5794684678643381),
            (-0.7504883828755602,   -0.4004672082940195,    0.15296486218853164,   0.5029860367700724),
            (-0.7504883828755602,    0.15296486218853164,  -0.4004672082940195,    0.5029860367700724),
            (-0.8828161875373585,    0.08164729285680945,   0.08164729285680945,   0.4553054119602712),
            (-0.4553054119602712,   -0.08164729285680945,  -0.08164729285680945,   0.8828161875373585),
            (-0.5029860367700724,   -0.15296486218853164,   0.4004672082940195,    0.7504883828755602),
            (-0.5029860367700724,    0.4004672082940195,   -0.15296486218853164,   0.7504883828755602),
            (-0.5794684678643381,    0.3239847771997537,    0.3239847771997537,    0.6740059517812944),
            (-0.3239847771997537,   -0.6740059517812944,   -0.3239847771997537,    0.5794684678643381),
            (-0.4004672082940195,   -0.7504883828755602,    0.15296486218853164,   0.5029860367700724),
            ( 0.15296486218853164,  -0.7504883828755602,   -0.4004672082940195,    0.5029860367700724),
            ( 0.08164729285680945,  -0.8828161875373585,    0.08164729285680945,   0.4553054119602712),
            (-0.08164729285680945,  -0.4553054119602712,   -0.08164729285680945,   0.8828161875373585),
            (-0.15296486218853164,  -0.5029860367700724,    0.4004672082940195,    0.7504883828755602),
            ( 0.4004672082940195,   -0.5029860367700724,   -0.15296486218853164,   0.7504883828755602),
            ( 0.3239847771997537,   -0.5794684678643381,    0.3239847771997537,    0.6740059517812944),
            (-0.3239847771997537,   -0.3239847771997537,   -0.6740059517812944,    0.5794684678643381),
            (-0.4004672082940195,    0.15296486218853164,  -0.7504883828755602,    0.5029860367700724),
            ( 0.15296486218853164,  -0.4004672082940195,   -0.7504883828755602,    0.5029860367700724),
            ( 0.08164729285680945,   0.08164729285680945,  -0.8828161875373585,    0.4553054119602712),
            (-0.08164729285680945,  -0.08164729285680945,  -0.4553054119602712,    0.8828161875373585),
            (-0.15296486218853164,   0.4004672082940195,   -0.5029860367700724,    0.7504883828755602),
            ( 0.4004672082940195,   -0.15296486218853164,  -0.5029860367700724,    0.7504883828755602),
            ( 0.3239847771997537,    0.3239847771997537,   -0.5794684678643381,    0.6740059517812944),
            (-0.6740059517812944,   -0.3239847771997537,    0.5794684678643381,   -0.3239847771997537),
            (-0.7504883828755602,   -0.4004672082940195,    0.5029860367700724,    0.15296486218853164),
            (-0.7504883828755602,    0.15296486218853164,   0.5029860367700724,   -0.4004672082940195),
            (-0.8828161875373585,    0.08164729285680945,   0.4553054119602712,    0.08164729285680945),
            (-0.4553054119602712,   -0.08164729285680945,   0.8828161875373585,   -0.08164729285680945),
            (-0.5029860367700724,   -0.15296486218853164,   0.7504883828755602,    0.4004672082940195),
            (-0.5029860367700724,    0.4004672082940195,    0.7504883828755602,   -0.15296486218853164),
            (-0.5794684678643381,    0.3239847771997537,    0.6740059517812944,    0.3239847771997537),
            (-0.3239847771997537,   -0.6740059517812944,    0.5794684678643381,   -0.3239847771997537),
            (-0.4004672082940195,   -0.7504883828755602,    0.5029860367700724,    0.15296486218853164),
            ( 0.15296486218853164,  -0.7504883828755602,    0.5029860367700724,   -0.4004672082940195),
            ( 0.08164729285680945,  -0.8828161875373585,    0.4553054119602712,    0.08164729285680945),
            (-0.08164729285680945,  -0.4553054119602712,    0.8828161875373585,   -0.08164729285680945),
            (-0.15296486218853164,  -0.5029860367700724,    0.7504883828755602,    0.4004672082940195),
            ( 0.4004672082940195,   -0.5029860367700724,    0.7504883828755602,   -0.15296486218853164),
            ( 0.3239847771997537,   -0.5794684678643381,    0.6740059517812944,    0.3239847771997537),
            (-0.3239847771997537,   -0.3239847771997537,    0.5794684678643381,   -0.6740059517812944),
            (-0.4004672082940195,    0.15296486218853164,   0.5029860367700724,   -0.7504883828755602),
            ( 0.15296486218853164,  -0.4004672082940195,    0.5029860367700724,   -0.7504883828755602),
            ( 0.08164729285680945,   0.08164729285680945,   0.4553054119602712,   -0.8828161875373585),
            (-0.08164729285680945,  -0.08164729285680945,   0.8828161875373585,   -0.4553054119602712),
            (-0.15296486218853164,   0.4004672082940195,    0.7504883828755602,   -0.5029860367700724),
            ( 0.4004672082940195,   -0.15296486218853164,   0.7504883828755602,   -0.5029860367700724),
            ( 0.3239847771997537,    0.3239847771997537,    0.6740059517812944,   -0.5794684678643381),
            (-0.6740059517812944,    0.5794684678643381,   -0.3239847771997537,   -0.3239847771997537),
            (-0.7504883828755602,    0.5029860367700724,   -0.4004672082940195,    0.15296486218853164),
            (-0.7504883828755602,    0.5029860367700724,    0.15296486218853164,  -0.4004672082940195),
            (-0.8828161875373585,    0.4553054119602712,    0.08164729285680945,   0.08164729285680945),
            (-0.4553054119602712,    0.8828161875373585,   -0.08164729285680945,  -0.08164729285680945),
            (-0.5029860367700724,    0.7504883828755602,   -0.15296486218853164,   0.4004672082940195),
            (-0.5029860367700724,    0.7504883828755602,    0.4004672082940195,   -0.15296486218853164),
            (-0.5794684678643381,    0.6740059517812944,    0.3239847771997537,    0.3239847771997537),
            (-0.3239847771997537,    0.5794684678643381,   -0.6740059517812944,   -0.3239847771997537),
            (-0.4004672082940195,    0.5029860367700724,   -0.7504883828755602,    0.15296486218853164),
            ( 0.15296486218853164,   0.5029860367700724,   -0.7504883828755602,   -0.4004672082940195),
            ( 0.08164729285680945,   0.4553054119602712,   -0.8828161875373585,    0.08164729285680945),
            (-0.08164729285680945,   0.8828161875373585,   -0.4553054119602712,   -0.08164729285680945),
            (-0.15296486218853164,   0.7504883828755602,   -0.5029860367700724,    0.4004672082940195),
            ( 0.4004672082940195,    0.7504883828755602,   -0.5029860367700724,   -0.15296486218853164),
            ( 0.3239847771997537,    0.6740059517812944,   -0.5794684678643381,    0.3239847771997537),
            (-0.3239847771997537,    0.5794684678643381,   -0.3239847771997537,   -0.6740059517812944),
            (-0.4004672082940195,    0.5029860367700724,    0.15296486218853164,  -0.7504883828755602),
            ( 0.15296486218853164,   0.5029860367700724,   -0.4004672082940195,   -0.7504883828755602),
            ( 0.08164729285680945,   0.4553054119602712,    0.08164729285680945,  -0.8828161875373585),
            (-0.08164729285680945,   0.8828161875373585,   -0.08164729285680945,  -0.4553054119602712),
            (-0.15296486218853164,   0.7504883828755602,    0.4004672082940195,   -0.5029860367700724),
            ( 0.4004672082940195,    0.7504883828755602,   -0.15296486218853164,  -0.5029860367700724),
            ( 0.3239847771997537,    0.6740059517812944,    0.3239847771997537,   -0.5794684678643381),
            ( 0.5794684678643381,   -0.6740059517812944,   -0.3239847771997537,   -0.3239847771997537),
            ( 0.5029860367700724,   -0.7504883828755602,   -0.4004672082940195,    0.15296486218853164),
            ( 0.5029860367700724,   -0.7504883828755602,    0.15296486218853164,  -0.4004672082940195),
            ( 0.4553054119602712,   -0.8828161875373585,    0.08164729285680945,   0.08164729285680945),
            ( 0.8828161875373585,   -0.4553054119602712,   -0.08164729285680945,  -0.08164729285680945),
            ( 0.7504883828755602,   -0.5029860367700724,   -0.15296486218853164,   0.4004672082940195),
            ( 0.7504883828755602,   -0.5029860367700724,    0.4004672082940195,   -0.15296486218853164),
            ( 0.6740059517812944,   -0.5794684678643381,    0.3239847771997537,    0.3239847771997537),
            ( 0.5794684678643381,   -0.3239847771997537,   -0.6740059517812944,   -0.3239847771997537),
            ( 0.5029860367700724,   -0.4004672082940195,   -0.7504883828755602,    0.15296486218853164),
            ( 0.5029860367700724,    0.15296486218853164,  -0.7504883828755602,   -0.4004672082940195),
            ( 0.4553054119602712,    0.08164729285680945,  -0.8828161875373585,    0.08164729285680945),
            ( 0.8828161875373585,   -0.08164729285680945,  -0.4553054119602712,   -0.08164729285680945),
            ( 0.7504883828755602,   -0.15296486218853164,  -0.5029860367700724,    0.4004672082940195),
            ( 0.7504883828755602,    0.4004672082940195,   -0.5029860367700724,   -0.15296486218853164),
            ( 0.6740059517812944,    0.3239847771997537,   -0.5794684678643381,    0.3239847771997537),
            ( 0.5794684678643381,   -0.3239847771997537,   -0.3239847771997537,   -0.6740059517812944),
            ( 0.5029860367700724,   -0.4004672082940195,    0.15296486218853164,  -0.7504883828755602),
            ( 0.5029860367700724,    0.15296486218853164,  -0.4004672082940195,   -0.7504883828755602),
            ( 0.4553054119602712,    0.08164729285680945,   0.08164729285680945,  -0.8828161875373585),
            ( 0.8828161875373585,   -0.08164729285680945,  -0.08164729285680945,  -0.4553054119602712),
            ( 0.7504883828755602,   -0.15296486218853164,   0.4004672082940195,   -0.5029860367700724),
            ( 0.7504883828755602,    0.4004672082940195,   -0.15296486218853164,  -0.5029860367700724),
            ( 0.6740059517812944,    0.3239847771997537,    0.3239847771997537,   -0.5794684678643381),
            ( 0.03381941603233842,   0.03381941603233842,   0.03381941603233842,   0.9982828964265062),
            (-0.044802370851755174, -0.044802370851755174,  0.508629699630796,     0.8586508742123365),
            (-0.044802370851755174,  0.508629699630796,    -0.044802370851755174,  0.8586508742123365),
            (-0.12128480194602098,   0.4321472685365301,    0.4321472685365301,    0.7821684431180708),
            ( 0.508629699630796,    -0.044802370851755174, -0.044802370851755174,  0.8586508742123365),
            ( 0.4321472685365301,   -0.12128480194602098,   0.4321472685365301,    0.7821684431180708),
            ( 0.4321472685365301,    0.4321472685365301,   -0.12128480194602098,   0.7821684431180708),
            ( 0.37968289875261624,   0.37968289875261624,   0.37968289875261624,   0.753341017856078),
            ( 0.03381941603233842,   0.03381941603233842,   0.9982828964265062,    0.03381941603233842),
            (-0.044802370851755174,  0.044802370851755174,  0.8586508742123365,    0.508629699630796),
            (-0.044802370851755174,  0.508629699630796,     0.8586508742123365,   -0.044802370851755174),
            (-0.12128480194602098,   0.4321472685365301,    0.7821684431180708,    0.4321472685365301),
            ( 0.508629699630796,    -0.044802370851755174,  0.8586508742123365,   -0.044802370851755174),
            ( 0.4321472685365301,   -0.12128480194602098,   0.7821684431180708,    0.4321472685365301),
            ( 0.4321472685365301,    0.4321472685365301,    0.7821684431180708,   -0.12128480194602098),
            ( 0.37968289875261624,   0.37968289875261624,   0.753341017856078,     0.37968289875261624),
            ( 0.03381941603233842,   0.9982828964265062,    0.03381941603233842,   0.03381941603233842),
            (-0.044802370851755174,  0.8586508742123365,   -0.044802370851755174,  0.508629699630796),
            (-0.044802370851755174,  0.8586508742123365,    0.508629699630796,    -0.044802370851755174),
            (-0.12128480194602098,   0.7821684431180708,    0.4321472685365301,    0.4321472685365301),
            ( 0.508629699630796,     0.8586508742123365,   -0.044802370851755174, -0.044802370851755174),
            ( 0.4321472685365301,    0.7821684431180708,   -0.12128480194602098,   0.4321472685365301),
            ( 0.4321472685365301,    0.7821684431180708,    0.4321472685365301,   -0.12128480194602098),
            ( 0.37968289875261624,   0.753341017856078,     0.37968289875261624,   0.37968289875261624),
            ( 0.9982828964265062,    0.03381941603233842,   0.03381941603233842,   0.03381941603233842),
            ( 0.8586508742123365,   -0.044802370851755174, -0.044802370851755174,  0.508629699630796),
            ( 0.8586508742123365,   -0.044802370851755174,  0.508629699630796,    -0.044802370851755174),
            ( 0.7821684431180708,   -0.12128480194602098,   0.4321472685365301,    0.4321472685365301),
            ( 0.8586508742123365,    0.508629699630796,    -0.044802370851755174, -0.044802370851755174),
            ( 0.7821684431180708,    0.4321472685365301,   -0.12128480194602098,   0.4321472685365301),
            ( 0.7821684431180708,    0.4321472685365301,    0.4321472685365301,   -0.12128480194602098),
            ( 0.753341017856078,     0.37968289875261624,   0.37968289875261624,   0.37968289875261624)
        ];

        for i in 0..grad4.len() {
            grad4[i].0 /= N4;
            grad4[i].1 /= N4;
            grad4[i].2 /= N4;
            grad4[i].3 /= N4;
        }

        let mut gradients = Vec::with_capacity(PSIZE);
        for i in 0..PSIZE {
            gradients.push(grad4[i % grad4.len()]);
        }

        gradients
    };
}
