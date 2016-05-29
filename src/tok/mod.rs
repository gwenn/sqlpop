//! A SQL tokenizer.
//! Adapted from [LALRPOP own Tokenizer](https://github.com/nikomatsakis/lalrpop/blob/master/lalrpop/src/tok/mod.rs)
//! and [SQLite tokenizer](http://www.sqlite.org/cgi/src/artifact/32aeca12f0d57a5c)

use std::str::CharIndices;
use std::ascii::AsciiExt;

use self::ErrorCode::*;
use self::Tok::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub location: usize,
    pub code: ErrorCode,
    pub line: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnrecognizedToken,
    UnterminatedLiteral,
    UnterminatedBracket,
    UnterminatedBlockComment,
    BadVariableName,
    BadNumber,
    ExpectedEqualsSign,
    MalformedBlobLiteral,
    MalformedHexInteger,
}

fn error<T>(c: ErrorCode, l: usize, t: &str) -> Result<T, Error> {
    let line = t[..l].chars().filter(|c| *c == '\n').count() + 1;
    Err(Error {
        location: l,
        code: c,
        line: line,
    })
}

pub struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    shift: usize,
}

pub type Spanned<T> = (usize, T, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    // Keywords:
    Abort,
    Action,
    Add,
    After,
    All,
    Alter,
    Analyze,
    And,
    // Any,
    As,
    Asc,
    Attach,
    Autoincr,
    Before,
    Begin,
    Between,
    By,
    Cascade,
    Case,
    Cast,
    Check,
    Collate,
    ColumnKw,
    Commit,
    Conflict,
    Constraint,
    Create,
    CtimeKw, // (&'input str)
    Database,
    Default,
    Deferrable,
    Deferred,
    Delete,
    Desc,
    Detach,
    Distinct,
    Drop,
    Each,
    Else,
    End,
    Escape,
    Except,
    Exclusive,
    Exists,
    Explain,
    Fail,
    For,
    Foreign,
    From,
    // Function,
    Group,
    Having,
    If,
    Ignore,
    Immediate,
    In,
    Index,
    Indexed,
    Initially,
    Insert,
    Instead,
    Intersect,
    Into,
    Is,
    // IsNot,
    IsNull,
    // Join,
    JoinKw, // (&'input str)
    Key,
    LikeKw, // (&'input str)
    Limit,
    Match,
    No,
    Not,
    NotNull,
    Null,
    Of,
    Offset,
    On,
    Or,
    Order,
    Plan,
    Pragma,
    Primary,
    Query,
    Raise,
    Recursive,
    References,
    Reindex,
    Release,
    Rename,
    Replace,
    Restrict,
    Rollback,
    Row,
    Savepoint,
    Select,
    Set,
    Table,
    Temp,
    Then,
    To,
    Transaction,
    Trigger,
    Union,
    Unique,
    Update,
    Using,
    Vacuum,
    Values,
    View,
    Virtual,
    When,
    Where,
    With,
    Without,

    // Identifiers:
    StringLiteral(&'input str),
    Id(&'input str),
    Variable(&'input str),

    // Values:
    Blob(&'input str),
    Integer(&'input str),
    Float(&'input str),

    // Symbols:
    BitAnd,
    BitNot,
    BitOr,
    Comma,
    Concat,
    Dot,
    Equals,
    GreaterThan,
    GreaterEquals,
    LeftParen,
    LeftShift,
    LessEquals,
    LessThan,
    Minus,
    NotEquals,
    Reminder,
    Plus,
    RightParen,
    RightShift,
    Semi,
    Slash,
    Star,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const KEYWORDS: &'static [(&'static str, Tok<'static>)] = &[
    ("ABORT", Abort),
    ("ACTION", Action),
    ("ADD", Add),
    ("AFTER", After),
    ("ALL", All),
    ("ALTER", Alter),
    ("ANALYZE", Analyze),
    ("AND", And),
    ("AS", As),
    ("ASC", Asc),
    ("ATTACH", Attach),
    ("AUTOINCREMENT", Autoincr),
    ("BEFORE", Before),
    ("BEGIN", Begin),
    ("BETWEEN", Between),
    ("BY", By),
    ("CASCADE", Cascade),
    ("CASE", Case),
    ("CAST", Cast),
    ("CHECK", Check),
    ("COLLATE", Collate),
    ("COLUMN", ColumnKw),
    ("COMMIT", Commit),
    ("CONFLICT", Conflict),
    ("CONSTRAINT", Constraint),
    ("CREATE", Create),
    ("CROSS", JoinKw), // FIXME
    ("CURRENT_DATE", CtimeKw), // FIXME
    ("CURRENT_TIME", CtimeKw), // FIXME
    ("CURRENT_TIMESTAMP", CtimeKw), /* FIXME */
    ("DATABASE", Database),
    ("DEFAULT", Default),
    ("DEFERRABLE", Deferrable),
    ("DEFERRED", Deferred),
    ("DELETE", Delete),
    ("DESC", Desc),
    ("DETACH", Detach),
    ("DISTINCT", Distinct),
    ("DROP", Drop),
    ("EACH", Each),
    ("ELSE", Else),
    ("END", End),
    ("ESCAPE", Escape),
    ("EXCEPT", Except),
    ("EXCLUSIVE", Exclusive),
    ("EXISTS", Exists),
    ("EXPLAIN", Explain),
    ("FAIL", Fail),
    ("FOR", For),
    ("FOREIGN", Foreign),
    ("FROM", From),
    ("FULL", JoinKw), // FIXME
    ("GLOB", LikeKw), // FIXME
    ("GROUP", Group),
    ("HAVING", Having),
    ("IF", If),
    ("IGNORE", Ignore),
    ("IMMEDIATE", Immediate),
    ("IN", In),
    ("INDEX", Index),
    ("INDEXED", Indexed),
    ("INITIALLY", Initially),
    ("INNER", JoinKw), // FIXME
    ("INSERT", Insert),
    ("INSTEAD", Instead),
    ("INTERSECT", Intersect),
    ("INTO", Into),
    ("IS", Is),
    ("ISNULL", IsNull),
    ("JOIN", JoinKw),
    ("KEY", Key),
    ("LEFT", JoinKw), // FIXME
    ("LIKE", LikeKw), // FIXME
    ("LIMIT", Limit),
    ("MATCH", Match),
    ("NATURAL", JoinKw), // FIXME
    ("NO", No),
    ("NOT", Not),
    ("NOTNULL", NotNull),
    ("NULL", Null),
    ("OF", Of),
    ("OFFSET", Offset),
    ("ON", On),
    ("OR", Or),
    ("ORDER", Order),
    ("OUTER", JoinKw), // FIXME
    ("PLAN", Plan),
    ("PRAGMA", Pragma),
    ("PRIMARY", Primary),
    ("QUERY", Query),
    ("RAISE", Raise),
    ("RECURSIVE", Recursive),
    ("REFERENCES", References),
    ("REGEXP", LikeKw), // FIXME
    ("REINDEX", Reindex),
    ("RELEASE", Release),
    ("RENAME", Rename),
    ("REPLACE", Replace),
    ("RESTRICT", Restrict),
    ("RIGHT", JoinKw), // FIXME
    ("ROLLBACK", Rollback),
    ("ROW", Row),
    ("SAVEPOINT", Savepoint),
    ("SELECT", Select),
    ("SET", Set),
    ("TABLE", Table),
    ("TEMP", Temp),
    ("TEMPORARY", Temp),
    ("THEN", Then),
    ("TO", To),
    ("TRANSACTION", Transaction),
    ("TRIGGER", Trigger),
    ("UNION", Union),
    ("UNIQUE", Unique),
    ("UPDATE", Update),
    ("USING", Using),
    ("VACUUM", Vacuum),
    ("VALUES", Values),
    ("VIEW", View),
    ("VIRTUAL", Virtual),
    ("WHEN", When),
    ("WHERE", Where),
    ("WITH", With),
    ("WITHOUT", Without)
    ];

impl<'input> Tokenizer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Tokenizer<'input> {
        let mut t = Tokenizer {
            text: text,
            chars: text.char_indices(),
            lookahead: None,
            shift: shift,
        };
        t.bump();
        t
    }

    fn next_unshifted(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        loop {
            return match self.lookahead {
                Some((_, c)) if c.is_whitespace() => {
                    self.bump();
                    continue;
                }
                Some((idx0, '-')) => {
                    match self.bump() {
                        Some((_, '-')) => {
                            self.take_until(|c| c == '\n');
                            continue;
                        }
                        _ => Some(Ok((idx0, Minus, idx0 + 1))),
                    }
                }
                Some((idx0, '(')) => {
                    self.bump();
                    Some(Ok((idx0, LeftParen, idx0 + 1)))
                }
                Some((idx0, ')')) => {
                    self.bump();
                    Some(Ok((idx0, RightParen, idx0 + 1)))
                }
                Some((idx0, ';')) => {
                    self.bump();
                    Some(Ok((idx0, Semi, idx0 + 1)))
                }
                Some((idx0, '+')) => {
                    self.bump();
                    Some(Ok((idx0, Plus, idx0 + 1)))
                }
                Some((idx0, '*')) => {
                    self.bump();
                    Some(Ok((idx0, Star, idx0 + 1)))
                }
                Some((idx0, '/')) => {
                    match self.bump() {
                        Some((_, '*')) => {
                            match self.block_comment(idx0) {
                                Ok(_) => {
                                    continue;
                                }
                                Err(e) => Some(Err(e)),
                            }
                        }
                        _ => Some(Ok((idx0, Slash, idx0 + 1))),
                    }
                }
                Some((idx0, '%')) => {
                    self.bump();
                    Some(Ok((idx0, Reminder, idx0 + 1)))
                }
                Some((idx0, '=')) => {
                    match self.bump() {
                        Some((idx1, '=')) => {
                            self.bump();
                            Some(Ok((idx0, Equals, idx1 + 1)))
                        }
                        _ => Some(Ok((idx0, Equals, idx0 + 1))),
                    }
                }
                Some((idx0, '<')) => {
                    match self.bump() {
                        Some((idx1, '=')) => {
                            self.bump();
                            Some(Ok((idx0, LessEquals, idx1 + 1)))
                        }
                        Some((idx1, '>')) => {
                            self.bump();
                            Some(Ok((idx0, NotEquals, idx1 + 1)))
                        }
                        Some((idx1, '<')) => {
                            self.bump();
                            Some(Ok((idx0, LeftShift, idx1 + 1)))
                        }
                        _ => Some(Ok((idx0, LessThan, idx0 + 1))),
                    }
                }
                Some((idx0, '>')) => {
                    match self.bump() {
                        Some((idx1, '=')) => {
                            self.bump();
                            Some(Ok((idx0, GreaterEquals, idx1 + 1)))
                        }
                        Some((idx1, '>')) => {
                            self.bump();
                            Some(Ok((idx0, RightShift, idx1 + 1)))
                        }
                        _ => Some(Ok((idx0, GreaterThan, idx0 + 1))),
                    }
                }
                Some((idx0, '!')) => {
                    match self.bump() {
                        Some((idx1, '=')) => {
                            self.bump();
                            Some(Ok((idx0, NotEquals, idx1 + 1)))
                        }
                        _ => Some(error(ExpectedEqualsSign, idx0, self.text)),
                    }
                }
                Some((idx0, '|')) => {
                    match self.bump() {
                        Some((idx1, '|')) => {
                            self.bump();
                            Some(Ok((idx0, Concat, idx1 + 1)))
                        }
                        _ => Some(Ok((idx0, BitOr, idx0 + 1))),
                    }
                }
                Some((idx0, ',')) => {
                    self.bump();
                    Some(Ok((idx0, Comma, idx0 + 1)))
                }
                Some((idx0, '&')) => {
                    self.bump();
                    Some(Ok((idx0, BitAnd, idx0 + 1)))
                }
                Some((idx0, '~')) => {
                    self.bump();
                    Some(Ok((idx0, BitNot, idx0 + 1)))
                }
                Some((idx0, c)) if c == '`' || c == '\'' || c == '"' => Some(self.literal(idx0, c)),
                Some((idx0, '.')) => {
                    match self.bump() {
                        Some((_, c)) if c.is_digit(10) => Some(self.fractional_part(idx0)),
                        _ => Some(Ok((idx0, Dot, idx0 + 1))),
                    }
                }
                Some((idx0, c)) if c.is_digit(10) => Some(self.number(idx0, c)),
                Some((idx0, '[')) => Some(self.bracket(idx0)),
                Some((idx0, '?')) => {
                    self.bump();
                    let num = match self.take_while(|c| c.is_digit(10)) {
                        Some((end, _)) if end > idx0 + 1 => {
                            (idx0, Variable(&self.text[idx0..end]), end)
                        } // '?' is included as part of the name
                        _ => (idx0, Variable(""), idx0 + 1),
                    };
                    Some(Ok(num))
                }
                Some((idx0, c)) if c == '$' || c == '@' || c == '#' || c == ':' => {
                    self.bump();
                    // '$' is included as part of the name
                    let (start, name, end) = self.word(idx0);
                    if name.len() == 1 {
                        Some(error(BadVariableName, idx0, self.text))
                    } else {
                        Some(Ok((start, Variable(name), end)))
                    }
                }
                Some((idx0, c)) if is_identifier_start(c) => {
                    if c == 'x' || c == 'X' {
                        match self.bump() {
                            Some((idx1, '\'')) => Some(self.blob_literal(idx1)),
                            _ => Some(self.identifierish(idx0)),
                        }
                    } else {
                        Some(self.identifierish(idx0))
                    }
                }
                Some((idx, _)) => Some(error(UnrecognizedToken, idx, self.text)),
                None => None,
            };
        }
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    fn literal(&mut self, idx0: usize, delim: char) -> Result<Spanned<Tok<'input>>, Error> {
        let mut t;
        loop {
            t = self.bump();
            match t {
                Some((_, c)) if c == delim => {
                    if let Some((_, nc)) = self.bump() {
                        if nc == delim {
                            continue;
                        }
                    }
                    break;
                }
                Some((_, _)) => {
                    continue;
                }
                None => {
                    break;
                }
            }
        }
        match t {
            Some((idx1, c)) if c == delim => {
                let text = &self.text[idx0 + 1..idx1];
                let tok = if delim == '\'' {
                    StringLiteral(text)
                } else {
                    Id(text) // empty Id (ie "") is OK
                };
                Ok((idx0, tok, idx1 + 1))
            }
            _ => error(UnterminatedLiteral, idx0, self.text),
        }
    }

    fn blob_literal(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        let mut n = 0;
        loop {
            match self.bump() {
                Some((_, c)) if c.is_digit(16) => {
                    n += 1;
                }
                Some((idx1, '\'')) if n % 2 == 0 => {
                    self.bump(); // consume the `'`
                    return Ok((idx0, Blob(&self.text[idx0 + 1..idx1]), idx1 + 1));
                }
                _ => {
                    return error(MalformedBlobLiteral, idx0, self.text);
                }
            }
        }
    }

    // Real
    fn fractional_part(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        match self.take_while(|c| c.is_digit(10)) {
            Some((end, c)) => {
                if c == 'e' || c == 'E' {
                    self.exponential_part(idx0)
                } else {
                    Ok((idx0, Float(&self.text[idx0..end]), end))
                }
            }
            None => Ok((idx0, Float(&self.text[idx0..]), self.text.len())),
        }
    }

    // Real
    fn exponential_part(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        match self.bump() {
            Some((_, '+')) | Some((_, '-')) => {
                self.bump();
            }
            _ => {}
        };

        match self.take_while_1(|c| c.is_digit(10)) {
            Some(end) => Ok((idx0, Float(&self.text[idx0..end]), end)),
            None => error(BadNumber, idx0, self.text),
        }
    }

    // Decimal or Hexadecimal Integer or Real
    fn number(&mut self, idx0: usize, digit: char) -> Result<Spanned<Tok<'input>>, Error> {
        if digit == '0' {
            match self.bump() {
                Some((_, 'x')) | Some((_, 'X')) => {
                    self.bump();
                    return self.hex_integer(idx0);
                }
                _ => {}
            }
        }
        match self.take_while(|c| c.is_digit(10)) {
            Some((end, c)) => {
                if c == '.' {
                    self.bump();
                    self.fractional_part(idx0)
                } else if c == 'e' || c == 'E' {
                    self.exponential_part(idx0)
                } else {
                    Ok((idx0, Integer(&self.text[idx0..end]), end))
                }
            }
            None => Ok((idx0, Integer(&self.text[idx0..]), self.text.len())),
        }
    }

    fn hex_integer(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        // Must not be empty (Ox is invalid)
        match self.take_while_1(|c| c.is_digit(16)) {
            Some(end) => Ok((idx0, Integer(&self.text[idx0..end]), end)),
            None => error(MalformedHexInteger, idx0, self.text),
        }
    }

    fn identifierish(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        let (start, word, end) = self.word(idx0);
        // search for a keyword first; if none are found, this is an Id
        let tok = KEYWORDS.iter()
            .filter(|&&(w, _)| w.eq_ignore_ascii_case(word))
            .map(|&(_, ref t)| t.clone())
            .next()
            .unwrap_or_else(|| Id(word));
        Ok((start, tok, end))
    }

    fn word(&mut self, idx0: usize) -> Spanned<&'input str> {
        match self.take_while(is_identifier_continue) {
            Some((end, _)) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        }
    }

    fn bracket(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        match self.take_until(|c| c == ']') {
            Some(idx1) => {
                self.bump(); // consume the ']'
                let id: &'input str = &self.text[idx0 + 1..idx1]; // do not include the '['/']' in the str
                Ok((idx0, Id(id), idx1 + 1)) // empty Id (ie []) is OK
            }
            _ => error(UnterminatedBracket, idx0, self.text),
        }
    }

    fn block_comment(&mut self, idx0: usize) -> Result<(), Error> {
        let mut pc = '\0';
        loop {
            match self.bump() {
                Some((_, '/')) if pc == '*' => {
                    self.bump(); // consume the '/'
                    return Ok(());
                }
                Some((_, c)) => {
                    pc = c;
                }
                None => {
                    return error(UnterminatedBlockComment, idx0, self.text);
                }
            }
        }
    }

    // Returns `None` when `keep_going` does not succeed on at least once.
    fn take_while_1<F>(&mut self, mut keep_going: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        let mut none = true;
        loop {
            match self.lookahead {
                None => {
                    return None;
                }
                Some((idx1, c)) => {
                    if !keep_going(c) {
                        if none {
                            return None;
                        } else {
                            return Some(idx1);
                        }
                    } else {
                        self.bump();
                        none = false;
                    }
                }
            }
        }
    }


    fn take_while<F>(&mut self, mut keep_going: F) -> Option<(usize, char)>
        where F: FnMut(char) -> bool
    {
        loop {
            match self.lookahead {
                None => {
                    return None;
                }
                Some((_, c)) => {
                    if !keep_going(c) {
                        return self.lookahead;
                    } else {
                        self.bump();
                    }
                }
            }
        }
    }

    fn take_until<F>(&mut self, mut terminate: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        loop {
            match self.lookahead {
                None => {
                    return None;
                }
                Some((idx1, c)) => {
                    if terminate(c) {
                        return Some(idx1);
                    } else {
                        self.bump();
                    }
                }
            }
        }
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        match self.next_unshifted() {
            None => None,
            Some(Ok((l, t, r))) => Some(Ok((l + self.shift, t, r + self.shift))),
            Some(Err(Error { location, code, line })) => {
                Some(Err(Error {
                    location: location + self.shift,
                    code: code,
                    line: line,
                }))
            }
        }
    }
}


fn is_identifier_start(c: char) -> bool {
    (c >= 'A' && c <= 'Z') || c == '_' || (c >= 'a' && c <= 'z') || c > '\x7F'
}

fn is_identifier_continue(c: char) -> bool {
    c == '$' || (c >= '0' && c <= '9') || (c >= 'A' && c <= 'Z') || c == '_' ||
    (c >= 'a' && c <= 'z') || c > '\x7F'
}

#[cfg(test)]
mod test {
    use super::{Error, ErrorCode, Tokenizer, Tok};

    fn assert_tokens(expected_tokens: Vec<Tok>, input: &str) {
        let lexer = Tokenizer::new(input, 0);
        let actual_tokens: Vec<Tok> = lexer.into_iter()
            .map(|r| {
                let (_, t, _) = r.unwrap();
                t
            })
            .collect();
        assert_eq!(expected_tokens, actual_tokens);
    }

    fn assert_error(expected_tokens: Vec<Result<Tok, Error>>, input: &str) {
        let lexer = Tokenizer::new(input, 0);
        let mut expected_tokens_it = expected_tokens.into_iter();
        for actual in lexer {
            let expected = expected_tokens_it.next().unwrap();
            match (actual, expected) {
                (Ok((_, actual_token, _)), Ok(expected_token)) => {
                    assert_eq!(expected_token, actual_token);
                }
                (Err(Error { code: actual_code, .. }), Err(Error { code: expected_code, .. })) => {
                    assert_eq!(expected_code, actual_code);
                }
                (actual, expected) => panic!("expected: {:?}, got: {:?}", expected, actual),
            }
        }
        let missing_token = expected_tokens_it.next();
        assert_eq!(None, missing_token);
    }

    #[test]
    fn test_insert() {
        let expected_tokens = vec![Tok::Insert,
                                   Tok::Into,
                                   Tok::Id("t3"),
                                   Tok::Values,
                                   Tok::LeftParen,
                                   Tok::StringLiteral("r c"),
                                   Tok::Comma,
                                   Tok::StringLiteral(""),
                                   Tok::RightParen,
                                   Tok::Semi];
        assert_tokens(expected_tokens, "INSERT INTO t3 VALUES( 'r c', '');");
        let expected_tokens = vec![Tok::Insert,
                                   Tok::Into,
                                   Tok::Id("email"),
                                   Tok::LeftParen,
                                   Tok::Id("from"),
                                   Tok::Comma,
                                   Tok::Id("to"),
                                   Tok::Comma,
                                   Tok::Id("subject"),
                                   Tok::Comma,
                                   Tok::Id("body"),
                                   Tok::RightParen,
                                   Tok::Values,
                                   Tok::LeftParen,
                                   Tok::StringLiteral("..."),
                                   Tok::Comma,
                                   Tok::StringLiteral("..."),
                                   Tok::Comma,
                                   Tok::StringLiteral("..."),
                                   Tok::Comma,
                                   Tok::StringLiteral("..."),
                                   Tok::RightParen];
        assert_tokens(expected_tokens,
                      "INSERT INTO email([from],[to],subject,body) VALUES('...', '...', '...', \
                       '...')");
    }

    #[test]
    fn test_comment() {
        let expected_tokens = vec![];
        assert_tokens(expected_tokens, "-- ......");
        let expected_tokens = vec![];
        assert_tokens(expected_tokens, " -- ......\n");
    }

    #[test]
    fn test_minus() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Minus, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a-b");
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Minus, Tok::Minus, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a - -b");
    }

    #[test]
    fn test_plus() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Plus, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a+b");
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Plus, Tok::Plus, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a + +b");
    }

    #[test]
    fn test_star() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Star, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a*b");
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Star, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a * b");
    }

    #[test]
    fn test_block_comment() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a")];
        assert_tokens(expected_tokens, "SELECT a/*b***/");
    }

    #[test]
    fn test_slash() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Slash, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a/b");
    }

    #[test]
    fn test_reminder() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Reminder, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a%b");
    }

    #[test]
    fn test_equals() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Equals, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a=b");
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Equals, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a==b");
    }

    #[test]
    fn test_less() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::LessThan, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a<b");
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::LessEquals, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a<=b");
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::NotEquals, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a<>b");
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::LeftShift, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a<<b");
    }

    #[test]
    fn test_greater() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::GreaterThan, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a>b");
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::GreaterEquals, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a>=b");
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::RightShift, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a>>b");
    }

    #[test]
    fn test_exclamation_mark() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::NotEquals, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a!=b");
    }

    #[test]
    fn test_exclamation_mark_alone() {
        let expected_tokens = vec![Ok(Tok::Select),
                                   Ok(Tok::Id("a")),
                                   super::error(ErrorCode::ExpectedEqualsSign, 0, ""),
                                   Ok(Tok::Id("b"))];
        assert_error(expected_tokens, "SELECT a!b");
    }

    #[test]
    fn test_pipe() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::BitOr, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a|b");
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Concat, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a||b");
    }

    #[test]
    fn test_comma() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::Comma, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a,b");
    }

    #[test]
    fn test_ampersand() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::BitAnd, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a&b");
    }

    #[test]
    fn test_tilde() {
        let expected_tokens = vec![Tok::Select, Tok::Id("a"), Tok::BitNot, Tok::Id("b")];
        assert_tokens(expected_tokens, "SELECT a~b");
    }

    #[test]
    fn test_literal() {
        let expected_tokens = vec![Tok::Select, Tok::StringLiteral("")];
        assert_tokens(expected_tokens, "SELECT ''");
        let expected_tokens = vec![Tok::Select, Tok::Id("")];
        assert_tokens(expected_tokens, "SELECT \"\"");
        let expected_tokens = vec![Tok::Select, Tok::Id("")];
        assert_tokens(expected_tokens, "SELECT ``");

        let expected_tokens = vec![Tok::Select, Tok::StringLiteral("hel''lo")];
        assert_tokens(expected_tokens, "SELECT 'hel''lo'");
        let expected_tokens = vec![Tok::Select, Tok::Id("hel\"\"lo")];
        assert_tokens(expected_tokens, "SELECT \"hel\"\"lo\"");
        let expected_tokens = vec![Tok::Select, Tok::Id("hel``lo")];
        assert_tokens(expected_tokens, "SELECT `hel``lo`");
    }
}
