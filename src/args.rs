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
        return prefix_type;
    }
    pub fn insert_text<'a>(&self) -> &'a str {
        let text: &'a str = (*self.text).concat().leak();
        return text;
    }
}
