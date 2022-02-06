mod system;

use std::io::stdin;
use system::System;

fn main() {
    // 초기 단어를 설정한다. 일단 Hello로 설정
    let selected_word = String::from("Hello");

    let gameSystem = System::new(selected_word);

    // 사용자가 단어를 입력하게끔 한다.
    loop {
        let mut guess_word = String::new();
        stdin().read_line(&mut guess_word);
        guess_word = guess_word.trim().to_string();
        match guess_word.len() {
            1..=4 => {
                println!("더 긴 단어를 입력해주세요.");
                continue;
            }
            5 => {
                println!("good");
            }
            _ => {
                println!("더 짧은 단어를 입력해주세요.");
                continue;
            }
        }
    }
}
