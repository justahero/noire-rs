#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Color::new(r: r, g: g, b: b, a: 1.0)
    }
}
