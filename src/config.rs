use crate::preproc::{DefaultNormalizer, Normalize, Normalizer};
use crate::tokenizer::VocabMap;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{read_to_string, File};
use std::io::{Read, Write};


// NOTE: impl `Pretrained` for tokenizer?

pub trait Pretrained: Sized {
    fn save_pretrained(&self, path: &str) -> Result<(), std::io::Error>;
    fn from_pretrained(path: &str) -> Result<Self, std::io::Error>;
}

impl<T> Pretrained for T
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    fn save_pretrained(&self, path: &str) -> Result<(), std::io::Error> {
        let file = File::create(path)?;
        serde_json::to_writer(file, &self).expect("failed to save pretrained !");
        Ok(())
    }

    fn from_pretrained(path: &str) -> Result<Self, std::io::Error> {
        let s = read_to_string(path)?;
        let config = serde_json::from_str::<Self>(&s).expect("failed to load pretrained");
        Ok(config)
    }
}


#[derive(Serialize, Deserialize)]
pub struct TokenizerConfig {
    pub vocab_size: usize,
    pub special_tokens_map: Option<VocabMap>,
    #[serde(default)]
    pub preproc: Normalizer,
}

//TODO: more methods to add special tokens ?

impl TokenizerConfig {
    pub fn new(vocab_size: usize, preproc: Option<Normalizer>) -> Self {
        assert!(vocab_size > 0, "can't train on vocab_size <= 0!");

        let preproc = match preproc{
            Some(p) => p,
            None => Normalizer::default(),
        };

        Self {
            vocab_size,
            preproc,
            special_tokens_map: None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    //FIXME: add tests here to check ser-de

    #[test]
    fn test_serialize() {
        //serialize to tmp file (maybe need crate)
        //passes
    }
}
