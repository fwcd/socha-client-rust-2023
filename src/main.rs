use std::env;
use std::str::FromStr;
use simplelog::{SimpleLogger, Config};
use log::LevelFilter;
use getopts::Options;
use socha_client_2023::client::{SCClient, DebugMode};
use socha_client_2023::logic::OwnGameLogic;

fn print_usage(program: &str, options: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", options.usage(&brief));
}

fn main() {
    // Parse command line arguments
    let args = env::args().collect::<Vec<_>>();
    let mut options = Options::new();
    options.optopt("h", "host", "The game server's host address", "HOST");
    options.optopt("p", "port", "The game server's port", "PORT");
    options.optopt("r", "reservation", "A game reservation", "RESERVATION");
    options.optopt("l", "level", "Optionally provides a custom log level ('Info' by default)", "LEVEL");
    options.optflag("d", "debug-reader", "Reads incoming XML messages from the console for debugging");
    options.optflag("D", "debug-writer", "Prints incoming XML messages to the console for debugging");
    options.optflag("H", "help", "Prints usage info");
    
    let parsed_args = options.parse(&args[1..]).expect("Could not parse arguments!");
    if parsed_args.opt_present("help") {
        print_usage(&args[0], options);
        return;
    }
    
    let host = parsed_args.opt_str("host").unwrap_or("localhost".to_owned());
    let port = parsed_args.opt_str("port").unwrap_or("13050".to_owned()).parse::<u16>().expect("Invalid port.");
    let reservation = parsed_args.opt_str("reservation");
    let level = parsed_args.opt_str("level").unwrap_or("Info".to_owned());
    
    // Setup logging
    SimpleLogger::init(LevelFilter::from_str(&level).expect("Invalid log level."), Config::default()).expect("Could not initialize logger.");
    
    // Setup the client and the delegate
    let debug_mode = DebugMode {
        debug_reader: parsed_args.opt_present("debug-reader"),
        debug_writer: parsed_args.opt_present("debug-writer")
    };
    let client = SCClient::new(OwnGameLogic, debug_mode, reservation);
    
    let _result = client.connect(&host, port).expect("Error while running client.");
}
