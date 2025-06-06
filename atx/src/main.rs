use atx::prelude::loader;
use log::{info, error};
use atx::prelude::engine::Engine;
use atx::native::bishop::unscrambler::BSXDecoder;

fn main() {
    simple_logger::init().unwrap();
    info!("Started logger!");
    let load = loader::Loader::new().unwrap();
    let res = load.load_raw("bsxx.dat");
    let bytes = match res {
        Err(e) => {
            error!("{}",e);
            return;
        },
        Ok(b) => {
            info!("File read successfully!");
            b
        }
    };

    let decoder = BSXDecoder::new(bytes).unwrap();
}
