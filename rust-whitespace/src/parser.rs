use std::str::Chars;
use super::{Token, Instruction, Label};

#[derive(PartialEq, Debug)]
pub struct ParseError {
    message: &'static str
}

impl ParseError {
    fn err<T>(message: &'static str) -> Result<T, ParseError> {
        Err(ParseError {message: message} )
    }
}

pub fn next_token(iter: &mut Chars) -> Option<Token> {
    while let Some(c) = iter.next() {
        match c {
            ' ' => return Some(Token::Space),
            '\t' => return Some(Token::Tab),
            '\n' => return Some(Token::LineFeed),
            _ => continue
        }
    }
    None
}

pub fn expect_a_token(mut iter: &mut Chars) -> Result<Token, ParseError> {
    match next_token(&mut iter) {
        Some(token) => Ok(token),
        None => Err(ParseError { message: "Expected a token; but no more are available"})
    }
}

pub fn parse(source: &str) -> Result<Vec<Instruction>, ParseError> {
    use Token::*;
    let mut program = Vec::new();

    let mut chars = source.chars();
    while let Some(token) = next_token(&mut chars) {
        let instruction = match token {
            Space => parse_stack(&mut chars)?,
            Tab => match expect_a_token(&mut chars)? {
                /*tab => */ Space => parse_arithmetic(&mut chars)?,
                /*tab => */ Tab => parse_heap(&mut chars)?,
                /*tab => */ LineFeed => parse_io(&mut chars)?,
            }
            LineFeed => parse_flowcontrol(&mut chars)?
        };
        program.push(instruction);
    }

    Ok(program)
}

fn parse_stack(mut iter: &mut Chars) -> Result<Instruction, ParseError> {
    use Token::*;
    use Instruction::*;

    Ok(match expect_a_token(&mut iter)? {
        Space => PushNrOnStack(parse_number(&mut iter)?),
        fst => match (fst, expect_a_token(& mut iter)?) {
            (Space, _) => panic!("should be unreachable"),
            (LineFeed, Space)  => DuplicateTopStack,
            (Tab, Space) => CopyNthOnTop(parse_number(&mut iter)?),
            (LineFeed, Tab)  => SwapTopTwoOnStack,
            (LineFeed, LineFeed) => DiscardTopOfStack,
            (Tab, LineFeed) => SlideNOfTopOfStackKeepTopItem(parse_number(&mut iter)?),
            (Tab, Tab) => ParseError::err("STT instruction does not exist")?,
        }
    })
}

fn parse_arithmetic(mut iter: &mut Chars) -> Result<Instruction, ParseError> {
    use Token::*;
    use Instruction::*;

    Ok(match (expect_a_token(&mut iter)?, expect_a_token(&mut iter)?) {
        (Space, Space) => Addition,
        (Space, Tab) => Substraction,
        (Space, LineFeed) => Multiplication,
        (Tab, Space) => IntegerDivision,
        (Tab, Tab) => Modulo,
        (_, _) => ParseError::err("No such arithmetic instruction")?,
    })
}

fn parse_heap(mut iter: &mut Chars) -> Result<Instruction, ParseError> {
    use Token::*;
    use Instruction::*;

    Ok(match expect_a_token(&mut iter)? {
        Space => StoreInHeap,
        Tab => RetreiveFomHeap,
        LineFeed => ParseError::err("No such heap instruction")?,
    })
}

fn parse_flowcontrol(mut iter: &mut Chars) -> Result<Instruction, ParseError> {
    use Token::*;
    use Instruction::*;

    Ok(match (expect_a_token(&mut iter)?, expect_a_token(&mut iter)?) {
        (Space, Space) => MarkLocation(parse_label(&mut iter)?),
        (Space, Tab) => CallSubroutine(parse_label(&mut iter)?),
        (Space, LineFeed) => Jmp(parse_label(&mut iter)?),
        (Tab, Space) => JmpTopZero(parse_label(&mut iter)?),
        (Tab, Tab) => JmpTopNegative(parse_label(&mut iter)?),
        (Tab, LineFeed) => Ret,
        (LineFeed, LineFeed) => End,
        (_, _) => ParseError::err("No such flowcontrol instruction")?,
    })
}

fn parse_io(mut iter: &mut Chars) -> Result<Instruction, ParseError> {
    use Token::*;
    use Instruction::*;

    Ok(match (expect_a_token(&mut iter)?, expect_a_token(&mut iter)?) {
        (Space, Space) => OutputChar,
        (Space, Tab) => OutputNum,
        (Tab, Space) => ReadChar,
        (Tab, Tab) => ReadNum,
        (_, _) => ParseError::err("No such io instruction")?,
    })
}

fn parse_number(mut iter: &mut Chars) -> Result<i64, ParseError> {
    use Token::*;
    let sign = expect_a_token(&mut iter)?;
    let mut pow = 0;
    let mut val = 0;
    loop {
        match expect_a_token(&mut iter)? {
            Space => { /* zero */},
            Tab => { /* one*/
                val += 2_i64.pow(pow);
            },
            LineFeed => break,
        }
        pow += 1;
    }

    if sign  == Tab {
        val = val * -1;
    }

    Ok(val)
}

fn parse_label(mut iter: &mut Chars) -> Result<Label, ParseError> {
    parse_number(&mut iter)
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::test_helper::prepare_example;

    fn compile(tokens: &[Token]) -> String {
        use Token::*;
        let mut result = String::new();

        for token in tokens.iter() {
            let c = match token {
                Space => " ",
                Tab => "\t",
                LineFeed => "\n",
            };
            result += c;
        }


        result
    }

    #[test]
    fn test_lex_simple() {
        use Token::*;
        let tokens = [Space, Space, Tab, LineFeed];
        let input = compile(&tokens);
        let mut chars = input.chars();
        assert_eq!(Some(Space), next_token(&mut chars));
        assert_eq!(Some(Space), next_token(&mut chars));
        assert_eq!(Some(Tab), next_token(&mut chars));
        assert_eq!(Some(LineFeed), next_token(&mut chars));
        assert_eq!(None, next_token(&mut chars));
        assert_eq!(None, next_token(&mut chars));
    }

    #[test]
    fn test_parse_hello_world() {
        use super::Instruction::*;

        let hello_world = prepare_example(include_str!("example_programs/hello_world.txt"));
        let program = parse(&hello_world);
        assert_eq!(Ok(vec![PushNrOnStack(9), OutputChar, PushNrOnStack(83), OutputChar, PushNrOnStack(27), OutputChar, PushNrOnStack(27), OutputChar, PushNrOnStack(123), OutputChar, PushNrOnStack(13), OutputChar, PushNrOnStack(1), OutputChar, PushNrOnStack(119), OutputChar, PushNrOnStack(123), OutputChar, PushNrOnStack(39), OutputChar, PushNrOnStack(27), OutputChar, PushNrOnStack(19), OutputChar, End]), program);
    }

}

