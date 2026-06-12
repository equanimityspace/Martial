// create shared c2pa readers for different commands

use c2pa::{Context, Reader, Result};
use std::sync::Arc;

pub struct C2paState {
    pub image_reader: Reader,
    pub video_reader: Reader,
    pub audio_reader: Reader,
    pub other_reader: Reader,
}

impl C2paState {
    pub fn new() -> Result<Arc<Self>> {
        let context = Context::new()
            .with_settings(include_str!("../config.json"))?
            .into_shared();

        let state = Self {
            image_reader: Reader::from_shared_context(&context),
            video_reader: Reader::from_shared_context(&context),
            audio_reader: Reader::from_shared_context(&context),
            other_reader: Reader::from_shared_context(&context),
        };

        Ok(Arc::new(state))
    }
}
