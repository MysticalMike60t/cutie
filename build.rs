fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format_output = std::process::Command::new("cargo")
        .args(["fmt", "--message-format", "human", "--all"])
        .output()
        .unwrap();
    println!(
        "Format Output: {}",
        String::from_utf8_lossy(&format_output.stdout)
    );
    Ok(())
}
