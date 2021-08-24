#[cfg(feature = "readline")]
use rustyline::{Editor, Result};

use std::io::Write;

#[cfg(feature = "readline")]
pub fn display(crusty_prompt: String) -> Result<()> {
    let mut rl = Editor::<()>::new();
    let prompt = rl.readline(&std::env::var("PROMPT").unwrap_or(crusty_prompt))?;
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    Ok(())
}

#[cfg(not(feature = "readline"))]
pub fn display(crusty_prompt: String) {
    let prompt = std::env::var("PROMPT").unwrap_or(crusty_prompt);
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
}
