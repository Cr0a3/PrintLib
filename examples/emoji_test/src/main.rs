use PrintLib::emoji::EmojiMaker;

fn main() {
    let mut maker = EmojiMaker::new();
    maker.save(":thumbs_up:", "👍");
    
    println!("that is {}", maker.from(":thumbs_up:").unwrap());
}
