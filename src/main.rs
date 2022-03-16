#[derive(Debug)]
enum WordleChar {
    Certain(char, u8),
    Somewhere(char, Vec<u8>),
}

const MAX_POSSIBILITIES: usize = 512;

fn main() {
    let chars = vec![
        WordleChar::Certain('a', 2),
        WordleChar::Certain('t', 3),
        WordleChar::Somewhere('c', vec![3]),
        WordleChar::Somewhere('r', vec![2, 3, 4]),
        WordleChar::Somewhere('e', vec![5]),
    ];

    let possibilities = generate_possibilities(chars);
    if possibilities.len() > MAX_POSSIBILITIES {
        println!(
            "Too many possibilities ({}), not printing",
            possibilities.len()
        );
    } else {
        possibilities.into_iter().for_each(|word| {
            println!("{}", word);
        })
    }
}

fn generate_possibilities(chars: Vec<WordleChar>) -> Vec<String> {
    let mut words: Vec<String> = vec![String::from("")];

    'outer: for word_pos in 1..=5 {
        // Check if we already know this letter
        for char in chars.iter() {
            if let WordleChar::Certain(letter, pos) = char {
                if *pos == word_pos {
                    words = words
                        .iter()
                        .map(|word| {
                            let mut word = word.clone();
                            word.push(letter.clone());
                            word
                        })
                        .collect();

                    continue 'outer;
                }
            }
        }

        let mut new_words = Vec::new();
        for char in chars.iter() {
            if let WordleChar::Somewhere(letter, invalid_positions) = char {
                if invalid_positions.contains(&word_pos) == false {
                    words
                        .iter()
                        .filter(|word| word.contains(&letter.to_string()) == false)
                        .for_each(|word| {
                            let mut new_word = word.clone();
                            new_word.push(letter.clone());
                            new_words.push(new_word);
                        })
                }
            }
        }

        words = new_words;
    }

    return words;
}
