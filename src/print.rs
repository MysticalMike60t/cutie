cfg_if::cfg_if! {
    if #[cfg(feature = "ansi24bit")] {
        use uwulib::prefixes::ansi24bit::Prefix;
    } else if #[cfg(feature = "ansi256")] {
        use uwulib::prefixes::ansi256::Prefix;
    } else if #[cfg(feature = "ansi16")] {
        use uwulib::prefixes::ansi16::Prefix;
    } else {
        use uwulib::prefixes::ansi16::Prefix;
    }
}

#[cfg(all(feature = "ansi16", feature = "ansi256"))]
compile_error!("You cannot enable both \"ansi16\" and \"ansi256\" features at the same time.");

pub fn print<'prefix, 'text, 'output>(prefix_type: &'prefix str, text: &'text str) -> () {
    let prefix: Prefix = Prefix::new(prefix_type);
    let ansi: String = format!("{}    {}", prefix.insert(), text);
    println!("{}", (ansi.clone()).leak());
}
