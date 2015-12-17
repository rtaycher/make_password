extern crate rand;
use rand::{thread_rng, Rng};
static WORDS_FILE: &'static str = include_str!("american-english-small");

fn main() {
    let words: Vec<&str> = WORDS_FILE.lines().collect();
    let len = words.len();

    let mut rng = thread_rng();
    let password_words: Vec<&str> = (1..5).map(|_| words[rng.gen::<usize>() % len]).collect();
    // let numbers = "0123456789";
    // let symbols = "!@#$%^&*()-+";

  
    let password = password_words.join(" ");
    println!("{:}", password);
    // println!("# of words {:}", len);
}
