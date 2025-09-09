/// Lexer for the Alien programming language
/// Handles Unicode mathematical symbols and SCTT-specific tokens

use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Module,
    Where,
    Import,
    Open,
    Public,
    Private,
    Data,
    Match,
    With,
    Let,
    In,
    Do,
    Return,
    If,
    Then,
    Else,
    Auto,
    Proof,
    Qed,
    
    // Types
    Type,
    Smooth,
    Path,
    Dim,
    
    // Mathematical symbols
    RealNumbers,     // ℝ
    NaturalNumbers,  // ℕ
    BooleanType,     // 𝔹
    ComplexNumbers,  // ℂ
    
    // Operators
    Arrow,           // →
    DoubleArrow,     // ⇒
    Equiv,           // ≡
    Similar,         // ∼
    Times,           // ×
    Lambda,          // λ
    Forall,          // ∀
    Exists,          // ∃
    Partial,         // ∂
    Gradient,        // ∇
    Integral,        // ∫
    PathIntegral,    // ∮
    
    // Cubical operators
    Meet,            // ∧
    Join,            // ∨
    Neg,             // ~
    Comp,            // comp
    Coe,             // coe
    HCom,            // hcom
    
    // Path operations
    PathLambda,      // <i>
    PathApp,         // @
    PathCompose,     // ∙
    
    // Identifiers and literals
    Identifier(String),
    Number(f64),
    StringLit(String),
    
    // Dimension literals
    DimZero,         // i=0
    DimOne,          // i=1
    DimVar(String),  // i, j, k
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LeftAngle,       // <
    RightAngle,      // >
    
    // Punctuation
    Colon,
    Semicolon,
    Comma,
    Dot,
    DoubleDot,       // ..
    Equals,
    DoubleEquals,    // ==
    PlusSign,
    MinusSign,
    Star,
    Slash,
    Percent,
    Pipe,            // |
    MapTo,           // ↦
    
    // Special
    Underscore,
    At,              // @
    Hash,            // #
    Dollar,          // $
    Question,        // ?
    
    // End of file
    Eof,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_char: Option<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().peekable(),
            current_char: None,
            position: 0,
            line: 1,
            column: 0,
        };
        lexer.advance();
        lexer
    }
    
    fn advance(&mut self) {
        self.current_char = self.input.next();
        self.position += 1;
        
        if let Some(ch) = self.current_char {
            if ch == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
    }
    
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        if self.current_char == Some('-') && self.peek() == Some(&'-') {
            // Single line comment
            while self.current_char.is_some() && self.current_char != Some('\n') {
                self.advance();
            }
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' || ch == '\'' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        ident
    }
    
    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();
        
        // Integer part
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        // Decimal part
        if self.current_char == Some('.') && self.peek().map_or(false, |c| c.is_ascii_digit()) {
            num_str.push('.');
            self.advance();
            
            while let Some(ch) = self.current_char {
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        // Scientific notation
        if self.current_char == Some('e') || self.current_char == Some('E') {
            num_str.push('e');
            self.advance();
            
            if self.current_char == Some('+') || self.current_char == Some('-') {
                num_str.push(self.current_char.unwrap());
                self.advance();
            }
            
            while let Some(ch) = self.current_char {
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }
        
        num_str.parse().unwrap_or(0.0)
    }
    
    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.advance(); // Skip opening quote
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // Skip closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escape) = self.current_char {
                    match escape {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        _ => {
                            string.push('\\');
                            string.push(escape);
                        }
                    }
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }
        
        string
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        while self.current_char == Some('-') && self.peek() == Some(&'-') {
            self.skip_comment();
            self.skip_whitespace();
        }
        
        match self.current_char {
            None => Token::Eof,
            
            Some('(') => {
                self.advance();
                Token::LeftParen
            }
            Some(')') => {
                self.advance();
                Token::RightParen
            }
            Some('[') => {
                self.advance();
                Token::LeftBracket
            }
            Some(']') => {
                self.advance();
                Token::RightBracket
            }
            Some('{') => {
                self.advance();
                Token::LeftBrace
            }
            Some('}') => {
                self.advance();
                Token::RightBrace
            }
            Some('<') => {
                self.advance();
                Token::LeftAngle
            }
            Some('>') => {
                self.advance();
                Token::RightAngle
            }
            Some(':') => {
                self.advance();
                Token::Colon
            }
            Some(';') => {
                self.advance();
                Token::Semicolon
            }
            Some(',') => {
                self.advance();
                Token::Comma
            }
            Some('.') => {
                self.advance();
                if self.current_char == Some('.') {
                    self.advance();
                    Token::DoubleDot
                } else {
                    Token::Dot
                }
            }
            Some('=') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::DoubleEquals
                } else if self.current_char == Some('>') {
                    self.advance();
                    Token::DoubleArrow
                } else {
                    Token::Equals
                }
            }
            Some('+') => {
                self.advance();
                Token::PlusSign
            }
            Some('-') => {
                self.advance();
                if self.current_char == Some('>') {
                    self.advance();
                    Token::Arrow
                } else {
                    Token::MinusSign
                }
            }
            Some('*') => {
                self.advance();
                Token::Star
            }
            Some('/') => {
                self.advance();
                Token::Slash
            }
            Some('%') => {
                self.advance();
                Token::Percent
            }
            Some('|') => {
                self.advance();
                Token::Pipe
            }
            Some('_') => {
                self.advance();
                Token::Underscore
            }
            Some('@') => {
                self.advance();
                Token::At
            }
            Some('#') => {
                self.advance();
                Token::Hash
            }
            Some('$') => {
                self.advance();
                Token::Dollar
            }
            Some('?') => {
                self.advance();
                Token::Question
            }
            Some('~') => {
                self.advance();
                Token::Neg
            }
            Some('"') => {
                Token::StringLit(self.read_string())
            }
            
            // Mathematical Unicode symbols
            Some('ℝ') => {
                self.advance();
                Token::RealNumbers
            }
            Some('ℕ') => {
                self.advance();
                Token::NaturalNumbers
            }
            Some('𝔹') => {
                self.advance();
                Token::BooleanType
            }
            Some('ℂ') => {
                self.advance();
                Token::ComplexNumbers
            }
            Some('→') => {
                self.advance();
                Token::Arrow
            }
            Some('⇒') => {
                self.advance();
                Token::DoubleArrow
            }
            Some('≡') => {
                self.advance();
                Token::Equiv
            }
            Some('∼') => {
                self.advance();
                Token::Similar
            }
            Some('×') => {
                self.advance();
                Token::Times
            }
            Some('λ') => {
                self.advance();
                Token::Lambda
            }
            Some('∀') => {
                self.advance();
                Token::Forall
            }
            Some('∃') => {
                self.advance();
                Token::Exists
            }
            Some('∂') => {
                self.advance();
                Token::Partial
            }
            Some('∇') => {
                self.advance();
                Token::Gradient
            }
            Some('∫') => {
                self.advance();
                Token::Integral
            }
            Some('∮') => {
                self.advance();
                Token::PathIntegral
            }
            Some('∧') => {
                self.advance();
                Token::Meet
            }
            Some('∨') => {
                self.advance();
                Token::Join
            }
            Some('∙') => {
                self.advance();
                Token::PathCompose
            }
            Some('↦') => {
                self.advance();
                Token::MapTo
            }
            
            Some(ch) if ch.is_ascii_digit() => {
                Token::Number(self.read_number())
            }
            
            Some(ch) if ch.is_alphabetic() => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "module" => Token::Module,
                    "where" => Token::Where,
                    "import" => Token::Import,
                    "open" => Token::Open,
                    "public" => Token::Public,
                    "private" => Token::Private,
                    "data" => Token::Data,
                    "match" => Token::Match,
                    "with" => Token::With,
                    "let" => Token::Let,
                    "in" => Token::In,
                    "do" => Token::Do,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "then" => Token::Then,
                    "else" => Token::Else,
                    "auto" => Token::Auto,
                    "proof" => Token::Proof,
                    "qed" => Token::Qed,
                    "Type" => Token::Type,
                    "Smooth" => Token::Smooth,
                    "Path" => Token::Path,
                    "dim" => Token::Dim,
                    "comp" => Token::Comp,
                    "coe" => Token::Coe,
                    "hcom" => Token::HCom,
                    _ => Token::Identifier(ident),
                }
            }
            
            Some(ch) => {
                self.advance();
                Token::Identifier(ch.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("module Test where");
        assert_eq!(lexer.next_token(), Token::Module);
        assert_eq!(lexer.next_token(), Token::Identifier("Test".to_string()));
        assert_eq!(lexer.next_token(), Token::Where);
        assert_eq!(lexer.next_token(), Token::Eof);
    }
    
    #[test]
    fn test_mathematical_symbols() {
        let mut lexer = Lexer::new("ℝ → ℝ");
        assert_eq!(lexer.next_token(), Token::RealNumbers);
        assert_eq!(lexer.next_token(), Token::Arrow);
        assert_eq!(lexer.next_token(), Token::RealNumbers);
    }
    
    #[test]
    fn test_path_syntax() {
        let mut lexer = Lexer::new("<i> x @ i");
        assert_eq!(lexer.next_token(), Token::LeftAngle);
        assert_eq!(lexer.next_token(), Token::Identifier("i".to_string()));
        assert_eq!(lexer.next_token(), Token::RightAngle);
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token(), Token::At);
        assert_eq!(lexer.next_token(), Token::Identifier("i".to_string()));
    }
}