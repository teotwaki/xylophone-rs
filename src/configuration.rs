extern crate argparse;
extern crate core;

// Casting from isize to i32
use self::core::num::ToPrimitive;

// Setting the exit status
use std::env;

// Argument parsing
use self::argparse::{ArgumentParser, StoreTrue, Store};

// The struct that will contain the configuration
pub struct Arguments {
    pub listen: String,
    pub config_file: String
}

impl Arguments {
    pub fn new() -> Arguments {
        Arguments {
            listen: "127.0.0.1".to_string(),
            config_file: "".to_string()
        }
    }

    pub fn parse() -> Option<Arguments> {
        debug!("Parsing arguments");

        let mut args = Arguments::new();

        {  // this block limits scope of borrows by ap.refer() method
            let mut ap = ArgumentParser::new();
            ap.set_description("Cascade is a high-performance VoIP server and IVR.");
            ap.refer(&mut args.listen)
                .add_option(&["--listen"], Box::new(Store::<String>),
                "The interface on which to listen to");
            ap.refer(&mut args.config_file)
                .add_option(&["-c", "--configuration"], Box::new(Store::<String>),
                "The main configuration file");

            match ap.parse_args() {
                Ok(()) => {}
                Err(x) => {
                    env::set_exit_status(x.to_i32().unwrap());
                    return None;
                }
            }
        }

        Some(args)
    }
}
