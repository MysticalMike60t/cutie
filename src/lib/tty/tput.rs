pub fn get_color_type() -> u8 {
    let command_output: u8 = match std::process::Command::new("tput").arg("colors").output() {
        Ok(number) => number.stdout[0],
        Err(_) => 16,
    };
    return command_output;
}
