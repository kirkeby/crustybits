#[derive(Debug)]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;


enum Letter {
    //AnchorStart,
    //AnchorEnd,
    Any,
    Optional(Box<Letter>),
    Many(Box<Letter>),
    Char(char),
}

pub struct Re {
    compiled: Vec<Letter>,
}

impl Re {
    pub fn compile(re: &str) -> Result<Self> {
        let mut letters = Vec::new();
        for c in re.chars() {
            let next = match c {
                //'^' => Letter::AnchorStart,
                //'$' => Letter::AnchorEnd,
                '|' | '\\' | '[' | ']' | '(' | ')' => unimplemented!(),
                '.' => Letter::Any,
                '?' => Letter::Optional(Box::new(letters.pop().unwrap())),
                '*' => Letter::Many(Box::new(letters.pop().unwrap())),
                _ => Letter::Char(c),
            };
            letters.push(next);
        }
        Ok(Re { compiled: letters })
    }

    pub fn matches(&self, s: &str) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        self.match_at(&self.compiled, &s)
    }

    /// a: offset into self.compiled
    /// n: offset into s
    fn match_at(&self, r: &[Letter], s: &[char]) -> bool {
        if r.len() == 0 {
            return true
        }
        if s.len() == 0 {
            return false
        }
        match r[0] {
            Letter::Any => self.match_at(&r[1..], &s[1..]),
            Letter::Char(c) => c == s[0] && self.match_at(&r[1..], &s[1..]),
            Letter::Many(ref next) => self.match_many(next, &r[1..], &s[1..]),
            Letter::Optional(_) => unimplemented!(),
        }
    }

    fn match_many(&self, l: &Box<Letter>, r: &[Letter], s: &[char]) -> bool {
        // FIXME - make me greedy?
        loop {
            if self.match_at(r, s) {
                return true;
            }
        }
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
    fn string_match_star() -> Result<()> {
        let re = Re::compile("Hel*o,")?;
        assert!(re.matches("Heo,"));
        assert!(re.matches("Helo,"));
        assert!(re.matches("Hello,"));
        assert!(re.matches("Helllo,"));
        Ok(())
    }

    /*
    #[test]
    fn anchor_match_works() -> Result<()> {
        let re = Re::compile("^Hello, World!$")?;
        assert!(re.matches("Hello, World!"));
        assert!(! re.matches("Hello, "));
        assert!(! re.matches("Hello, World!\r\n"));
        Ok(())
    }
    */
}
