use thiserror::Error;

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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParserError {
    #[error("Unmatched '{0}'")]
    UnmatchedBracket(char),
}

pub fn ast<T: AsRef<str>>(source: T) -> Result<Vec<Token>, ParserError> {
    let chars: Vec<char> = source
        .as_ref()
        .chars()
        .filter(|&ch| "<>+-.,[]".contains(|c| c == ch))
        .collect();

    chars
        .iter()
        .enumerate()
        .map(|(i, ch)| match ch {
            '>' => Ok(Token::Right),
            '<' => Ok(Token::Left),
            '+' => Ok(Token::Add),
            '-' => Ok(Token::Sub),
            '.' => Ok(Token::Write),
            ',' => Ok(Token::Read),
            '[' => {
                // TODO: refactor this monstrosity
                let mut jump = None;
                let mut la = 1;
                let mut ra = 0;
                for (j, c) in chars[i + 1..].iter().enumerate() {
                    if *c == '[' {
                        la += 1;
                    }

                    if *c == ']' {
                        ra += 1;
                    }

                    if la == ra {
                        jump = Some(j);
                        break;
                    }
                }

                if let Some(jump) = jump {
                    Ok(Token::Break(i + 1 + jump))
                } else {
                    Err(ParserError::UnmatchedBracket('['))
                }
            }
            ']' => {
                // TODO: refactor this monstrosity
                let mut jump = None;
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
                        jump = Some(j);
                        break;
                    }
                }

                if let Some(jump) = jump {
                    Ok(Token::Loop(i - 1 - jump))
                } else {
                    Err(ParserError::UnmatchedBracket(']'))
                }
            }
            _ => Ok(Token::Comment),
        })
        .collect()
}

mod tests {
    use super::*;
    use rstest::*;

    #[allow(unused)]
    type T = Token;

    #[rstest]
    #[case("", Ok(vec![]))]
    #[case(">", Ok(vec![T::Right]))]
    #[case("[>]", Ok(vec![T::Break(2), T::Right, T::Loop(0)]))]
    #[case(">>>>", Ok(vec![T::Right, T::Right, T::Right, T::Right]))]
    #[case(">+>+", Ok(vec![T::Right, T::Add, T::Right, T::Add]))]
    #[case("[[.-]]", Ok(vec![T::Break(5), T::Break(4), T::Write, T::Sub, T::Loop(1), T::Loop(0)]))]
    #[case("><+-.,[]", Ok(vec![T::Right, T::Left, T::Add, T::Sub, T::Write, T::Read, T::Break(7), T::Loop(6)]))]
    #[case("++++[>+.<-]", Ok(vec![T::Add, T::Add, T::Add, T::Add, T::Break(10), T::Right, T::Add, T::Write, T::Left, T::Sub, T::Loop(4)]))]
    #[case("[[+++>++]", Err(ParserError::UnmatchedBracket('[')))]
    #[case("[+>++]]]", Err(ParserError::UnmatchedBracket(']')))]
    fn test_parser(#[case] source: &str, #[case] expected: Result<Vec<T>, ParserError>) {
        assert_eq!(ast(source), expected);
    }
}
