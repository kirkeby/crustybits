#[derive(Debug)]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
enum Pattern {
    Any,
    Char(char),
    AnchorStart,
    AnchorEnd,
    Many(Vec<Pattern>),
    Optional(Vec<Pattern>),
}

pub struct Re {
    compiled: Vec<Pattern>,
}

impl Re {
    pub fn compile(re: &str) -> Result<Self> {
        let mut letters = Vec::new();
        for c in re.chars() {
            let next = match c {
                '^' => Pattern::AnchorStart,
                '$' => Pattern::AnchorEnd,
                '|' | '\\' | '[' | ']' | '(' | ')' => unimplemented!(),
                '.' => Pattern::Any,
                '?' => Pattern::Optional(vec![letters.pop().unwrap()]),
                '*' => Pattern::Many(vec![letters.pop().unwrap()]),
                _ => Pattern::Char(c),
            };
            letters.push(next);
        }
        Ok(Re { compiled: letters })
    }

    pub fn matches(&self, s: &str) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let mut p = &self.compiled[..];
        if p[0] == Pattern::AnchorStart {
            p = &p[1..];
        }
        self.match_at(p, &s).is_some()
    }

    fn match_at<'a>(&self, p: &[Pattern], s: &'a [char]) -> Option<&'a [char]> {
        if p.len() == 0 {
            return Some(s)
        }

        if s.len() == 0 {
            if p == &[Pattern::AnchorEnd] {
                return Some(s)
            }
            return None
        }

        match self.match_head(p, s) {
            None => None,
            Some(s) => self.match_at(&p[1..], s),
        }
    }

    fn match_head<'a>(&self, p: &[Pattern], s: &'a [char]) -> Option<&'a [char]> {
        match p[0] {
            Pattern::AnchorStart => None,
            Pattern::AnchorEnd => None,
            Pattern::Any => {
                Some(&s[1..])
            }
            Pattern::Char(c) if c == s[0] => {
                Some(&s[1..])
            }
            Pattern::Char(_) => None,
            Pattern::Optional(ref p) => {
                let s = match self.match_head(p, s) {
                    Some(s) => s,
                    None => s,
                };
                Some(s)
            }
            Pattern::Many(ref r) => {
                self.match_many(r, &p[1..], s)
            }
        }
    }

    fn match_many<'a>(&self, r: &[Pattern], p: &[Pattern], s: &'a [char]) -> Option<&'a [char]> {
        // FIXME - this is stupid inefficient, return (p, s)!
        if self.match_at(p, s).is_some() {
            return Some(s)
        }
        // FIXME - make me greedy?
        match self.match_at(r, s) {
            Some(t) => return self.match_many(r, p, t),
            None => return None,
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
    fn match_qmark() -> Result<()> {
        let re = Re::compile("Hel?o,")?;
        assert!(re.matches("Heo,"));
        assert!(re.matches("Helo,"));
        assert!(! re.matches("Hello,"));
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

    #[test]
    fn anchor_match_works() -> Result<()> {
        let re = Re::compile("^Hello, World!$")?;
        assert!(re.matches("Hello, World!"));
        assert!(! re.matches("Hello, "));
        assert!(! re.matches("Hello, World!\r\n"));
        Ok(())
    }
}
