pub mod parser;
pub mod interpreter;

#[cfg(test)]
mod test_helper;

#[derive(PartialEq, Debug)]
pub enum Token {
    Space,
    Tab,
    LineFeed
}

pub type Label = i64;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Instruction {
    PushNrOnStack(i64),

    /* Stack operations */
    DuplicateTopStack,
    CopyNthOnTop(i64),
    SwapTopTwoOnStack,
    DiscardTopOfStack,
    SlideNOfTopOfStackKeepTopItem(i64),

    /* Arithmetic */
    Addition,
    Substraction,
    Multiplication,
    IntegerDivision,
    Modulo,

    /* Heap access */
    StoreInHeap,
    RetreiveFomHeap,


    /* Flow control */
    MarkLocation(Label),
    CallSubroutine(Label),
    Jmp(Label),
    JmpTopZero(Label),
    JmpTopNegative(Label),
    Ret,
    End,

    /* IO */
    OutputChar,
    OutputNum,
    ReadChar,
    ReadNum
}
