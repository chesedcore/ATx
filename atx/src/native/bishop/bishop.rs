//bishop.rs 

use crate::native::bishop::unscrambler::BSXDecoder;
use crate::prelude::engine::Engine;
use crate::prelude::loader::Loader;

pub struct BishopEngine {
    unscrambler: BSXDecoder,
}

impl BishopEngine {
    pub fn new() -> std::io::Result<Self> {
        let loader = Loader::new()?;
        let bytes = loader.load_raw("bsxx.dat")?;
        let unscrambler = BSXDecoder::new(bytes)?;
        
        let engine = BishopEngine{unscrambler};
        Ok(engine)
    }
}

impl Engine for BishopEngine {
    fn unscramble(&self) {
        self.unscrambler.unscramble_text();
    }
}
