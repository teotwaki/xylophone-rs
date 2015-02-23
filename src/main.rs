#[macro_use]
extern crate log;

mod configuration;
mod logging;
mod server;

fn main() {
    logging::install_logger();

    info!("Xylophone starting up");

    let args = match configuration::Arguments::parse() {
        Some(x) => { x }
        None => { return }
    };

	server::build();
}
