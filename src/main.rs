use clap::Parser;
use rand::prelude::*;

/// Random generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Random type, "num", "alpha", "alphanum", "lower", "upper"
    #[arg(short, long)]
    rt: String,

    /// Random length
    #[arg(short, long)]
    len: usize,
}

fn main() {
    let args = Args::parse();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    if args.rt == "num" {
        rand_num(args);
    } else if args.rt == "alpha" {
        rand_alpha(args);
    } else if args.rt == "alphanum" {
        rand_alphanum(args);
    } else if args.rt == "lower" {
        rand_lower(args);
    } else if args.rt == "upper" {
        rand_upper(args);
    }
}

fn rand_num(args: Args) {
    let mut rng = rand::thread_rng();
    for _ in 0..args.len {
        print!("{}", rng.gen_range(0..10));
    }
}

fn rand_alpha(args: Args) {
    let mut rng = thread_rng();
    let chars: String = (0..args.len)
        .map(|_| {
            let num = rng.gen_range(0..2);
            if num == 0 {
                rng.gen_range(97u8..123u8) as char
            } else {
                rng.gen_range(65u8..91u8) as char
            }
        })
        .collect();
    print!("{}", chars);
}

fn rand_alphanum(args: Args) {
    let s: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(args.len)
        .map(char::from)
        .collect();
    print!("{}", s);
}

fn rand_lower(args: Args) {
    let mut rng = thread_rng();
    let chars: String = (0..args.len)
        .map(|_| rng.gen_range(97u8..123u8) as char)
        .collect();
    print!("{}", chars);
}

fn rand_upper(args: Args) {
    let mut rng = thread_rng();
    let chars: String = (0..args.len)
        .map(|_| rng.gen_range(65u8..91u8) as char)
        .collect();
    print!("{}", chars);
}
