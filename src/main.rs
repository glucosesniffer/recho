use clap::{App, Arg};

fn main() {
    let matches = App::new("recho")
    .version("0.1.0")
    .author("Ahmad Saeed <ahmadsaeed3290@gmail.com>")
    .about("Rust echo")
    .arg(
        Arg::with_name("text")
        .value_name("TEXT")
        .help("Input text")
        .required(true)
        .min_values(1),
    )
    .arg(
        Arg::with_name("omit_newline")
        .short("n")
        .help("Don't print newline")
        .takes_value(false),
    )
    .get_matches();

    println!("{:?}", matches);
}

