use std::char;

fn cesar_cipher(letter: char, shift: u8) -> char {
    let letter = letter as u8;
    match letter {
        b'A'..=b'Z' => {
            let alphabetic_value = (letter - 'A' as u8) % 26;
            let shifted_value = (alphabetic_value + shift) % 26;
            let encrypted_value = (shifted_value + 'A' as u8) as char;
            return encrypted_value;
        }
        b'a'..=b'z' => {
            let alphanumeric_value = (letter - 'a' as u8) % 26;
            let shifted_value = (alphanumeric_value + shift) % 26;
            let encrypted_value = (shifted_value + 'a' as u8) as char;
            return encrypted_value;
        }
        _ => println!("please enter a letter"),
    }
    letter as char
}

fn main() {
    let word = "ChainSaw";
    let encrypted_word: String = word.trim().chars().map(|c| cesar_cipher(c, 30)).collect();
    println!("{}", encrypted_word);
    let decrypted_word: String = word.trim().chars().map(|c| decrypt_cipher(c)).collect();
    println!("{}", decrypted_word);
}

fn decrypt_cipher(word: char) -> char {
    let word = word as u8;
    let shift = 0;
    while shift <= 26 {
        match word {
            b'A'..=b'Z' => {
                let alphanumeric_value = (word - 'A' as u8) % 26;
                let unshifted_value = (alphanumeric_value - shift) % 26;
                let unencrypted_value = (unshifted_value + 'A' as u8) as char;
                return unencrypted_value;
            }
            b'a'..=b'z' => {
                let alphanumeric_value = (word - 'a' as u8) % 26;
                let unshifted_value = (alphanumeric_value - shift) % 26;
                let unencrypted_value = (unshifted_value + 'a' as u8) as char;
                return unencrypted_value;
            }
            _ => break,
        }
    }
    word as char
}
