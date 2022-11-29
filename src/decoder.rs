extern crate reqwest;

use tokio::runtime::Runtime; // 0.3.5

#[derive(Debug)]
pub struct Decode {
    deciphered: String,
    shift: u8,
    pub word_matches: u8,
}

pub fn decode_message(message: String) -> Vec<Decode> {
    let mut result_decodes: Vec<Decode> = Vec::new();

    for shift in 1..32 {
        let result: String = message
            .chars()
            .map(|c| std::char::from_u32(
                if c == ' ' {
                    ' ' as u32
                } else if c as u32 + shift > 'z' as u32 {
                    c as u32 + shift - 'z' as u32 + 'a' as u32 - 1
                } else {
                    c as u32 + shift
                }
            ).unwrap()
            )
            .collect();
        let mut count_exist_word = 0;
        result.split(' ').for_each(|word| count_exist_word += count_exit_word(word));
        result_decodes.push(Decode {
            deciphered: result.clone(),
            shift: shift as u8,
            word_matches: count_exist_word,
        })
    }

    result_decodes
}

fn count_exit_word(str: &str) -> u8 {
    str.split(" ")
        .map(|word| Runtime::new().unwrap().block_on(is_exist(word)))
        .sum()
}

pub async fn is_exist(word: &str) -> u8 {
    let resp = reqwest::get(String::from("https://api.dictionaryapi.dev/api/v2/entries/en/") + word)
        .await
        .unwrap()
        .status();
    if resp == reqwest::StatusCode::OK {
        1
    } else {
        0
    }
}
