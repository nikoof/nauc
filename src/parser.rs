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

            /* Explanation for the following two operations:
            - To find the matching ] for a [, we keep track of a number that
                 - is incremented when we hit [
                 - is decremented when we hit ]
                 - is unmodified otherwise
            - Once this value reaches 0, we have hit the matching bracket.
            - Its index is the index of the first 0 in the map. */
            '[' => {
                let mut count = 1;
                let jump = chars[i + 1..]
                    .iter()
                    .map(move |ch| {
                        count += match ch {
                            '[' => 1,
                            ']' => -1,
                            _ => 0,
                        };

                        count
                    })
                    .position(|count| count == 0);

                if let Some(jump) = jump {
                    Ok(Token::Break(i + 1 + jump))
                } else {
                    Err(ParserError::UnmatchedBracket('['))
                }
            }
            ']' => {
                let mut count = -1;
                let jump = chars[..i]
                    .iter()
                    .rev()
                    .map(move |ch| {
                        count += match ch {
                            '[' => 1,
                            ']' => -1,
                            _ => 0,
                        };

                        count
                    })
                    .position(|count| count == 0);

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
