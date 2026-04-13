use std::io::Error;
use std::process::ExitCode;
use clap::Parser;

#[derive(Parser)]
#[command(version, name = "cutie", about = "Prints cute stuff :3", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub prefix_type: String,
    #[arg(trailing_var_arg(true))]
    pub text: Vec<String>,
}

impl Args {
    pub fn convert_type<'a>(&self) -> &'a str {
        let prefix_type: &'a str = (*self.prefix_type).to_owned().leak();
        return prefix_type
    }
    pub fn insert_text<'a>(&self) -> &'a str {
        let text: &'a str = (*self.text).concat().leak();
        return text;
    }
}

fn main() -> Result<ExitCode, Error> {
    let args: Args = Args::parse();
    let prefix_type = match args.convert_type() {
        "debug" => "\x1b[38;2;200;140;240mDEBUG\x1b[0m",
        "success" => "\x1b[38;2;140;240;140mSUCCESS\x1b[0m",
        "warning" => "\x1b[38;2;240;240;140mWARNING\x1b[0m",
        "error" => "\x1b[38;2;240;140;140mERROR\x1b[0m",
        "info" => "\x1b[38;2;140;160;240mINFO\x1b[0m",
        &_ => ""
    };

    println!("\x1b[38;2;60;60;60m<\x1b[0m {} \x1b[38;2;60;60;60m>\x1b[0m    {}", prefix_type, args.insert_text());
    std::process::exit(1);
}
