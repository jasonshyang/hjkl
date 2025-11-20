use ratatui::style::Style;

use crate::tui::theme::*;

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

const TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64", "bool", "char", "str", "String", "Vec", "Option", "Result", "Box", "HashMap", "HashSet",
];

const PUNCTUATION: &[char] = &[
    '{', '}', '(', ')', '[', ']', '<', '>', ';', ',', '.', ':', '=', '+', '-', '*', '&', '|', '!',
    '?',
];

/// Types of tokens for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Keyword,
    Type,
    String,
    Number,
    Comment,
    Punctuation,
    Normal,
}

impl TokenType {
    pub fn style(self) -> Style {
        match self {
            TokenType::Keyword => Style::default().fg(SYNTAX_KEYWORD_COLOR),
            TokenType::Type => Style::default().fg(SYNTAX_TYPE_COLOR),
            TokenType::String => Style::default().fg(SYNTAX_STRING_COLOR),
            TokenType::Number => Style::default().fg(SYNTAX_NUMBER_COLOR),
            TokenType::Comment => Style::default().fg(SYNTAX_COMMENT_COLOR),
            TokenType::Punctuation => Style::default().fg(SYNTAX_PUNCTUATION_COLOR),
            TokenType::Normal => Style::default().fg(SYNTAX_NORMAL_COLOR),
        }
    }
}

/// Simple token for syntax highlighting
#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
}

/// Tokenize a line of Rust code for syntax highlighting
pub fn tokenize_line(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable();
    let mut current = String::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Handle comments
            '/' if chars.clone().nth(1) == Some('/') => {
                if !current.is_empty() {
                    tokens.push(classify_token(current.clone()));
                    current.clear();
                }
                // Consume rest of line as comment
                let comment: String = chars.collect();
                tokens.push(Token {
                    text: comment,
                    token_type: TokenType::Comment,
                });
                break;
            }
            // Handle strings
            '"' => {
                if !current.is_empty() {
                    tokens.push(classify_token(current.clone()));
                    current.clear();
                }
                let mut string = String::from('"');
                chars.next(); // consume opening quote
                let mut escaped = false;

                for c in chars.by_ref() {
                    string.push(c);
                    if escaped {
                        escaped = false;
                    } else if c == '\\' {
                        escaped = true;
                    } else if c == '"' {
                        break;
                    }
                }

                tokens.push(Token {
                    text: string,
                    token_type: TokenType::String,
                });
            }
            // Handle char literals
            '\'' => {
                if !current.is_empty() {
                    tokens.push(classify_token(current.clone()));
                    current.clear();
                }
                let mut char_lit = String::from('\'');
                chars.next(); // consume opening quote

                if let Some(c) = chars.next() {
                    char_lit.push(c);
                    if c == '\\'
                        && let Some(escaped) = chars.next()
                    {
                        char_lit.push(escaped);
                    }
                }
                if let Some(c) = chars.next() {
                    char_lit.push(c);
                }

                tokens.push(Token {
                    text: char_lit,
                    token_type: TokenType::String,
                });
            }
            // Handle punctuation
            c if PUNCTUATION.contains(&c) => {
                if !current.is_empty() {
                    tokens.push(classify_token(current.clone()));
                    current.clear();
                }
                tokens.push(Token {
                    text: ch.to_string(),
                    token_type: TokenType::Punctuation,
                });
                chars.next();
            }
            // Handle whitespace
            ' ' | '\t' => {
                if !current.is_empty() {
                    tokens.push(classify_token(current.clone()));
                    current.clear();
                }
                tokens.push(Token {
                    text: ch.to_string(),
                    token_type: TokenType::Normal,
                });
                chars.next();
            }
            _ => {
                current.push(ch);
                chars.next();
            }
        }
    }

    if !current.is_empty() {
        tokens.push(classify_token(current));
    }

    tokens
}

/// Classifies a token based on its text
fn classify_token(text: String) -> Token {
    let token_type = if KEYWORDS.contains(&text.as_str()) {
        TokenType::Keyword
    } else if TYPES.contains(&text.as_str()) {
        TokenType::Type
    } else if text.chars().all(|c| c.is_ascii_digit() || c == '_') {
        TokenType::Number
    } else {
        TokenType::Normal
    };

    Token { text, token_type }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let line = "fn main() {";
        let tokens = tokenize_line(line);

        assert_eq!(tokens[0].text, "fn");
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[2].text, "main");
        assert_eq!(tokens[3].text, "(");
        assert_eq!(tokens[3].token_type, TokenType::Punctuation);
    }

    #[test]
    fn test_tokenize_with_string() {
        let line = r#"let x = "hello";"#;
        let tokens = tokenize_line(line);

        assert!(tokens.iter().any(|t| t.token_type == TokenType::Keyword));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::String));
    }
}
