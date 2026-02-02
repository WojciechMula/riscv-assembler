use generator::generate;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    stderrlog::new().verbosity(3).init()?;

    let input = Path::new("assets/dump.sail");
    let pseudo = Path::new("assets/pseudoinstructions.sail");
    let output = Path::new("crates/assembler/src/assembler.rs");
    generate(input, pseudo, output)?;

    Ok(())
}
