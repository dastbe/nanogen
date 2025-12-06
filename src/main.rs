use clap::Parser;
use nanoid;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // nanoid alphabet
    #[arg(short, long, default_value = "abcdefghijklmnopqrstuvwxyz1234567890")]
    alphabet: String,

    // nanoid length
    #[arg(short, long, default_value_t = 10)]
    length: usize,
}

fn main() {
    let args = Args::parse();
    let x: Vec<char> = args.alphabet.chars().collect();

    let id = nanoid::format(nanoid::rngs::default, &x, args.length);

    print!("{}", id)
}
