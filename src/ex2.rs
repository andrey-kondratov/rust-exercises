pub fn main(input: &str) -> String {
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    let mut result = String::new();
    for word in input.split(' ') {
        let mut chars = word.chars();
        let first = &chars.nth(0).expect("No first char?");
        result += &word[first.len_utf8()..];
        if word.len() > 1 {
            result += "-"
        };
        result += if is_vowel(first) { "hay" } else { "fay" };
        result += " ";
    }

    result
}

fn is_vowel(char: &char) -> bool {
    for vowel in "aouie".chars() {
        if &vowel == char {
            return true;
        }
    }

    false
}
