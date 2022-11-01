use super::TokenKeyword;
use super::TokenLiteral;
use super::TokenPrimitive;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // NewLine,
    // Whitespace,

    // BlockComment,
    // LineComment,
    Literal(TokenLiteral),
    Keyword(TokenKeyword),
    Primitive(TokenPrimitive),

    Identifier,

    TemplateStringStart,
    TemplateStringMiddle,
    TemplateStringEnd,

    OpenBrace,  // {
    CloseBrace, // }

    OpenBracket,  // [
    CloseBracket, // ]

    OpenParenthesis,  // (
    CloseParenthesis, // )

    Comma, // ,

    Semicolon, // ;

    Colon,       // :
    DoubleColon, // ::

    Tilde,             // ~

    QuestionMark,       // ?
    DoubleQuestionMark, // ??

    Period,                   // .
    DoublePeriod,             // ..

    Equals,       // =
    DoubleEquals, // ==
    FatArrow,     // =>

    ExclamationMark, // !
    NotEquals,       // !=

    Ampersand,       // &
    DoubleAmpersand, // &&

    Pipe,            // |
    DoublePipe,      // ||
    PipeGreaterThan, // |>

    LessThan,       // <
    LessThanEquals, // <=

    GreaterThan,       // >
    GreaterThanEquals, // >=

    Plus,       // +
    PlusEquals, // +=

    Minus,       // -
    MinusEquals, // -=
    SkinnyArrow, // ->

    Asterisk,       // *
    AsteriskEquals, // *=
    AsteriskAt,     // *@

    ForwardSlash,       // /
    ForwardSlashEquals, // /=

    Percent,            // %
    PercentEquals,      // %=
    PercentOpenBrace,   // %{
    PercentOpenBracket, // %[

    Dollar, // $

    Caret,       // ^
    CaretEquals, // ^=

    At, // @

    Pound, // #
}
