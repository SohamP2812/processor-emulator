struct Operand {
    token: Token
}

struct Instruction {
    operation: u8,
    operands: Vec<Operand>
}

struct Lexer {
    input: String
}

enum TokenType {
    Operation,
    Data,
    Label,
    Register,
    Immediate
}

struct Token {
    token_type: TokenType,
    value: String
}

struct Parser {
    tokens: Vec<Token>
}

struct Assembler {
    parser: Parser,
    
}

fn main() {

}