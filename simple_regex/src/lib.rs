#[derive(Debug)]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;


enum Pattern {
    Any,
    Char(char),
    //AnchorStart,
    //AnchorEnd,
    //Many(Box<Pattern>),
    Optional(Vec<Pattern>),
}

struct Search<'a> {
    s: &'a [char],
    p: &'a [Pattern],
}

pub struct Re {
    compiled: Vec<Pattern>,
}

impl Re {
    pub fn compile(re: &str) -> Result<Self> {
        let mut letters = Vec::new();
        for c in re.chars() {
            let next = match c {
                //'^' => Pattern::AnchorStart,
                //'$' => Pattern::AnchorEnd,
                '|' | '\\' | '[' | ']' | '(' | ')' => unimplemented!(),
                '.' => Pattern::Any,
                '?' => Pattern::Optional(vec![letters.pop().unwrap()]),
                //'*' => Pattern::Many(Box::new(letters.pop().unwrap())),
                _ => Pattern::Char(c),
            };
            letters.push(next);
        }
        Ok(Re { compiled: letters })
    }

    pub fn matches(&self, s: &str) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        self.match_at(Search { s: &s, p: &self.compiled })
    }

    fn match_at(&self, search: Search) -> bool {
        if search.p.len() == 0 {
            return true
        }
        else if search.s.len() == 0 {
            return false
        }
        else {
            match self.match_head(search) {
                None => false,
                Some(search) => self.match_at(search),
            }
        }
    }

    fn match_head<'a>(&self, search: Search<'a>) -> Option<Search<'a>> {
        assert!(search.p.len() != 0);
        assert!(search.s.len() != 0);
        match search.p[0] {
            Pattern::Any => {
                Some(Search { p: &search.p[1..], s: &search.s[1..] })
            }
            Pattern::Char(c) if c == search.s[0] => {
                Some(Search { p: &search.p[1..], s: &search.s[1..] })
            }
            Pattern::Char(_) => None,
            Pattern::Optional(ref p) => {
                let s = match self.match_head(Search { p: p, s: search.s }) {
                    Some(Search { s, .. }) => s,
                    None => search.s,
                };
                Some(Search { p: &search.p[1..], s: s })
            }
        }
    }

    /*
    fn match_many(&self, l: &Box<Pattern>, r: &[Pattern], s: &[char]) -> bool {
        // FIXME - make me greedy?
        loop {
            if self.match_at(r, s) {
                return true;
            }
        }
    }
    */
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

    /*
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
    */
}
