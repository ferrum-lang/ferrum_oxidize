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

    Identifier, // Starts with lowercase letter
    TypeName, // Starts with uppercase letter

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

    Tilde, // ~

    QuestionMark,       // ?
    DoubleQuestionMark, // ??
    QuestionMarkPeriod, // ?.

    Period,             // .
    DoublePeriod,       // ..
    DoublePeriodEquals, // ..=

    Equals,       // =
    DoubleEquals, // ==
    FatArrow,     // =>

    ExclamationMark, // !
    NotEquals,       // !=

    Ampersand,       // &
    AmpersandMut,    // &mut
    DoubleAmpersand, // &&

    Pipe,       // |
    DoublePipe, // ||

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
