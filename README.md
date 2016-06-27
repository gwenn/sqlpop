# SQLPOP

SQL parser (as understood by SQLite)

* [LARLPOP and custom lexer](https://github.com/nikomatsakis/lalrpop/issues/39)
* [SQLite tokenizer](http://www.sqlite.org/cgi/src/artifact/32aeca12f0d57a5c)
* [SQLite parser](http://www.sqlite.org/cgi/src/artifact/d7bff41d460f2df9)
* [SQLite BNF grammar](http://www.sqlite.org/docsrc/doc/trunk/art/syntax/all-bnf.html)

Currenly, only the lexer is complete and tested.
The parser is not complete (`expr` parsing is missing) and not tested...