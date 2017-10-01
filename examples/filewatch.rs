#![allow(dead_code)]
#![allow(unused_variables)]

extern crate notify;

use notify::*;
use std::sync::mpsc::channel;
use std::time::Duration;

fn watch_files(files: &Vec<String>) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_millis(125)));

    for file in files {
        try!(watcher.watch(&file, RecursiveMode::NonRecursive));
    }

    loop {
        match rx.recv() {
            Ok(event) => println!("Event: {:?}", event),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

fn main() {
    let files: Vec<String> = vec![
        "./examples/shaders/vertex.glsl".to_string(),
        "./examples/shaders/fragment.glsl".to_string(),
    ];
    if let Err(e) = watch_files(&files) {
        println!("ERROR: {:?}", e);
    }
}
