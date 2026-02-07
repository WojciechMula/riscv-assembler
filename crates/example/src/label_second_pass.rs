use crate::FirstPassLabelResolver;
use crate::err;
use assembler::LabelResolverTrait;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct SecondPassLabelResolver {
    pub current_offset: u64,
    pub offsets: BTreeMap<String, u64>,
}

impl SecondPassLabelResolver {
    pub fn new(lr: &FirstPassLabelResolver) -> crate::Result<Self> {
        let mut result = Self::default();

        for (label, info) in &lr.labels {
            if info.references.is_empty() {
                continue;
            }

            if let Some(offset) = info.offset {
                result.offsets.insert(label.clone(), offset);
            } else {
                println!("label '{}' referenced at lines:", label);
                for r in &info.references {
                    println!("{:4}: {}", r.asm_line_no, r.asm_code);
                }

                return err!("undefined label '{}'", label);
            }
        }

        Ok(result)
    }
}

impl LabelResolverTrait for SecondPassLabelResolver {
    fn lookup(&mut self, name: &str) -> i64 {
        match self.offsets.get(name) {
            Some(offset) => {
                let offset = *offset;
                if self.current_offset <= offset {
                    let diff = offset - self.current_offset;
                    let diff = diff as i64;

                    diff
                } else {
                    let diff = self.current_offset - offset;
                    let diff = diff as i64;

                    -diff
                }
            }
            None => {
                unreachable!("{name} not found");
            }
        }
    }
}
