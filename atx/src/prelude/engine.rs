//engine.rs
pub trait Engine {
    ///a method that takes in some bytes, unscrambles them into human readable format, then writes
    ///them into the /from folder. 
    fn unscramble(&self);
}
