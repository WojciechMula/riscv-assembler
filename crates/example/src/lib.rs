use assembler::Context;
use assembler::assemble;
use std::path::Path;

mod label_first_pass;
use label_first_pass::FirstPassLabelResolver;

mod label_second_pass;
use label_second_pass::SecondPassLabelResolver;

// --------------------------------------------------

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! err {
    ($($args:expr),*) => {
        Err(format!($($args),*).as_str().into())
    };
}

pub fn assemble_file(path: &Path, ctx: &Context) -> crate::Result<Vec<u16>> {
    let mut result = Vec::<u16>::new();

    let source = std::fs::read_to_string(path)?;

    let mut label_resolver = FirstPassLabelResolver::default();

    for src_line in source.lines() {
        let mut line = src_line.trim_start();
        let indent = src_line.len() - line.len();

        label_resolver.asm_line_no += 1;

        if line.starts_with(".") {
            // an assembler directive
            continue;
        }

        match line.split_once(['#', ';']) {
            Some((body, _comment)) => {
                line = body;
            }
            None => (),
        }

        if let Some((label, rest)) = line.split_once(':') {
            line = rest.trim();

            label_resolver.asm_code = line.to_string();
            match label_resolver.register_label(label) {
                Ok(()) => (),
                Err(err) => {
                    println!("{src_line}");
                    return err!("line {}: {err}", label_resolver.asm_line_no);
                }
            }
        }

        if line.is_empty() {
            continue;
        }

        label_resolver.asm_code = line.to_string();

        let instr = match assemble(line, ctx, &mut label_resolver) {
            Ok(instr) => instr,
            Err(err) => {
                println!("{src_line}");
                if let Some(column) = err.column() {
                    println!("{}^", " ".repeat(indent + column));
                }

                return err!("line {}: {err}", label_resolver.asm_line_no);
            }
        };

        println!("{line:40} => {:08x}", instr);

        if is_uncompressed(instr) {
            result.push((instr & 0xffff) as u16);
            result.push((instr >> 16) as u16);
            label_resolver.current_offset += 4;
        } else {
            result.push(instr as u16);
            label_resolver.current_offset += 2;
        }
    }

    // resolve labels
    let mut second_pass_resolver = SecondPassLabelResolver::new(&label_resolver)?;
    let show_header = true;
    for (_, label_info) in &label_resolver.labels {
        if label_info.references.is_empty() {
            continue;
        }

        assert!(label_info.offset.is_some());

        if label_info.offset.is_some() {
            for r in &label_info.references {
                match assemble(&r.asm_code, ctx, &mut second_pass_resolver) {
                    Err(_) => unreachable!(), // the instruction has been already assembled, it must not fail
                    Ok(instr) => {
                        if show_header {
                            println!();
                            println!("patching branches");
                        }
                        println!("{:40} => {instr:04x}", r.asm_code);

                        // replace the instructions
                        let index = (r.offset / 2) as usize;
                        if is_uncompressed(instr) {
                            result[index + 0] = (instr & 0xffff) as u16;
                            result[index + 1] = (instr >> 16) as u16;
                        } else {
                            result[index + 0] = (instr & 0xffff) as u16;
                        }
                    }
                }
            }
        } else {
        }
    }

    Ok(result)
}

fn is_uncompressed(opcode: u32) -> bool {
    // "All the 32-bit instructions in the base ISA have their lowest two bits set to 11.
    //  The optional compressed 16-bit instruction-set extensions have their lowest two
    //  bits equal to 00, 01, or 10."
    (opcode & 0b11) == 0b11
}
