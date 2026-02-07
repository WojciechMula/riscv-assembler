use assembler::Context;
use example::Result;
use example::assemble_file;
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    let ctx = Context::xlen64();

    for argument in env::args().skip(1) {
        println!("assembling {}", argument);
        let path = Path::new(&argument);
        let _ = assemble_file(&path, &ctx)?;
    }

    Ok(())
}
