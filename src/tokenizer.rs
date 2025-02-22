
macro_rules! return_token {
    ($token_type:expr, $position:expr) => {
        return Some(Token {
            token_type: $token_type,
            lexeme: None,
            position: $position,
        })
    };
    ($token_type:expr, $position:expr, $lexeme:expr) => {
        return Some(Token {
            token_type: $token_type,
            lexeme: Some($lexeme),
            position: $position,
        })
    };
}

#[derive(Debug, PartialEq)]
pub enum TokenType {

    // Keywords
    Assert,
    Var,
    Const,
    If,
    Else,
    Then,
    While,
    Do,
    For,
    In,
    Function,
    Return,
    Break,
    Continue,
    End,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Ampersand,
    Pipe,
    Tilde,
    Bang,
    Question,

    // Comparison
    Equal,
    DoubleEqual,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Not,
    And,
    Or,

    // Delimiters
    Comma,
    Colon,
    Semicolon,
    Dot,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Literals
    Identifier,
    Integer,
    Float,
    String,
    True,
    False,
    Null,

    EndOfInput,

    Unknown,

}

#[derive(Debug, PartialEq)]
pub struct TokenPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
    pub position: TokenPosition,
}

#[derive(Debug)]
pub struct Tokenizer {
    pub chars: Vec<char>,
    pub cursor: usize,
    pub line: usize,
    pub column: usize,
}

impl Tokenizer {

    pub fn tokenize(input: String) -> Vec<Token> {
        let mut tokenizer = Tokenizer {
            chars: input.chars().collect(),
            cursor: 0,
            line: 1,
            column: 1,
        };

        let mut tokens = Vec::new();

        loop {
            match tokenizer.next() {
                Some(token) => tokens.push(token),
                None => break,
            }
        }

        tokens
    }

    fn next(&mut self) -> Option<Token> {

        // Skip whitespace
        while self.cursor < self.chars.len() && self.chars[self.cursor].is_whitespace() {
            if self.chars[self.cursor] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.cursor += 1;
        }

        if self.cursor >= self.chars.len() {
            return None;
        }

        let position = TokenPosition {
            line: self.line,
            column: self.column,
        };

        // handle identifiers and keywords
        if self.chars[self.cursor].is_alphabetic() {
            let start = self.cursor;
            while self.cursor < self.chars.len() && self.chars[self.cursor].is_alphanumeric()  {
                self.cursor += 1;
                self.column += 1;
            }
            let lexeme = self.chars[start..self.cursor].iter().collect::<String>();
            match lexeme.as_str() {
                "assert" => return_token!(TokenType::Assert, position),
                "var" => return_token!(TokenType::Var, position),
                "const" => return_token!(TokenType::Const, position),
                "if" => return_token!(TokenType::If, position),
                "else" => return_token!(TokenType::Else, position),
                "then" => return_token!(TokenType::Then, position),
                "while" => return_token!(TokenType::While, position),
                "do" => return_token!(TokenType::Do, position),
                "for" => return_token!(TokenType::For, position),
                "in" => return_token!(TokenType::In, position),
                "function" => return_token!(TokenType::Function, position),
                "return" => return_token!(TokenType::Return, position),
                "break" => return_token!(TokenType::Break, position),
                "continue" => return_token!(TokenType::Continue, position),
                "end" => return_token!(TokenType::End, position),
                "and" => return_token!(TokenType::And, position),
                "or" => return_token!(TokenType::Or, position),
                "not" => return_token!(TokenType::Not, position),
                "true" => return_token!(TokenType::True, position),
                "false" => return_token!(TokenType::False, position),
                "null" => return_token!(TokenType::Null, position),
                _ => return_token!(TokenType::Identifier, position, lexeme),
            }
        }

        // handle numbers
        if self.cursor < self.chars.len() && self.chars[self.cursor].is_numeric() {
            let start = self.cursor;
            while self.cursor < self.chars.len() && self.chars[self.cursor].is_numeric() {
                self.cursor += 1;
                self.column += 1;
            }
            if self.cursor < self.chars.len() &&  self.chars[self.cursor] == '.' {
                self.cursor += 1;
                while self.cursor < self.chars.len() && self.chars[self.cursor].is_numeric() {
                    self.cursor += 1;
                }
                let lexeme = self.chars[start..self.cursor].iter().collect::<String>();
                return_token!(TokenType::Float, position, lexeme);
            } else {
                let lexeme = self.chars[start..self.cursor].iter().collect::<String>();
                return_token!(TokenType::Integer, position, lexeme);
            }
        }

        // handle strings
        if self.chars[self.cursor] == '"' {
            self.cursor += 1;
            let start = self.cursor;
            while self.chars[self.cursor] != '"' {
                self.cursor += 1;
                self.column += 1;
            }
            let lexeme = self.chars[start..self.cursor].iter().collect::<String>();
            self.cursor += 1;
            return_token!(TokenType::String, position, lexeme);
        }

        // handle single character tokens
        match self.chars[self.cursor] {
            '+' => {
                self.cursor += 1;
                return_token!(TokenType::Plus, position)
            }
            '-' => {
                self.cursor += 1;
                return_token!(TokenType::Minus, position)
            }
            '*' => {
                self.cursor += 1;
                return_token!(TokenType::Star, position)
            }
            '/' => {
                self.cursor += 1;
                return_token!(TokenType::Slash, position)
            }
            '%' => {
                self.cursor += 1;
                return_token!(TokenType::Percent, position)
            }
            '^' => {
                self.cursor += 1;
                return_token!(TokenType::Caret, position)
            }
            '&' => {
                self.cursor += 1;
                return_token!(TokenType::Ampersand, position)
            }
            '|' => {
                self.cursor += 1;
                return_token!(TokenType::Pipe, position)
            }
            '!' => {
                self.cursor += 1;
                if self.chars[self.cursor] == '=' {
                    self.cursor += 1;
                    return_token!(TokenType::NotEqual, position)
                } else {
                    return_token!(TokenType::Bang, position)
                }
            }
            '=' => {
                self.cursor += 1;
                if self.chars[self.cursor] == '=' {
                    self.cursor += 1;
                    return_token!(TokenType::DoubleEqual, position)
                } else {
                    return_token!(TokenType::Equal, position)
                }
            }
            '<' => {
                self.cursor += 1;
                if self.chars[self.cursor] == '=' {
                    self.cursor += 1;
                    return_token!(TokenType::LessThanOrEqual, position)
                } else {
                    return_token!(TokenType::LessThan, position)
                }
            }
            '>' => {
                self.cursor += 1;
                if self.chars[self.cursor] == '=' {
                    self.cursor += 1;
                    return_token!(TokenType::GreaterThanOrEqual, position)
                } else {
                    return_token!(TokenType::GreaterThan, position)
                }
            }
            ',' => {
                self.cursor += 1;
                return_token!(TokenType::Comma, position)
            }
            ':' => {
                self.cursor += 1;
                return_token!(TokenType::Colon, position)
            }
            ';' => {
                self.cursor += 1;
                return_token!(TokenType::Semicolon, position)
            }
            '.' => {
                self.cursor += 1;
                return_token!(TokenType::Dot, position)
            },
            '(' => {
                self.cursor += 1;
                return_token!(TokenType::LeftParen, position)
            },
            ')' => {
                self.cursor += 1;
                return_token!(TokenType::RightParen, position)
            },
            '{' => {
                self.cursor += 1;
                return_token!(TokenType::LeftBrace, position)
            },
            '}' => {
                self.cursor += 1;
                return_token!(TokenType::RightBrace, position)
            },
            '[' => {
                self.cursor += 1;
                return_token!(TokenType::LeftBracket, position)
            },
            ']' => {
                self.cursor += 1;
                return_token!(TokenType::RightBracket, position)
            },
            _ => panic!("Unknown token: {}", self.chars[self.cursor]),
        }

    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tokenizer() {
        let input = String::from("var x = 5 + 3.14");
        let tokens = vec![
            Token {
                token_type: TokenType::Var,
                lexeme: None,
                position: TokenPosition { line: 1, column: 1 },
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: Option::from(String::from("x")),
                position: TokenPosition { line: 1, column: 5 },
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: None,
                position: TokenPosition { line: 1, column: 7 },
            },
            Token {
                token_type: TokenType::Integer,
                lexeme: Option::from(String::from("5")),
                position: TokenPosition { line: 1, column: 8 },
            },
            Token {
                token_type: TokenType::Plus,
                lexeme: None,
                position: TokenPosition { line: 1, column: 10 },
            },
            Token {
                token_type: TokenType::Float,
                lexeme: Option::from(String::from("3.14")),
                position: TokenPosition { line: 1, column: 11 },
            },
        ];
        assert_eq!(Tokenizer::tokenize(input), tokens);
    }

    #[test]
    fn test_tokenizer_with_string() {
        let input = String::from(r#"var x = "Hello, World!""#);
        let tokens = vec![
            Token {
                token_type: TokenType::Var,
                lexeme: None,
                position: TokenPosition { line: 1, column: 1 },
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: Option::from(String::from("x")),
                position: TokenPosition { line: 1, column: 5 },
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: None,
                position: TokenPosition { line: 1, column: 7 },
            },
            Token {
                token_type: TokenType::String,
                lexeme: Option::from(String::from("Hello, World!")),
                position: TokenPosition { line: 1, column: 8 },
            },
        ];
        assert_eq!(Tokenizer::tokenize(input), tokens);
    }
}