pub fn cd(input: &str) -> std::io::Result<()> {
    let path = std::path::Path::new(input);
    std::env::set_current_dir(&path)?;
    Ok(())
}
