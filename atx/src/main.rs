mod prelude {
    pub mod loader;
}
use prelude::loader;
use log::{info, error};

fn main() {
    simple_logger::init().unwrap();
    info!("Started logger!");
    let load = loader::Loader::new().unwrap();
    let bytes = load.load_raw("bsxx.dat");
    match bytes {
        Err(e) => error!("{}",e),
        Ok(_) => {
            info!("File read successfully!");
        }
    }
}
