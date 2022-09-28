mod error;
pub mod token;

pub use error::LexerError;

use token::{Token, TokenKeyword, TokenLiteral, TokenPrimitive, TokenType};

use crate::span::SpanPoint;
use crate::Result;

pub fn lex_into_tokens(content: String) -> Result<Vec<Token>> {
    let mut chars = content.chars().peekable();
    let mut tokens = vec![];

    let mut line = 1;
    let mut column = 0;

    let cursor_newline = |line: &mut usize, column: &mut usize| {
        *line += 1;
        *column = 0;
    };

    let mut is_in_string_template = false;

    while let Some(c) = chars.next() {
        column += 1;

        match c {
            // New line
            '\n' => {
                // New lines should be ignored
                // tokens.push(Token::new(TokenType::NewLine, c, (line, column)));

                cursor_newline(&mut line, &mut column);
                continue;
            }

            // Whitespace
            _ if c.is_whitespace() =>
                // Whitespace should be ignored
                {} // tokens.push(Token::new(TokenType::Whitespace, c, (line, column))),

            // Block comment
            '/' if chars.peek() == Some(&'*') => {
                // let from = SpanPoint { line, column };

                chars.next();
                column += 1;

                // let mut buffer = String::from("/*");

                let mut nested_block_count = 0;

                while let Some(c) = chars.next() {
                    column += 1;

                    if c == '\n' {
                        cursor_newline(&mut line, &mut column);
                        // buffer.push('\n');
                    } else if c == '/' && chars.peek() == Some(&'*') {
                        chars.next();
                        column += 1;
                        nested_block_count += 1;
                        // buffer.push_str("/*");
                    } else if c == '*' && chars.peek() == Some(&'/') {
                        chars.next();
                        column += 1;
                        // buffer.push_str("*/");

                        if nested_block_count > 0 {
                            nested_block_count -= 1;
                        } else {
                            break;
                        }
                    } else {
                        // buffer.push(c);
                    }
                }

                // Block comments should be ignored
                // tokens.push(Token::new(TokenType::BlockComment, buffer, (from, (line, column))));
            }

            // Line comment
            '/' if chars.peek() == Some(&'/') => {
                // let from = SpanPoint { line, column };

                chars.next();
                column += 1;

                // let mut buffer = String::from("//");

                // Note: Literal::LineComment's representation doesn't include the new-line char
                while chars.peek() != Some(&'\n') {
                    // let c = chars.next().unwrap();
                    chars.next();
                    column += 1;
                    // buffer.push(c);
                }

                // Line comments should be ignored
                // tokens.push(Token::new(TokenType::LineComment, buffer, (from, (line, column))));
            }

            // Plain string or start of string-template
            '"' => {
                let from = SpanPoint { line, column };

                // Note: Need to account for both plain and template strings
                // Also accounting for escaped chars w/ backslash
                let mut buffer = String::new();

                let mut is_template = false;
                let mut is_escaped = false;

                while let Some(c) = chars.next() {
                    column += 1;

                    if c == '\\' {
                        buffer.push(c);
                        is_escaped = true;
                    } else {
                        if !is_escaped {
                            match c {
                                '"' => break,
                                '{' => {
                                    is_template = true;
                                    break;
                                }
                                _ => {}
                            }
                        }

                        if c == '\n' {
                            cursor_newline(&mut line, &mut column);
                        }

                        buffer.push(c);
                        is_escaped = false;
                    }
                }

                if is_template {
                    tokens.push(Token::new(
                        TokenType::TemplateStringStart,
                        buffer,
                        (from, (line, column)),
                    ));
                    is_in_string_template = true;
                } else {
                    tokens.push(Token::new(
                        TokenType::Literal(TokenLiteral::String),
                        buffer,
                        (from, (line, column)),
                    ));
                }
            }

            // Middle or end of string-template
            '}' if is_in_string_template => {
                let from = SpanPoint { line, column };

                let mut buffer = String::new();

                let mut is_template_middle = false;
                let mut is_escaped = false;

                while let Some(c) = chars.next() {
                    column += 1;

                    if c == '\\' {
                        buffer.push(c);
                        is_escaped = true;
                    } else {
                        if !is_escaped {
                            match c {
                                '"' => break,
                                '{' => {
                                    is_template_middle = true;
                                    break;
                                }
                                _ => {}
                            }
                        }

                        if c == '\n' {
                            cursor_newline(&mut line, &mut column);
                        }

                        buffer.push(c);
                        is_escaped = false;
                    }
                }

                if is_template_middle {
                    tokens.push(Token::new(
                        TokenType::TemplateStringMiddle,
                        buffer,
                        (from, (line, column)),
                    ));
                } else {
                    is_in_string_template = false;
                    tokens.push(Token::new(
                        TokenType::TemplateStringEnd,
                        buffer,
                        (from, (line, column)),
                    ));
                }
            }

            '\'' => {
                let from = SpanPoint { line, column };

                let mut buffer = String::new();

                let mut is_escaped = true;

                while let Some(c) = chars.next() {
                    column += 1;

                    if c == '\\' {
                        buffer.push(c);
                        is_escaped = true;
                    } else {
                        if !is_escaped {
                            match c {
                                '\'' => break,
                                _ => {}
                            }
                        }

                        if c == '\n' {
                            cursor_newline(&mut line, &mut column);
                        }

                        buffer.push(c);
                        is_escaped = false;
                    }
                }

                tokens.push(Token::new(
                    TokenType::Literal(TokenLiteral::Char),
                    buffer,
                    (from, (line, column)),
                ));
            }

            // Numbers
            _ if c.is_numeric() => {
                let from = SpanPoint { line, column };

                let mut buffer = c.to_string();

                let mut allow_period = true;
                let mut allow_e = true;
                let mut prev_was_period = false;

                while let Some(&peek) = chars.peek() {
                    if !peek.is_numeric() {
                        match (peek, allow_period, allow_e) {
                            ('.', false, _) => break,
                            ('e', _, false) => break,

                            ('.', _, _) => {
                                allow_period = false;
                                prev_was_period = true;
                            }
                            ('e', _, _) if !prev_was_period => {
                                allow_period = false;
                                allow_e = false;
                            }

                            _ => break,
                        }
                    } else {
                        if prev_was_period {
                            buffer.push('.');
                        }

                        prev_was_period = false;
                    }

                    if !prev_was_period {
                        buffer.push(peek);
                    }

                    chars.next();

                    if !prev_was_period {
                        column += 1;
                    }
                }

                tokens.push(Token::new(
                    TokenType::Literal(TokenLiteral::Number),
                    buffer,
                    (from, (line, column)),
                ));
            }

            // TypeName
            _ if c.is_uppercase() => {
                let from = SpanPoint { line, column };

                let mut buffer = c.to_string();

                while let Some(&peek) = chars.peek() {
                    if !peek.is_alphanumeric() {
                        break;
                    }

                    buffer.push(peek);
                    chars.next();
                    column += 1;
                }

                tokens.push(Token::new(TokenType::TypeName, buffer, (from, (line, column))));
            }

            // Identifier or keyword
            _ if c.is_alphabetic() || c == '_' => {
                let from = SpanPoint { line, column };

                let mut buffer = c.to_string();

                while let Some(&peek) = chars.peek() {
                    if !peek.is_alphanumeric() && peek != '_' {
                        break;
                    }

                    buffer.push(peek);
                    chars.next();
                    column += 1;
                }

                let token_type = match buffer.as_str() {
                    "true" => TokenType::Literal(TokenLiteral::Bool(true)),
                    "false" => TokenType::Literal(TokenLiteral::Bool(false)),
                    "some" => TokenType::Literal(TokenLiteral::Option { is_some: true }),
                    "none" => TokenType::Literal(TokenLiteral::Option { is_some: false }),
                    "ok" => TokenType::Literal(TokenLiteral::Result { is_ok: true }),
                    "err" => TokenType::Literal(TokenLiteral::Result { is_ok: false }),

                    "use" => TokenType::Keyword(TokenKeyword::Use),
                    "from" => TokenType::Keyword(TokenKeyword::From),
                    "pub" => TokenType::Keyword(TokenKeyword::Pub),
                    "static" => TokenType::Keyword(TokenKeyword::Static),
                    "fn" => TokenType::Keyword(TokenKeyword::Fn),
                    "let" => TokenType::Keyword(TokenKeyword::Let),
                    "const" => TokenType::Keyword(TokenKeyword::Const),
                    "struct" => TokenType::Keyword(TokenKeyword::Struct),
                    "contract" => TokenType::Keyword(TokenKeyword::Contract),
                    "enum" => TokenType::Keyword(TokenKeyword::Enum),
                    "type" => TokenType::Keyword(TokenKeyword::Type),
                    "alias" => TokenType::Keyword(TokenKeyword::Alias),
                    "errors" => TokenType::Keyword(TokenKeyword::Errors),
                    "self" => TokenType::Keyword(TokenKeyword::Self_),
                    "construct" => TokenType::Keyword(TokenKeyword::Construct),
                    "impl" => TokenType::Keyword(TokenKeyword::Impl),
                    "return" => TokenType::Keyword(TokenKeyword::Return),
                    "yield" => TokenType::Keyword(TokenKeyword::Yield),
                    "if" => TokenType::Keyword(TokenKeyword::If),
                    "not" => TokenType::Keyword(TokenKeyword::Not),
                    "or" => TokenType::Keyword(TokenKeyword::Or),
                    "and" => TokenType::Keyword(TokenKeyword::And),
                    "else" => TokenType::Keyword(TokenKeyword::Else),
                    "match" => TokenType::Keyword(TokenKeyword::Match),
                    "is" => TokenType::Keyword(TokenKeyword::Is),
                    "loop" => TokenType::Keyword(TokenKeyword::Loop),
                    "while" => TokenType::Keyword(TokenKeyword::While),
                    "for" => TokenType::Keyword(TokenKeyword::For),
                    "in" => TokenType::Keyword(TokenKeyword::In),
                    "safe" => TokenType::Keyword(TokenKeyword::Safe),
                    "async" => TokenType::Keyword(TokenKeyword::Async),
                    "await" => TokenType::Keyword(TokenKeyword::Await),

                    "bool" => TokenType::Primitive(TokenPrimitive::Bool),
                    "bit" => TokenType::Primitive(TokenPrimitive::Bit),
                    "byte" => TokenType::Primitive(TokenPrimitive::Byte),
                    "uint" => TokenType::Primitive(TokenPrimitive::Uint),
                    "uint8" => TokenType::Primitive(TokenPrimitive::Uint8),
                    "uint16" => TokenType::Primitive(TokenPrimitive::Uint16),
                    "uint32" => TokenType::Primitive(TokenPrimitive::Uint32),
                    "uint64" => TokenType::Primitive(TokenPrimitive::Uint64),
                    "uint128" => TokenType::Primitive(TokenPrimitive::Uint128),
                    "biguint" => TokenType::Primitive(TokenPrimitive::BigUint),
                    "int" => TokenType::Primitive(TokenPrimitive::Int),
                    "int8" => TokenType::Primitive(TokenPrimitive::Int8),
                    "int16" => TokenType::Primitive(TokenPrimitive::Int16),
                    "int32" => TokenType::Primitive(TokenPrimitive::Int32),
                    "int64" => TokenType::Primitive(TokenPrimitive::Int64),
                    "int128" => TokenType::Primitive(TokenPrimitive::Int128),
                    "bigint" => TokenType::Primitive(TokenPrimitive::BigInt),
                    "float" => TokenType::Primitive(TokenPrimitive::Float),
                    "float32" => TokenType::Primitive(TokenPrimitive::Float32),
                    "float64" => TokenType::Primitive(TokenPrimitive::Float64),
                    "bignum" => TokenType::Primitive(TokenPrimitive::BigNum),
                    "char" => TokenType::Primitive(TokenPrimitive::Char),
                    "string" => TokenType::Primitive(TokenPrimitive::String),

                    _ => TokenType::Identifier,
                };

                tokens.push(Token::new(token_type, buffer, (from, (line, column))));
            }

            '{' => tokens.push(Token::new(TokenType::OpenBrace, c, (line, column))),
            '}' => tokens.push(Token::new(TokenType::CloseBrace, c, (line, column))),

            '[' => tokens.push(Token::new(TokenType::OpenBracket, c, (line, column))),
            ']' => tokens.push(Token::new(TokenType::CloseBracket, c, (line, column))),

            '(' => tokens.push(Token::new(TokenType::OpenParenthesis, c, (line, column))),
            ')' => tokens.push(Token::new(TokenType::CloseParenthesis, c, (line, column))),

            ',' => tokens.push(Token::new(TokenType::Comma, c, (line, column))),

            ';' => tokens.push(Token::new(TokenType::Semicolon, c, (line, column))),

            // Colon or DoubleColon
            ':' => match chars.peek() {
                Some(&':') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::DoubleColon,
                        "::",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::Colon, c, (line, column))),
            },

            '~' => tokens.push(Token::new(TokenType::Tilde, c, (line, column))),

            // QuestionMark, DoubleQuestionMark, or QuestionMarkPeriod
            '?' => match chars.peek() {
                Some(&'?') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::DoubleQuestionMark,
                        "??",
                        (from, (line, column)),
                    ));
                }
                Some(&'.') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::QuestionMarkPeriod,
                        "?.",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::QuestionMark, c, (line, column))),
            },

            // Period, DoublePeriod, or DoublePeriodEquals
            '.' => match chars.peek() {
                Some(&'.') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();

                    match chars.peek() {
                        Some(&'=') => {
                            column += 1;
                            chars.next();
                            tokens.push(Token::new(
                                TokenType::DoublePeriodEquals,
                                "..=",
                                (from, (line, column)),
                            ));
                        }
                        _ => tokens.push(Token::new(
                            TokenType::DoublePeriod,
                            "..",
                            (from, (line, column)),
                        )),
                    }
                }
                _ => tokens.push(Token::new(TokenType::Period, c, (line, column))),
            },

            // Equals, DoubleEquals, or FatArrow
            '=' => match chars.peek() {
                Some(&'=') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::DoubleEquals,
                        "==",
                        (from, (line, column)),
                    ));
                }
                Some(&'>') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::FatArrow,
                        "=>",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::Equals, c, (line, column))),
            },

            // ExclamationMark or NotEquals
            '!' => match chars.peek() {
                Some(&'=') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::NotEquals,
                        "!=",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::ExclamationMark, c, (line, column))),
            },

            // Ampersand or DoubleAmpersand
            '&' => match chars.peek() {
                Some(&'&') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::DoubleAmpersand,
                        "&&",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::Ampersand, c, (line, column))),
            },

            // Pipe or DoublePipe
            '|' => match chars.peek() {
                Some(&'|') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::DoublePipe,
                        "||",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::Pipe, c, (line, column))),
            },

            // LessThan or LessThanEquals
            '<' => match chars.peek() {
                Some(&'=') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::LessThanEquals,
                        "<=",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::LessThan, c, (line, column))),
            },

            // GreaterThan or GreaterThanEquals
            '>' => match chars.peek() {
                Some(&'=') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::GreaterThanEquals,
                        ">=",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::GreaterThan, c, (line, column))),
            },

            // Plus or PlusEquals
            '+' => match chars.peek() {
                Some(&'=') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::PlusEquals,
                        "+=",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::Plus, c, (line, column))),
            },

            // Minus, MinusEquals, or SkinnyArrow
            '-' => match chars.peek() {
                Some(&'=') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::MinusEquals,
                        "-=",
                        (from, (line, column)),
                    ));
                }
                Some(&'>') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::SkinnyArrow,
                        "->",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::Minus, c, (line, column))),
            },

            // Asterisk, AsteriskEquals, or AsteriskAt
            '*' => match chars.peek() {
                Some(&'=') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::AsteriskEquals,
                        "*=",
                        (from, (line, column)),
                    ));
                }
                Some(&'@') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::AsteriskAt,
                        "*@",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::Asterisk, c, (line, column))),
            },

            // ForwardSlash or ForwardSlashEquals
            '/' => match chars.peek() {
                Some(&'=') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::ForwardSlashEquals,
                        "/=",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::ForwardSlash, c, (line, column))),
            },

            // Percent, PercentEquals, PercentOpenBrace, or PercentOpenBracket
            '%' => match chars.peek() {
                Some(&'=') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::PercentEquals,
                        "%=",
                        (from, (line, column)),
                    ));
                }
                Some(&'{') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::PercentOpenBrace,
                        "%{",
                        (from, (line, column)),
                    ));
                }
                Some(&'[') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::PercentOpenBracket,
                        "%[",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::Percent, c, (line, column))),
            },

            '$' => tokens.push(Token::new(TokenType::Dollar, c, (line, column))),

            // Caret or CaretEquals
            '^' => match chars.peek() {
                Some(&'=') => {
                    let from = SpanPoint { line, column };
                    column += 1;
                    chars.next();
                    tokens.push(Token::new(
                        TokenType::CaretEquals,
                        "^=",
                        (from, (line, column)),
                    ));
                }
                _ => tokens.push(Token::new(TokenType::Caret, c, (line, column))),
            },

            '@' => tokens.push(Token::new(TokenType::At, c, (line, column))),

            '#' => tokens.push(Token::new(TokenType::Pound, c, (line, column))),

            _ => todo!("Unexpected char: '{c}'"),
        }
    }

    return Ok(tokens);
}

