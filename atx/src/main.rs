mod prelude {
    pub mod loader;
}
use prelude::loader;
use log::{info,error};
use project_root::get_project_root;

fn main() {
    simple_logger::init().unwrap();
    info!("Started logger!");
    println!("{}",get_project_root().unwrap().display());
    let res = loader::load_binary("raw/bsxx.dat")
        .map_err(|e| error!("Error: {}", e));
}
