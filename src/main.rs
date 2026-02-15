use std::env;
use std::process;

/// Parse a string into a u32.
/// Exits with error if parsing fails.
fn parse_u32(input: &str) -> u32 {
    input.parse::<u32>().unwrap_or_else(|_| {
        eprintln!("[FAIL] Invalid number: {}", input);
        process::exit(1);
    })
}

/// Print usage and exit.
fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("  bitcalc and <a> <b>");
    eprintln!("  bitcalc or  <a> <b>");
    eprintln!("  bitcalc xor <a> <b>");
    eprintln!("  bitcalc not <a>");
    eprintln!("  bitcalc shl <a> <b>");
    eprintln!("  bitcalc shr <a> <b>");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        usage();
    }

    let op = args[1].as_str();

    match op {
        "and" | "or" | "xor" | "shl" | "shr" => {
            if args.len() != 4 {
                usage();
            }

            let a = parse_u32(&args[2]);
            let b = parse_u32(&args[3]);

            let result = match op {
                "and" => a & b,
                "or" => a | b,
                "xor" => a ^ b,
                "shl" => a << b,
                "shr" => a >> b,
                _ => unreachable!(),
            };

            println!("{}", result);
        }

        "not" => {
            if args.len() != 3 {
                usage();
            }

            let a = parse_u32(&args[2]);
            let result = !a;
            println!("{}", result);
        }

        _ => usage(),
    }
}
