use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Right(usize),
    Left(usize),
    Add(u8),
    Sub(u8),
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

    let uncompressed_tokens: Vec<Token> = chars
        .iter()
        .map(|ch| match ch {
            '>' => Token::Right(1),
            '<' => Token::Left(1),
            '+' => Token::Add(1),
            '-' => Token::Sub(1),
            '.' => Token::Write,
            ',' => Token::Read,
            '[' => Token::Break(0),
            ']' => Token::Loop(0),
            _ => Token::Comment,
        })
        .collect();

    let compressed_tokens: Vec<Token> = uncompressed_tokens
        .iter()
        .chunk_by(|&&t| t)
        .into_iter()
        .flat_map(|(key, group)| {
            let original_tokens = group.cloned().collect_vec();
            let count = original_tokens.len();

            match key {
                Token::Right(_) => vec![Token::Right(count)],
                Token::Left(_) => vec![Token::Left(count)],
                Token::Add(_) => vec![Token::Add((count % 256) as u8)],
                Token::Sub(_) => vec![Token::Sub((count % 256) as u8)],
                _ => original_tokens,
            }
        })
        .collect();

    compressed_tokens
        .iter()
        .enumerate()
        .map(|(i, &token)| {
            match token {
                /* Explanation for the following two operations:
                - To find the matching ] for a [, we keep track of a number that
                     - is incremented when we hit [
                     - is decremented when we hit ]
                     - is unmodified otherwise
                - Once this value reaches 0, we have hit the matching bracket.
                - Its index is the index of the first 0 in the map. */
                Token::Break(_) => {
                    let mut count = 1;
                    let jump = compressed_tokens[i + 1..]
                        .iter()
                        .map(move |t| {
                            count += match t {
                                Token::Break(_) => 1,
                                Token::Loop(_) => -1,
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
                Token::Loop(_) => {
                    let mut count = -1;
                    let jump = compressed_tokens[..i]
                        .iter()
                        .rev()
                        .map(move |t| {
                            count += match t {
                                Token::Break(_) => 1,
                                Token::Loop(_) => -1,
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
                _ => Ok(token),
            }
        })
        .collect::<Result<Vec<Token>, ParserError>>()
}

mod tests {
    use super::*;
    use rstest::*;

    #[allow(unused)]
    type T = Token;

    #[rstest]
    #[case("", Ok(vec![]))]
    #[case(">", Ok(vec![T::Right(1)]))]
    #[case("[>]", Ok(vec![T::Break(2), T::Right(1), T::Loop(0)]))]
    #[case(">>>>", Ok(vec![T::Right(4)]))]
    #[case("comment\n>>>>\nanother [oops] comment", Ok(vec![T::Right(4), T::Break(2), T::Loop(1)]))]
    #[case(">+>+", Ok(vec![T::Right(1), T::Add(1), T::Right(1), T::Add(1)]))]
    #[case("[[.-]]", Ok(vec![T::Break(5), T::Break(4), T::Write, T::Sub(1), T::Loop(1), T::Loop(0)]))]
    #[case("><+-.,[]", Ok(vec![T::Right(1), T::Left(1), T::Add(1), T::Sub(1), T::Write, T::Read, T::Break(7), T::Loop(6)]))]
    #[case("++++[>+.<-]", Ok(vec![T::Add(4), T::Break(7), T::Right(1), T::Add(1), T::Write, T::Left(1), T::Sub(1), T::Loop(1)]))]
    #[case("[[+++>++]", Err(ParserError::UnmatchedBracket('[')))]
    #[case("[+>++]]]", Err(ParserError::UnmatchedBracket(']')))]
    fn test_parser(#[case] source: &str, #[case] expected: Result<Vec<T>, ParserError>) {
        assert_eq!(ast(source), expected);
    }
}
