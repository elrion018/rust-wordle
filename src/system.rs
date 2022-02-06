pub struct System {
    selected_word: String,
    current_word: String,
    past_words: Vec<String>,
}

impl System {
    // 생성자
    pub fn new(selected_word: String) -> System {
        System {
            selected_word: selected_word.to_uppercase(),
            current_word: String::from(""),
            past_words: Vec::new(),
        }
    }

    // current_word의 setter
    pub fn set_current_word(&mut self, current_word: String) -> () {
        self.current_word = current_word;

        println!("{}", self.current_word)
    }

    // current_word를 past_words에 넣고 갱신하는 함수
    pub fn add_past_word(&mut self) -> () {
        self.past_words.push(self.current_word.clone())
    }
}
