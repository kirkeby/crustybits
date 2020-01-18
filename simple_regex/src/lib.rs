use libpcre2::{Code, Result};

pub struct Re {
    compiled: Code,
}

impl Re {
    pub fn compile(re: &str) -> Result<Self> {
        Ok(Re { compiled: Code::compile(re)? })
    }

    pub fn search(&self, s: &str) -> Option<Match> {
        let m = self.compiled.search(s).expect(".search should not fail?!");
        m.map(|m| Match {
            matched: m.group(0).expect("BUG? .group(0) failed"),
            captured: m.groups().expect("BUG? .groups() failed"),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Match {
    pub matched: String,
    pub captured: Vec<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_match_works() -> Result<()> {
        let re = Re::compile("Hello, World!")?;
        assert_eq!(
            re.search("Hello, World!").unwrap().matched,
            "Hello, World!",
        );
        assert_eq!(
            re.search("Hello, World!\r\n").unwrap().matched,
            "Hello, World!",
        );
        assert!(re.search("Hello, ").is_none());
        Ok(())
    }

    #[test]
    fn match_qmark() -> Result<()> {
        let re = Re::compile("Hel?o,")?;
        assert!(re.search("Heo,").is_some());
        assert!(re.search("Helo,").is_some());
        assert!(re.search("Hello,").is_none());
        Ok(())
    }

    #[test]
    fn string_match_star() -> Result<()> {
        let re = Re::compile("Hel*o,")?;
        assert!(re.search("Heo,").is_some());
        assert!(re.search("Helo,").is_some());
        assert!(re.search("Hello,").is_some());
        assert!(re.search("Helllo,").is_some());
        Ok(())
    }

    #[test]
    fn anchor_match_works() -> Result<()> {
        let re = Re::compile("^Hello, World!$")?;
        assert!(re.search("Hello, World!").is_some());
        assert!(re.search("Hello, ").is_none());
        assert!(re.search("Hello, World!\r\n").is_none());
        Ok(())
    }

    #[test]
    fn can_capture() -> Result<()> {
        assert_eq!(
            Re::compile("(Hello)")?.search("Hello"),
            Some(Match {
                matched: "Hello".into(),
                captured: vec!["Hello".into()],
            })
        );

        let re = Re::compile("(Hello), (World!)")?;
        assert_eq!(
            re.search("Hello, World!"),
            Some(Match {
                matched: "Hello, World!".into(),
                captured: vec!["Hello".into(), "World!".into()],
            })
        );
        assert!(re.search("Hello, ").is_none());
        Ok(())
    }

    #[test]
    fn bracket() -> Result<()> {
        assert_eq!(
            Re::compile("[abcdef]*$")?.search("bacca").unwrap().matched,
            "bacca",
        );
        assert!(Re::compile("^[abcdef]*llo")?.search("Hello").is_none());
        Ok(())
    }

    #[test]
    fn inverted_bracket() -> Result<()> {
        assert_eq!(
            Re::compile("[^abc]*, ")?.search("Hello, a").unwrap().matched,
            "Hello, ",
        );
        Ok(())
    }

    #[test]
    fn bracket_dash() -> Result<()> {
        assert_eq!(
            Re::compile("[0-9]* Hell")?.search("42 Hellos").unwrap().matched,
            "42 Hell",
        );
        assert_eq!(
            Re::compile("[^0-9]*, ")?.search("Hello, 42").unwrap().matched,
            "Hello, ",
        );
        Ok(())
    }

    #[test]
    fn greedy_star() -> Result<()> {
        assert_eq!(
            Re::compile("[0-9]*")?.search("42 Hellos").unwrap().matched,
            "42",
        );
        Ok(())
    }

    #[test]
    fn backtracking() -> Result<()> {
        assert_eq!(
            Re::compile("a*aaa")?.search("aaaaaaa"),
            Some(Match {
                matched: "aaaaaaa".into(),
                captured: vec![],
            })
        );
        assert_eq!(
            Re::compile("(a*)(aaa)")?.search("aaaaaaa"),
            Some(Match {
                matched: "aaaaaaa".into(),
                captured: vec!["aaaa".into(), "aaa".into()],
            })
        );
        assert_eq!(
            Re::compile("[ab]*ab")?.search("ababab").unwrap().matched,
            "ababab",
        );
        Ok(())
    }
}
