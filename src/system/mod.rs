mod words;
use rand::prelude::*;
use words::valid_words;

pub struct System {
    selected_word: String,
    current_word: String,
    past_words: Vec<String>,
    valid_words: Vec<String>,
}

impl System {
    // 생성자
    pub fn new() -> System {
        System {
            selected_word: String::from(""),
            current_word: String::from(""),
            past_words: Vec::new(),
            valid_words: vec![],
        }
    }

    // 랜덤으로 selected word를 고르는 메소드
    pub fn random_selected_word(&mut self) -> () {
        self.selected_word =
            String::from(words::valid_words[thread_rng().gen_range(0..words::valid_words.len())]);

        println!("{}", self.selected_word);
    }

    // current_word를 update하는 메소드
    pub fn update_current_word(&mut self, input_word: String) -> () {
        match self.vaildate_input_word(&input_word) {
            true => {
                self.add_past_word();
                self.current_word = input_word;

                println!("{}", self.current_word);
                println!("{:?}", self.past_words);
            }
            _ => {
                println!("유효한 알파벳으로 구성된 단어를 입력해주세요.")
            }
        }
    }

    // 입력된 단어를 검증하는 메소드
    pub fn vaildate_input_word(&self, input_word: &String) -> bool {
        // 유효한 알파벳 문자로 구성되어있는 지 확인
        for character in input_word.chars() {
            match character {
                'a'..='z' => (),
                _ => {
                    return false;
                }
            }
        }

        // 유효한 단어 목록에 포함되어있는 지 확인
        return self.valid_words.contains(input_word);
    }

    // current_word를 past_words에 add 하는 메소드
    pub fn add_past_word(&mut self) -> () {
        if self.current_word.len() != 0 {
            self.past_words.push(self.current_word.clone())
        }
    }
}