use std::{io, path, env};

/// Returns the application directory.
///
/// When an example app is executed via `cargo run` the `CARGO_MANIFEST_DIR` is set
/// the application directory is returned to the location of the Cargo manifest,
/// otherwise the path of the executable is returned.
///
/// The main goal is to provide a way to find files / assets relative to the executable.
pub fn app_dir() -> Result<path::PathBuf, io::Error> {
    if let Some(dir) = env::var_os("CARGO_MANIFEST_DIR") {
        return Ok(dir.into());
    }

    env::current_exe()?
        .parent()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Failed to get parent dir"))
        .map(|path| path.to_owned())
}
