use log::{info, error};
use atx::prelude::engine::Engine;
use atx::native::bishop::bishop::BishopEngine;

fn main() {
    simple_logger::init().unwrap();
    info!("Started logger!");

    let engine = BishopEngine::new().unwrap();
}
