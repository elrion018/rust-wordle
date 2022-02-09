mod words;
use colored::*;
use rand::prelude::*;
use words::valid_words;

pub type CorrectLetter = Option<bool>;

pub struct System {
    target_word: String,
    current_word: String,
    past_words: [Option<String>; 5],
}

impl System {
    // 생성자
    pub fn new() -> System {
        System {
            target_word: String::from(""),
            current_word: String::from(""),
            past_words: [None, None, None, None, None],
        }
    }

    // 랜덤으로 selected word를 고르는 메소드
    pub fn random_target_word(&mut self) -> () {
        self.target_word =
            String::from(words::valid_words[thread_rng().gen_range(0..words::valid_words.len())]);

        println!("{}", self.target_word);
    }

    // current_word를 update하는 메소드
    pub fn update_current_word(&mut self, input_word: String) -> () {
        match self.vaildate_input_word(&input_word) {
            true => {
                self.add_past_word();
                self.current_word = input_word;
            }
            _ => (),
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
        return words::valid_words.contains(&input_word.as_str());
    }

    // current_word를 past_words에 add 하는 메소드
    pub fn add_past_word(&mut self) -> () {
        let first_none = self.past_words.iter_mut().find(|x| x.is_none());

        if let Some(item) = first_none {
            if self.current_word.len() != 0 {
                item.replace(self.current_word.clone());
            }
        }
    }

    // check_word String의 char들과 target_word String의 char들을 비교하는 메소드
    // 인덱스와 char끼리 일치하는 경우 Some(true)
    pub fn check_by_target_word(&self, check_word: &String) -> [CorrectLetter; 5] {
        let mut result = [Some(true); 5];

        for (i, (target_word_char, check_word_char)) in
            self.target_word.chars().zip(check_word.chars()).enumerate()
        {
            result[i] = if target_word_char == check_word_char {
                Some(true)
            } else if self.target_word.chars().any(|c| c == check_word_char) {
                Some(false)
            } else {
                None
            }
        }

        result
    }

    // format 배열의 원소와 word의 char들에 따라 coloredString을 원소로 갖는 백터를 반환하는 메소드
    pub fn format_with_color(
        &self,
        word: &String,
        format: [CorrectLetter; 5],
    ) -> Vec<ColoredString> {
        word.chars()
            .enumerate()
            .map(|(i, c)| match format[i] {
                Some(true) => c.to_string().green(),
                Some(false) => c.to_string().yellow(),
                None => c.to_string().normal(),
            })
            .collect()
    }

    pub fn display_words(&mut self) -> () {
        print!("\x1B[2J");
        print!("\x1B[0;0f");

        println!("┏━━━━━━━━━━━━━━━━━┓");
        println!("┃   W O R D L E   ┃");
        println!("┣━━━━━━━━━━━━━━━━━┫");

        let mut lines = 0;
        for past_word in self.past_words.iter() {
            if let Some(word) = past_word {
                print!("┃    ");
                let formatted_words = self.format_with_color(word, self.check_by_target_word(word));
                formatted_words.iter().for_each(|c| print!("{} ", c));
                println!("   ┃");
                lines += 1
            } else {
                break;
            }
        }

        if self.current_word.len() != 0 {
            print!("┃    ");
            let formatted_words = self.format_with_color(
                &self.current_word,
                self.check_by_target_word(&self.current_word),
            );
            formatted_words.iter().for_each(|c| print!("{} ", c));
            println!("   ┃");
            lines += 1;
        }

        for _ in lines..self.past_words.len() + 1 {
            println!("┃                 ┃");
        }
        println!("┗━━━━━━━━━━━━━━━━━┛");

        if self.win() {
            println!("정답!");
        } else if self.lose() {
            println!("정답은 {} 이랍니다.", self.target_word);
        }
    }

    pub fn win(&self) -> bool {
        self.current_word == self.target_word
    }

    pub fn lose(&self) -> bool {
        !self.win() && self.past_words.iter().all(|x| x.is_some())
    }

    pub fn over(&self) -> bool {
        self.win() || self.lose()
    }
}
