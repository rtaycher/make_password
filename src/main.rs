extern crate rand;
extern crate rustc_serialize;

#[macro_use]
extern crate clap;

extern crate itertools;

use rand::{thread_rng, Rng};
use clap::{App, Arg, SubCommand};
use itertools::Itertools;

static WORDS_FILE: &'static str = include_str!("american-english-small");


#[derive(Debug, RustcDecodable)]
struct Args {
    num_of_password_words: isize,
    add_random_sym_and_char: bool,
    mixed: bool,
    arg_interspersed_phrases: Vec<String>,
}

fn main() {

    let m = App::new("make_password")
                .version(crate_version!())
                .author("Roman A. Taycher <rtaycher1987@gmail.com>")
                .about("generate easy to remember passwords(not garunteed to be safe use at your \
                        own risk)")
                .arg(Arg::with_name("num_of_password_words")
                         .short("n")
                         .takes_value(false)
                         .default_value("5")
                         .help("Number of words in the password"))
                .arg(Arg::with_name("add_random_sym_and_char")
                         .short("a")
                         .takes_value(false)
                         .help("add a random symbol and char followed by a random char"))
                .subcommand(SubCommand::with_name("mixed")
                                .about("alternate password scheme mixing 2 phrases")
                                .version(crate_version!())
                                .author("Roman A. Taycher <rtaycher1987@gmail.com>")
                                .arg(Arg::with_name("p1")
                                         .help("phrase 1")
                                         .required(true)
                                         .index(1))
                                .arg(Arg::with_name("p2")
                                         .help("phrase 2")
                                         .required(true)
                                         .index(2)))
                .get_matches();
    let thing;
    if let Some(sub_command) = m.subcommand_matches("mixed") {
        let mut password = String::new();
        let p1 = sub_command.value_of("p1").unwrap();
        let p2 = sub_command.value_of("p2").unwrap();

        for ze in p1.chars().interleave(p2.chars()) {
            password.push(ze);
            // match ze {
            //    Both(ch1, ch2) => {
            //        password.push(ch1);
            //       password.push(ch2);
            //    }
            //    Left(ch1) => {
            //        password.push(ch1);
            //    }
            //    Right(ch1) => {
            //        password.push(ch1);
            //    }
            // }
        }
        let password = password.split_whitespace().collect::<Vec<_>>().concat();
        println!("password:\n\t{:}", password);
        return;
    }

    let words: Vec<&str> = WORDS_FILE.lines()
                                     .filter(|w| !w.ends_with("'s"))// ignore possessives
                                     .collect();
    let len = words.len();

    let mut rng = thread_rng();
    let mut password_words: Vec<&str> = (0..(value_t!(m, "num_of_password_words", u32)
                                                 .unwrap_or(5)))
                                            .map(|_| words[(rng.gen::<usize>() % len) - 1])
                                            .collect();
    let numbers = "0123456789".as_bytes();
    let symbols = "!@#$%^&*()-+".as_bytes();

    thing = format!("{:}{:}",
                    *rng.choose(&numbers).unwrap() as char,
                    *rng.choose(&symbols).unwrap() as char);

    if m.is_present("add_random_sym_and_char") {
        let thing_index = rng.gen::<usize>() % password_words.len();
        password_words.insert(thing_index, &thing);
    }
    let password = password_words.join(" ");
    println!("password:\n\t{:}", password);
}
