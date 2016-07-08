# SQLPOP

[![Build Status](https://travis-ci.org/gwenn/sqlpop.svg?branch=master)](https://travis-ci.org/gwenn/sqlpop)

SQL parser (as understood by SQLite)

* [LARLPOP and custom lexer](https://github.com/nikomatsakis/lalrpop/issues/39)
* [SQLite tokenizer](http://www.sqlite.org/cgi/src/artifact/32aeca12f0d57a5c)
* [SQLite parser](http://www.sqlite.org/cgi/src/artifact/d7bff41d460f2df9)
* [SQLite BNF grammar](http://www.sqlite.org/docsrc/doc/trunk/art/syntax/all-bnf.html)
* [SQLite syntax diagram data](http://www.sqlite.org/docsrc/doc/tip/art/syntax/bubble-generator-data.tcl?mimetype=text/plain)

Currenly, only the lexer is complete and tested.
The parser is not complete (some expressions parsing are missing) and no AST is generated...

Currently, I can't make progress anymore because nor my laptop, nor Travis can build the project (see https://github.com/nikomatsakis/lalrpop/issues/65).
