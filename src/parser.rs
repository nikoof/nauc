#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Right,
    Left,
    Add,
    Sub,
    Read,
    Write,
    Loop(usize),
    Break(usize),
    Comment,
}

pub fn ast<T: AsRef<str>>(source: T) -> Vec<Token> {
    let chars: Vec<char> = source
        .as_ref()
        .chars()
        .filter(|&ch| "<>+-.,[]".contains(|c| c == ch))
        .collect();

    chars
        .iter()
        .enumerate()
        .map(|(i, ch)| match ch {
            '>' => Token::Right,
            '<' => Token::Left,
            '+' => Token::Add,
            '-' => Token::Sub,
            '.' => Token::Write,
            ',' => Token::Read,
            '[' => {
                // TODO: refactor this monstrosity
                let mut jump = 0;
                let mut la = 1;
                let mut ra = 0;
                for (j, c) in chars[0..i].iter().enumerate() {
                    if *c == '[' {
                        la += 1;
                    }

                    if *c == ']' {
                        ra += 1;
                    }

                    if la == ra {
                        jump = j;
                        break;
                    }
                }
                Token::Break(i + jump)
            }
            ']' => {
                // TODO: refactor this monstrosity
                let mut jump = 0;
                let mut la = 0;
                let mut ra = 1;
                for (j, c) in chars[0..i].iter().rev().enumerate() {
                    if *c == '[' {
                        la += 1;
                    }

                    if *c == ']' {
                        ra += 1;
                    }

                    if la == ra {
                        jump = j;
                        break;
                    }
                }
                Token::Loop(i - 1 - jump)
            }
            _ => Token::Comment,
        })
        .collect()
}
