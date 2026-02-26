fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut frequencies: Vec<(&str, usize)> = Vec::new();

    for word in words {
        let mut found = false;
        for item in &mut frequencies {
            if item.0 == word {
                item.1 += 1;
                found = true;
                break;
            }
        }

        if !found {
            frequencies.push((word, 1));
        }
    }

    let mut max_word = String::new();
    let mut max_count = 0;

    for (word, count) in frequencies {
        if count > max_count {
            max_count = count;
            max_word = word.to_string();
        }
    }

    (max_word, max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}