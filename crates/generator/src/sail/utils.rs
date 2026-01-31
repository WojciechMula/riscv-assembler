pub struct StringLine<'a> {
    pub lineno: usize,
    pub offset: usize,
    pub line: &'a str,
}

pub fn find_line<'a>(s: &'a str, pos: usize) -> Option<StringLine<'a>> {
    let mut lineno = 0;
    let mut string = s;

    let mut offset = 0;

    loop {
        if let Some((line, rest)) = string.split_once('\n') {
            if pos >= offset && pos < offset + line.len() {
                return Some(StringLine {
                    lineno,
                    offset,
                    line,
                });
            } else {
                string = rest;
                lineno += 1;
                offset += line.len() + 1;
            }
        } else {
            let line = string;
            if pos >= offset && pos < offset + line.len() {
                return Some(StringLine {
                    lineno,
                    offset,
                    line,
                });
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_line() {
        let s = r#"first line first line first line
            second line
            third line!
            fourh line
        "#;

        let res = find_line(s, 30).unwrap();

        assert_eq!(res.line, "first line first line first line");
        assert_eq!(res.lineno, 0);

        let res = find_line(s, 50).unwrap();

        assert_eq!(res.line, "            second line");
        assert_eq!(res.lineno, 1);

        let res = find_line(s, 60).unwrap();

        assert_eq!(res.line, "            third line!");
        assert_eq!(res.lineno, 2);
    }
}
