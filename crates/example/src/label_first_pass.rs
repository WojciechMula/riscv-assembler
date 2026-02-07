use crate::err;
use assembler::LabelResolverTrait;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct FirstPassLabelResolver {
    pub current_offset: u64,
    pub asm_line_no: usize,
    pub asm_code: String,
    pub labels: BTreeMap<String, Label>,
}

#[derive(Debug)]
pub struct Label {
    pub offset: Option<u64>,
    pub references: Vec<LabelReference>,
}

#[derive(Debug)]
pub struct LabelReference {
    pub asm_code: String,
    pub asm_line_no: usize,
    pub offset: u64,
}

impl LabelResolverTrait for FirstPassLabelResolver {
    fn lookup(&mut self, name: &str) -> i64 {
        const OFFSET_PLACEHOLDER: i64 = 0;

        match self.labels.get_mut(name) {
            Some(entry) => {
                if let Some(offset) = entry.offset {
                    if self.current_offset <= offset {
                        let diff = offset - self.current_offset;
                        let diff = diff as i64;

                        diff
                    } else {
                        let diff = self.current_offset - offset;
                        let diff = diff as i64;

                        -diff
                    }
                } else {
                    let reference = LabelReference {
                        asm_code: self.asm_code.clone(),
                        asm_line_no: self.asm_line_no,
                        offset: self.current_offset,
                    };

                    entry.references.push(reference);

                    OFFSET_PLACEHOLDER
                }
            }
            None => {
                let reference = LabelReference {
                    asm_code: self.asm_code.clone(),
                    asm_line_no: self.asm_line_no,
                    offset: self.current_offset,
                };
                let label = Label {
                    offset: None,
                    references: vec![reference],
                };
                self.labels.insert(name.to_string(), label);

                OFFSET_PLACEHOLDER
            }
        }
    }
}

impl FirstPassLabelResolver {
    pub fn register_label(&mut self, name: &str) -> crate::Result<()> {
        match self.labels.get_mut(name) {
            Some(entry) => {
                if entry.offset.is_none() {
                    entry.offset = Some(self.current_offset);

                    Ok(())
                } else {
                    err!("label {name} already defined")
                }
            }
            None => {
                if !is_valid_label(name) {
                    return err!("'{name}' is not a valid label");
                }

                let label = Label {
                    offset: Some(self.current_offset),
                    references: vec![],
                };

                self.labels.insert(name.to_string(), label);

                Ok(())
            }
        }
    }
}

fn is_valid_label(label: &str) -> bool {
    label.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}
