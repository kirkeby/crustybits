use std::collections::HashSet;

#[derive(Debug)]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
enum Pattern {
    Any,
    Char(char),
    AnchorStart,
    AnchorEnd,
    Star(Vec<Pattern>),
    Optional(Vec<Pattern>),
    Capture(Vec<Pattern>),
    Bracket(HashSet<char>, bool),
}

pub struct Re {
    compiled: Vec<Pattern>,
}

impl Re {
    pub fn compile(re: &str) -> Result<Self> {
        let mut letters = Vec::new();
        let mut chars = re.chars();
        while let Some(c) = chars.next() {
            let next = match c {
                '|' | '\\' => unimplemented!(), // FIXME
                '^' => Pattern::AnchorStart,
                '$' => Pattern::AnchorEnd,
                '[' => Re::compile_bracket(&mut chars)?,
                '(' => Re::compile_capture(&mut chars)?,
                '.' => Pattern::Any,
                '?' => Pattern::Optional(vec![letters.pop().unwrap()]),
                '*' => Pattern::Star(vec![letters.pop().unwrap()]),
                _ => Pattern::Char(c),
            };
            letters.push(next);
        }
        Ok(Re { compiled: letters })
    }

    fn compile_capture(chars: &mut std::str::Chars) -> Result<Pattern> {
        let inner = Re::take_until(chars, ')')?;
        let s = inner.into_iter().collect::<String>();
        Ok(Pattern::Capture(Re::compile(&s)?.compiled))
    }

    fn compile_bracket(chars: &mut std::str::Chars) -> Result<Pattern> {
        let mut inner = Vec::new();
        let mut inverted = false;
        let mut first = true;
        for c in Re::take_until(chars, ']')? {
            match (first, c) {
                (true, '^') => inverted = true,
                (false, '-') => unimplemented!(),
                (_ , c) => { first = false; inner.push(c); }
            }
        }
        Ok(Pattern::Bracket(inner.into_iter().collect(), inverted))
    }

    fn take_until(chars: &mut std::str::Chars, end: char) -> Result<Vec<char>> {
        let mut inner = Vec::new();
        while let Some(c) = chars.next() {
            inner.push(c);
            if c == end { break }
        }
        assert!(inner.pop() == Some(end)); // FIXME
        Ok(inner)
    }

    pub fn matches(&self, s: &str) -> Option<Match> {
        let s = s.chars().collect::<Vec<_>>();
        let mut p = &self.compiled[..];
        if p[0] == Pattern::AnchorStart {
            p = &p[1..];
        }
        self.match_at(p, State::new(&s[..])).map(|s| {
            Match {
                matched: s.matched(),
                captured: s.c,
            }
        })
    }

    fn match_at<'a>(&self, p: &[Pattern], s: State<'a>) -> Option<State<'a>> {
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

    fn match_head<'a>(&self, p: &[Pattern], s: State<'a>) -> Option<State<'a>> {
        match p[0] {
            Pattern::AnchorStart => None,
            Pattern::AnchorEnd => None,
            Pattern::Any => Some(s.forward()),
            Pattern::Char(c) if c == s.peek() => Some(s.forward()),
            Pattern::Char(_) => None,
            Pattern::Bracket(ref c, false) if c.contains(&s.peek()) => Some(s.forward()),
            Pattern::Bracket(ref c, true) if !c.contains(&s.peek()) => Some(s.forward()),
            Pattern::Bracket(_, _) => None,
            Pattern::Optional(ref p) => {
                // FIXME - refactor to avoid clone?
                let s = match self.match_head(p, s.clone()) {
                    Some(s) => s,
                    None => s,
                };
                Some(s)
            }
            Pattern::Star(ref r) => {
                self.match_many(r, &p[1..], s)
            }
            Pattern::Capture(ref p) => {
                self.match_at(p, s.slice()).map(|t| s.capture(t))
            },
        }
    }

    fn match_many<'a>(&self, r: &[Pattern], p: &[Pattern], s: State<'a>) -> Option<State<'a>> {
        // FIXME - refactor to avoid clone?
        // FIXME - this is stupid inefficient, return (p, s)!
        if self.match_at(p, s.clone()).is_some() {
            return Some(s)
        }
        // FIXME - make me greedy!
        match self.match_at(r, s) {
            Some(t) => return self.match_many(r, p, t),
            None => return None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct State<'a> {
    s: &'a [char],
    n: usize,
    c: Vec<String>,
}

impl<'a> State<'a> {
    fn new(s: &'a [char]) -> State<'a> {
        State { s: s, n: 0, c: Vec::new() }
    }

    fn matched(&self) -> String {
        self.s[..self.n].iter().collect()
    }

    fn slice(&self) -> State<'a> {
        State::new(&self.s[self.n..])
    }

    fn len(&self) -> usize {
        self.s.len() - self.n
    }

    fn peek(&self) -> char {
        self.s[self.n]
    }

    fn forward(self) -> State<'a> {
        assert!(self.len() > 0);
        State { n: self.n + 1, ..self }
    }

    fn capture(mut self, t: State) -> State<'a> {
        assert!(self.len() >= t.n);
        self.c.push(t.matched());
        self.n = self.n + t.n;
        self
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
            re.matches("Hello, World!").unwrap().matched,
            "Hello, World!",
        );
        assert_eq!(
            re.matches("Hello, World!\r\n").unwrap().matched,
            "Hello, World!",
        );
        assert!(re.matches("Hello, ").is_none());
        Ok(())
    }

    #[test]
    fn match_qmark() -> Result<()> {
        let re = Re::compile("Hel?o,")?;
        assert!(re.matches("Heo,").is_some());
        assert!(re.matches("Helo,").is_some());
        assert!(re.matches("Hello,").is_none());
        Ok(())
    }

    #[test]
    fn string_match_star() -> Result<()> {
        let re = Re::compile("Hel*o,")?;
        assert!(re.matches("Heo,").is_some());
        assert!(re.matches("Helo,").is_some());
        assert!(re.matches("Hello,").is_some());
        assert!(re.matches("Helllo,").is_some());
        Ok(())
    }

    #[test]
    fn anchor_match_works() -> Result<()> {
        let re = Re::compile("^Hello, World!$")?;
        assert!(re.matches("Hello, World!").is_some());
        assert!(re.matches("Hello, ").is_none());
        assert!(re.matches("Hello, World!\r\n").is_none());
        Ok(())
    }

    #[test]
    fn can_capture() -> Result<()> {
        assert_eq!(
            Re::compile("(Hello)")?.matches("Hello"),
            Some(Match {
                matched: "Hello".into(),
                captured: vec!["Hello".into()],
            })
        );

        let re = Re::compile("(Hello), (World!)")?;
        assert_eq!(
            re.matches("Hello, World!"),
            Some(Match {
                matched: "Hello, World!".into(),
                captured: vec!["Hello".into(), "World!".into()],
            })
        );
        assert!(re.matches("Hello, ").is_none());
        Ok(())
    }

    #[test]
    fn bracket() -> Result<()> {
        assert_eq!(
            Re::compile("[abcdef]*$")?.matches("bacca").unwrap().matched,
            "bacca",
        );
        assert!(Re::compile("[abcdef]*llo")?.matches("Hello").is_none());
        Ok(())
    }

    #[test]
    fn inverted_bracket() -> Result<()> {
        assert_eq!(
            Re::compile("[^abc]*, ")?.matches("Hello, a").unwrap().matched,
            "Hello, ",
        );
        Ok(())
    }

    #[test]
    fn bracket_dash() -> Result<()> {
        assert_eq!(
            Re::compile("[0-9]* Hell")?.matches("42 Hellos").unwrap().matched,
            "42 Hell",
        );
        assert_eq!(
            Re::compile("[^0-9]*, ")?.matches("Hello, 42").unwrap().matched,
            "Hello, ",
        );
        Ok(())
    }

    #[test]
    fn greedy_star() -> Result<()> {
        assert_eq!(
            Re::compile("[0-9]*")?.matches("42 Hellos").unwrap().matched,
            "42",
        );
        Ok(())
    }
}
