mod configuration;

fn main() {
    let args = match configuration::Arguments::parse() {
        Some(x) => { x }
        None => { return }
    };

    println!("{}", args.verbose)
}
