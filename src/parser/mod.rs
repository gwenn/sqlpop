use lalrpop_util;
use tok;

#[allow(dead_code)]
mod lrsql;

#[cfg(test)]
mod test;

pub type ParseError<'input> = lalrpop_util::ParseError<usize, tok::Tok<'input>, tok::Error>;

pub fn parse_sql<'input>(input: &'input str) -> Result<(), ParseError<'input>> {
    let tokenizer = tok::Tokenizer::new(input, 0);
    let sql = try!(lrsql::parse_Cmd(input, tokenizer));

    Ok(sql)
}
