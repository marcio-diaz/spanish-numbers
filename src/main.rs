extern crate getopts;
extern crate spanish_numbers;

use getopts::Options;
use spanish_numbers::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} POSITIVE_NUMBER [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("s", "short", "use short scale");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("n", "newline", "use newline as separator");

    if args.len() < 2 {
        print_usage(&program, opts);
        return;
    }

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(msg) => {
            println!("{}", msg);
            print_usage(&program, opts);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let num = match u128::from_str_radix(&args[1], 10) {
        Ok(num) => num,
        Err(msg) => {
            println!("{}", msg);
            print_usage(&program, opts);
            return;
        } 
    };

    let scale_type = if matches.opt_present("s") { ScaleType::Short } else { ScaleType::Long };
    let separator = if matches.opt_present("n") { "\n" } else { " " };
    let ns = NumberToSpanish::new(scale_type);
    let translation = ns.number_to_spanish(num, separator);

    println!("{}", translation);
}