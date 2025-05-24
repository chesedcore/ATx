//engines.rs 
//an enumeration over the currently natively supported engines.

pub trait Engine {
    fn preprocess(string: &str) -> String;
}
