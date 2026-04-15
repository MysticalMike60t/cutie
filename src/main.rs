use clap::Parser;
use std::io::Error;
use std::process::ExitCode;

pub mod args;
pub mod print;

fn main() -> Result<ExitCode, Error> {
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
    print::print(args.prefix_type.leak(), args.text.concat().leak());
    std::process::exit(1);
}
