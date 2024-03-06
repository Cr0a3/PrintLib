use std::{collections::HashMap, ops::Deref};

pub struct EmojiMaker {
    emojis: HashMap<String, String>
}

impl EmojiMaker {
    pub fn new() -> Self {
        Self {
            emojis: HashMap::new()
        }
    }

    pub fn add(&mut self, name: &str, emoji: &str) {
        self.emojis.insert(name.into(), emoji.into());
    }

    pub fn get(&self, n: &str) -> Option<String> {

        if self.emojis.contains_key(n) {
            let emoji = self.emojis.get(n);
            Some(emoji.unwrap().deref().to_string())

        } else {
            println!("couldn't find emoji: '{n}'");
            Option::from(None)
        }
    }
}