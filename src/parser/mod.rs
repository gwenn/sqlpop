//! A SQL parser.
//! Adapted from [SQLite parser](http://www.sqlite.org/src/artifact?ci=trunk&filename=src/parse.y)

use crate::ast::Cmd;
use crate::tok;
use lalrpop_util;

lalrpop_mod!(lrsql, "/parser/lrsql.rs");

#[cfg(test)]
mod test;

pub type ParseError<'input> = lalrpop_util::ParseError<usize, tok::Tok<'input>, tok::Error>;

pub fn parse_sql<'input>(input: &'input str) -> Result<Vec<Option<Cmd>>, ParseError<'input>> {
    use self::lrsql::CmdListParser;
    let tokenizer = tok::Tokenizer::new(input, 0);
    let sql = CmdListParser::new().parse(input, tokenizer)?;

    Ok(sql)
}
