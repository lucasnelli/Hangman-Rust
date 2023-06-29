use std::io::{self, Write};
use std::process;
use rand::seq::SliceRandom;

fn main() {
    let mut guessed_letters: Vec<char> = Vec::new();
    let words = vec![
        "leao", "girafa", "elefante", "tigre", "zebra", "macaco", "rinoceronte", "hipopotamo"
    ];
    let mut rng = rand::thread_rng();
    let word = words.choose(&mut rng).unwrap().to_lowercase();
    let mut attempts = 6;
    let mut hangman_state = 0;

    println!("Bem-vindo ao Jogo da Forca!");
    println!("Adivinhe a palavra animal!");
    println!("Dica: tem {} letras.", word.len());

    loop {
        clear_console();

        println!("\nPalavra: {}", display_word(&word, &guessed_letters));
        println!("Tentativas restantes: {}", attempts);
        print_hangman(hangman_state);
        println!("Letras já tentadas: {:?}", guessed_letters);

        if attempts == 0 {
            println!("Você perdeu! A palavra era {}.", word);
            wait_for_input();
            break;
        }

        print!("Digite uma letra: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().to_lowercase();

        if guess.len() != 1 {
            println!("Por favor, digite apenas uma letra!");
            wait_for_input();
            continue;
        }

        let letter = guess.chars().next().unwrap();

        if guessed_letters.contains(&letter) {
            println!("Você já tentou essa letra. Tente novamente!");
            wait_for_input();
            continue;
        }

        guessed_letters.push(letter);

        if !word.contains(letter) {
            attempts -= 1;
            hangman_state += 1;
            println!("A letra {} não está na palavra.", letter);
        }

        if is_word_guessed(&word, &guessed_letters) {
            println!("Parabéns! Você adivinhou a palavra {}.", word);
            wait_for_input();
            break;
        }
    }
}

fn display_word(word: &str, guessed_letters: &[char]) -> String {
    let mut displayed_word = String::new();
    for c in word.chars() {
        if guessed_letters.contains(&c) {
            displayed_word.push(c);
        } else {
            displayed_word.push('_');
        }
        displayed_word.push(' ');
    }
    displayed_word.trim().to_owned()
}

fn is_word_guessed(word: &str, guessed_letters: &[char]) -> bool {
    for c in word.chars() {
        if !guessed_letters.contains(&c) {
            return false;
        }
    }
    true
}

fn print_hangman(state: u8) {
    let hangman = [
        "  +---+\n  |   |\n      |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n / \\  |\n      |\n=========",
    ];

    println!("{}", hangman[state as usize]);
}

fn wait_for_input() {
    print!("Pressione Enter para sair...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn clear_console() {
    if cfg!(windows) {
        let _ = process::Command::new("cmd").args(&["/c", "cls"]).status();
    } else {
        let _ = process::Command::new("sh").arg("-c").arg("clear").status();
    }
}
