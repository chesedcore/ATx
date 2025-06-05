//engines.rs 
//an enumeration over the currently natively supported engines.

///methods engines MUST expose in order to function.
///if you want to mod the engine in order to impl a new engine, you must make the 
///engine processor struct impl this trait.
pub trait Engine {
    fn preprocess(string: &str) -> String;
}

//impl engines in /native, NOT here.
