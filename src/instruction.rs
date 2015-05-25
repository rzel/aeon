use types::IntegerVector;

#[derive(Debug)]
pub enum InstructionType {
    SetInteger,
    SetFloat,
    SetString,
    SetArray,
    SetHash,
    SetLocal,
    GetLocal,
    SetConstant,
    GetConstant,
    SetInstanceVariable,
    GetInstanceVariable,
    SendMessage,
    Return
}

pub struct Instruction {
    pub instruction_type: InstructionType,
    pub arguments: IntegerVector,
    pub line: usize,
    pub column: usize
}

impl Instruction {
    pub fn new(ins_type: InstructionType, arguments: IntegerVector, line: usize, column: usize) -> Instruction {
        Instruction {
            instruction_type: ins_type,
            arguments: arguments,
            line: line,
            column: column
        }
    }
}