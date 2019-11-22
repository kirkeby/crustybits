#[derive(Debug)]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;


enum Letter {
    Char(char),
}

impl Letter {
    fn matches(&self, c: char) -> bool {
        match self {
            &Letter::Char(a) => a == c,
        }
    }
}

pub struct Re {
    compiled: Vec<Letter>,
}

impl Re {
    pub fn compile(re: &str) -> Result<Self> {
        let mut letters = Vec::new();
        for c in re.chars() {
            match c {
                '.' | '?' | '*' | '|' | '^' | '$' | '\\' | '[' | ']' | '(' | ')' => unimplemented!(),
                _ => letters.push(Letter::Char(c)),
            }
        }
        Ok(Re { compiled: letters })
    }

    pub fn matches(&self, s: &str) -> bool {
        let s = s.chars().collect::<Vec<_>>();

        let mut a = 0; // index into self.compiled
        let mut b = 0; // index into s

        loop {
            if a == self.compiled.len() {
                break
            }
            if b == s.len() {
                return false
            }
            if !self.compiled[a].matches(s[b]) {
                return false
            }
            a += 1;
            b += 1;
        }

        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_match_works() -> Result<()> {
        let re = Re::compile("Hello, World!")?;
        assert!(re.matches("Hello, World!"));
        assert!(! re.matches("Hello, "));
        assert!(re.matches("Hello, World!\r\n"));
        Ok(())
    }

    #[test]
    fn anchor_match_works() -> Result<()> {
        let re = Re::compile("^Hello, World!$")?;
        assert!(re.matches("Hello, World!"));
        assert!(! re.matches("Hello, "));
        assert!(! re.matches("Hello, World!\r\n"));
        Ok(())
    }
}
