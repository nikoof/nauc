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
                for (j, c) in chars[i + 1..].iter().enumerate() {
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
                Token::Break(i + 1 + jump)
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

mod tests {
    use super::*;
    use rstest::*;

    type T = Token;

    #[rstest]
    #[case("", vec![])]
    #[case(">", vec![T::Right])]
    #[case("[>]", vec![T::Break(2), T::Right, T::Loop(0)])]
    #[case(">>>>", vec![T::Right, T::Right, T::Right, T::Right])]
    #[case(">+>+", vec![T::Right, T::Add, T::Right, T::Add])]
    #[case("[[.-]]", vec![T::Break(5), T::Break(4), T::Write, T::Sub, T::Loop(1), T::Loop(0)])]
    #[case("><+-.,[]", vec![T::Right, T::Left, T::Add, T::Sub, T::Write, T::Read, T::Break(7), T::Loop(6)])]
    #[case("++++[>+.<-]", vec![T::Add, T::Add, T::Add, T::Add, T::Break(10), T::Right, T::Add, T::Write, T::Left, T::Sub, T::Loop(4)])]
    fn test_parser(#[case] source: &str, #[case] expected: Vec<T>) {
        assert_eq!(ast(source), expected);
    }
}
