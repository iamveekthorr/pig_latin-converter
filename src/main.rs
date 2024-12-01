fn main() {
    let words: Vec<&str> = vec![
        "hello",
        "world",
        "programming",
        "is",
        "fun",
        "apple",
        "first",
    ];
    let mut pig_latin_words: Vec<String> = Vec::new();

    for word in words {
        pig_latin_words.push(pig_latin_converter(&String::from(word)));
    }

    println!("{:?}", pig_latin_words);
}

fn pig_latin_converter(word: &String) -> String {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut result = String::new();

    match vowels.contains(&word.chars().next().unwrap().to_ascii_lowercase()) {
        true => {
            result.push_str(&word);
            result.push('-');
            result.push_str("hay");
        }

        _ => {
            let mut i = 0;
            while !vowels.contains(&word.chars().nth(i).unwrap().to_ascii_lowercase()) {
                i += 1;
            }

            result.push_str(&word[i..]);
            result.push('-');
            result.push_str(&word[0..i]);
            result.push_str("ay");
        }
    }

    result
}
