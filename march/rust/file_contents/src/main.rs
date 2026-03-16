use std::fs;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let mut word_count: HashMap<String, usize> = HashMap::new();
    
    for content in contents.split_whitespace() {
        let word = Word {
            word_ref: content,
        };

        let sanitized = word.sanitize();

        if sanitized.is_empty() { continue; }

        let count = word_count.entry(sanitized).or_insert(0);
        *count += 1;
    }

    for (word, count) in &word_count {
        println!("{word}: {count}");
    }

    Ok(())
}

struct Word<'a> {
    word_ref: &'a str,
}

impl<'a> Word<'a> {
    fn sanitize(&self) -> String {
        self.word_ref.trim_matches(&['(', ')', ',', '.', ';', ':', '/', '\n'][..])
        .to_lowercase()
    }
}