//! A SQL parser.
//! Adapted from [SQLite parser](http://www.sqlite.org/src/artifact?ci=trunk&filename=src/parse.y)

use lalrpop_util;
use tok;
use ast::Cmd;

#[allow(dead_code)]
mod lrsql;

#[cfg(test)]
mod test;

pub type ParseError<'input> = lalrpop_util::ParseError<usize, tok::Tok<'input>, tok::Error>;

pub fn parse_sql<'input>(input: &'input str) -> Result<Vec<Option<Cmd>>, ParseError<'input>> {
    let tokenizer = tok::Tokenizer::new(input, 0);
    let sql = try!(lrsql::parse_CmdList(input, tokenizer));

    Ok(sql)
}
