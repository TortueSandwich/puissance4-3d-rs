#![allow(unused_imports, dead_code)]
use log::{debug, error, info, trace, warn};
use log4rs;
use puissance::run;

fn main() {
    if let Err(e) = log4rs::init_file("logger_config.yaml", Default::default()) {
        panic!("{}", e);
    }
    info!("Logger succesfully set\n\tStarting program . . .");

    info!("Begin of the game");
    run();
    info!("End of the game");
    info!("End of program");
}
