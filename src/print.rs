use uwulib::prefixes::Prefix;

pub fn print<'prefix, 'text, 'output>(prefix_type: &'prefix str, text: &'text str) -> () {
    let prefix: Prefix = Prefix::new(prefix_type);
    let ansi: String = format!("{}    {}", prefix.insert(), text);
    println!("{}", (ansi.clone()).leak());
}
