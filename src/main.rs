use clap::App;

fn main() {
    let _matches = App::new("recho")
    .version("0.1.0")
    .author("Ahmad Saeed <ahmadsaeed3290@gmail.com>")
    .about("Rust echo")
    .get_matches();
}

