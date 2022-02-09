mod system;

use std::io::stdin;
use system::System;

fn main() {
    let mut game_system = System::new();
    game_system.start();

    // 사용자가 단어를 입력하게끔 한다.
    loop {
        let mut guess_word = String::new();
        game_system.display_words();
        stdin().read_line(&mut guess_word);
        guess_word = guess_word.trim().to_string().to_lowercase();
        match guess_word.len() {
            5 => {
                game_system.update_current_word(guess_word);
                game_system.display_words();
            }
            _ => {
                continue;
            }
        }

        if game_system.over() {
            break;
        }
    }
}
