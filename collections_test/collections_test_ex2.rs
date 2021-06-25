// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay”
// is added, so “first” becomes “irst-fay.” Words that start with a vowel have
// “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

fn main() {
    pig_latin_converter("first".to_string());
    pig_latin_converter("apple".to_string());
    pig_latin_converter("eternity".to_string());
}

fn pig_latin_converter(s: String) {
// if a, e, i, o, u, add '-hay' and end of word
// else, take first letter and add to 'ay', and place at end of word
    let mut s_vec: Vec<char> = s.chars().collect();
    
    match s_vec[0] {
        'a' | 'e' | 'i' | 'o' | 'u' => {
            let mut ending: Vec<char> = "-hay".chars().collect();
            s_vec.append(&mut ending);
            let pig_string: String = s_vec.into_iter().collect(); 
            println!("{}", pig_string);
        },
        _ => {
            let temp_end = format!("-{}ay", s_vec[0]);
            let mut ending: Vec<char> = temp_end.chars().collect();
            s_vec.remove(0);
            s_vec.append(&mut ending);
            let pig_string: String = s_vec.into_iter().collect(); 
            println!("{}", pig_string);
        },
    }
}