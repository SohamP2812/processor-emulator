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

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: Vec::new(),
        }
    }
}

struct Assembler {
    parser: Parser,
    output: Vec<u8>
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            parser: Parser::new(),
            output: Vec::new(),
        }
    }
}

fn main() {

}