use std::fs;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let mut word_count: HashMap<String, usize> = HashMap::new();
    
    for word in contents.split_whitespace() {
        let sanitized = sanitize(word);
        if sanitized.is_empty() { continue; }

        let count = word_count.entry(sanitized).or_insert(0);
        *count += 1;
    }

    for (word, count) in &word_count {
        println!("{word}: {count}");
    }

    Ok(())
}

fn sanitize(word: &str) -> String {
    word.trim_matches(&['(', ')', ',', '.', ';', ':', '/', '\n'][..])
        .to_lowercase()
}