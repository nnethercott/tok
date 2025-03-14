use toktokenizer::{
    preproc::{DefaultNormalizer, Normalize},
    Tokenizer, BPETokenizer, Pretrained, config::TokenizerConfig
};

use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::{Fake, Faker};



fn main() {
    let config = TokenizerConfig::new(1000, None);
    let mut tok = BPETokenizer::new(config);

    let corpus: String = Paragraph(3000..3001).fake();

    tok.train(&corpus);
    println!("{}", tok.len());

    // save 
    tok.save_pretrained("tokenizer.json").expect("failed to save");
}
