#[macro_use]
extern crate log;

mod configuration;
mod logging;

fn main() {
    logging::install_logger();

    info!("Cascade starting up");

    let args = match configuration::Arguments::parse() {
        Some(x) => { x }
        None => { return }
    };

    println!("{}", args.listen);
}
