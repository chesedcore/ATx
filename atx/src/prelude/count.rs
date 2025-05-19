//count.rs 
use std::sync::atomic::AtomicUsize;

///thread-safe state tracking.
///helps in computing costs and track usage across threads
#[derive(Debug)]
pub struct Count {
    pub input: AtomicUsize,    //number of tokens thrown in
    pub output: AtomicUsize,   //number of tokens processed
}

