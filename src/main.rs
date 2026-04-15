use clap::Parser;
use env_logger;
use log::{debug, info, trace};
use std::io::Error;
use std::process::ExitCode;
use uwulib::tty;

pub mod args;
pub mod print;

fn main() -> Result<ExitCode, Error> {
    env_logger::init();
    let args: args::Args = args::Args::parse();
    // let prefix_type = match args.convert_type() {
    //     "debug" => "\x1b[38;2;200;140;240mDEBUG\x1b[0m",
    //     "success" => format!("\x1b[38;2;{}mSUCCESS\x1b[0m", uwulib::colors::rgb::SUCCESS.insert()),
    //     "warning" => "\x1b[38;2;240;240;140mWARNING\x1b[0m",
    //     "error" => "\x1b[38;2;240;140;140mERROR\x1b[0m",
    //     "info" => "\x1b[38;2;140;160;240mINFO\x1b[0m",
    //     &_ => "",
    // };

    // println!(
    //     "\x1b[38;2;60;60;60m<\x1b[0m {} \x1b[38;2;60;60;60m>\x1b[0m    {}",
    //     prefix_type,
    //     args.insert_text()
    // );

    debug!("Checking if user asked to test color...");
    if args.test_color {
        trace!("Value of \"args.test_color\": {}", args.test_color);
        debug!("Printing test result...");
        println!(
            "\x1b[1mTTY\x1b[0m \x1b[1mColor\x1b[0m (via \"\x1b[35mtput\x1b[0m \x1b[33mcolors\x1b\"\x1b[93m:\x1b[0m \x1b[96m{}\x1b[0m",
            tty::tput::get_color_type()
        );
        debug!("Printed test result.");
    } else {
        info!("No test was performed.")
    }

    debug!("Calling main \"print\" function...");
    print::print(args.prefix_type.leak(), args.text.concat().leak());
    debug!("Printed.");
    info!("Exiting...");
    std::process::exit(1);
}
