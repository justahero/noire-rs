#![allow(dead_code)]
#![allow(unused_variables)]

extern crate notify;

use notify::*;
use std::sync::mpsc::channel;
use std::time::Duration;

fn watch_file(file_path: &str) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(2)));

    try!(watcher.watch(file_path, RecursiveMode::NonRecursive));

    loop {
        match rx.recv() {
            Ok(event) => println!("Event: {:?}", event),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

fn main() {
    if let Err(e) = watch_file("./examples/shaders/vertex.glsl") {
        println!("ERROR: {:?}", e);
    }
}
