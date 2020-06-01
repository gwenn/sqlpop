# SQLPOP

[![Build Status](https://github.com/gwenn/sqlpop/workflows/Rust/badge.svg)](https://github.com/gwenn/sqlpop/actions)
[![Latest Version](https://img.shields.io/crates/v/sqlpop.svg)](https://crates.io/crates/sqlpop)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/sqlpop)
[![dependency status](https://deps.rs/repo/github/gwenn/sqlpop/status.svg)](https://deps.rs/repo/github/gwenn/sqlpop)

SQL parser (as understood by SQLite)

* [LALRPOP and custom lexer](https://github.com/nikomatsakis/lalrpop/issues/39)
* [SQLite tokenizer](http://www.sqlite.org/src/artifact?ci=trunk&filename=src/tokenize.c)
* [SQLite parser](http://www.sqlite.org/src/artifact?ci=trunk&filename=src/parse.y)
* [SQLite syntax diagrams](https://www.sqlite.org/syntaxdiagrams.html)
* [SQLite syntax diagram data](http://www.sqlite.org/docsrc/doc/tip/art/syntax/bubble-generator-data.tcl?mimetype=text/plain)

Currenly, only the lexer is complete and tested.
The parser is almost complete (see [LALRPOP issues](https://github.com/nikomatsakis/lalrpop/issues/156)) but does not support recent SQLite features (like [UPSERT](https://sqlite.org/lang_UPSERT.html), [RENAME COLUMN](https://sqlite.org/lang_altertable.html), [Window functions](https://sqlite.org/windowfunctions.html)).
