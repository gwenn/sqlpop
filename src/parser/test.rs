use super::parse_sql;

#[test]
fn test_begin() {
    parse_sql("BEGIN").unwrap();
    parse_sql("BEGIN DEFERRED").unwrap();
    parse_sql("BEGIN IMMEDIATE").unwrap();
    parse_sql("BEGIN EXCLUSIVE").unwrap();

    parse_sql("BEGIN TRANSACTION").unwrap();
    parse_sql("BEGIN EXCLUSIVE TRANSACTION").unwrap();

    parse_sql("BEGIN TRANSACTION tx").unwrap();
    parse_sql("BEGIN EXCLUSIVE TRANSACTION `tx`").unwrap();
    parse_sql("BEGIN EXCLUSIVE TRANSACTION \"tx\"").unwrap();
    parse_sql("BEGIN EXCLUSIVE TRANSACTION [tx]").unwrap();

    assert!(parse_sql("BEGIN tx").is_err(),
            "error expected when transaction name is specified without `TRANSACTION` keyword \
             preceding");
}

#[test]
fn test_commit() {
    parse_sql("COMMIT").unwrap();
    parse_sql("END").unwrap();

    parse_sql("COMMIT TRANSACTION").unwrap();
    parse_sql("END TRANSACTION").unwrap();

    parse_sql("COMMIT TRANSACTION tx").unwrap();
    parse_sql("END TRANSACTION tx").unwrap();
    parse_sql("COMMIT TRANSACTION `tx`").unwrap();
    parse_sql("COMMIT TRANSACTION \"tx\"").unwrap();
    parse_sql("COMMIT TRANSACTION [tx]").unwrap();

    assert!(parse_sql("COMMIT tx").is_err(),
            "error expected when transaction name is specified without `TRANSACTION` keyword \
             preceding");
}

#[test]
fn test_rollback() {
    parse_sql("ROLLBACK").unwrap();

    parse_sql("ROLLBACK TRANSACTION").unwrap();

    parse_sql("ROLLBACK TRANSACTION tx").unwrap();
    parse_sql("ROLLBACK TRANSACTION `tx`").unwrap();
    parse_sql("ROLLBACK TRANSACTION \"tx\"").unwrap();
    parse_sql("ROLLBACK TRANSACTION [tx]").unwrap();

    assert!(parse_sql("ROLLBACK tx").is_err(),
            "error expected when transaction name is specified without `TRANSACTION` keyword \
             preceding");
}

#[test]
fn test_savepoint() {
    parse_sql("SAVEPOINT sp").unwrap();

    assert!(parse_sql("SAVEPOINT").is_err(),
            "error expected when no savepoint name is specified");
}

#[test]
fn test_release() {
    parse_sql("RELEASE sp").unwrap();
    parse_sql("RELEASE SAVEPOINT sp").unwrap();

    parse_sql("ROLLBACK TO SAVEPOINT sp").unwrap();
    parse_sql("ROLLBACK TO sp").unwrap();

    assert!(parse_sql("RELEASE").is_err(),
            "error expected when no savepoint name is specified");
    assert!(parse_sql("ROLLBACK TO").is_err(),
            "error expected when no savepoint name is specified");
}

#[test]
fn test_create_table() {
    parse_sql("CREATE TABLE test (col)").unwrap();
    parse_sql("CREATE TABLE main.test (col)").unwrap();
    parse_sql("CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL)").unwrap();

    parse_sql("CREATE TABLE test (id INTERGER NOT NULL, PRIMARY KEY (id))").unwrap();
    parse_sql("CREATE TABLE test AS SELECT 1").unwrap();

    parse_sql("CREATE TEMP TABLE test (col)").unwrap();
    parse_sql("CREATE TABLE IF NOT EXISTS test (col)").unwrap();

    assert!(parse_sql("CREATE TABLE test").is_err(),
            "error expected when no column list is specified");
    assert!(parse_sql("CREATE TABLE test ()").is_err(),
            "error expected when no column is specified");
    assert!(parse_sql("CREATE TABLE test (PRIMARY KEY (id))").is_err(),
            "error expected when only table constraint is specified");
    assert!(parse_sql("CREATE TABLE test (col,)").is_err(),
            "error expected with trailing comma");
}

#[test]
fn test_column_definition() {
    parse_sql("CREATE TABLE test (id UNSIGNED BIG INT)").unwrap();
    parse_sql("CREATE TABLE test (id INT8)").unwrap();
    parse_sql("CREATE TABLE test (id CHARACTER(20))").unwrap();
    parse_sql("CREATE TABLE test (id VARYING CHARACTER(255))").unwrap();
    parse_sql("CREATE TABLE test (id DOUBLE PRECISION)").unwrap();
    parse_sql("CREATE TABLE test (id DECIMAL(10,5))").unwrap();
}

#[test]
fn test_column_constraints() {
    parse_sql("CREATE TABLE test (id CONSTRAINT not_null NOT NULL)").unwrap();
    parse_sql("CREATE TABLE test (id INTEGER PRIMARY KEY AUTOINCREMENT)").unwrap();
    parse_sql("CREATE TABLE test (id INTEGER PRIMARY KEY ON CONFLICT IGNORE)").unwrap();
    parse_sql("CREATE TABLE test (id UNIQUE)").unwrap();
    parse_sql("CREATE TABLE test (id CHECK (id > 0))").unwrap();
    parse_sql("CREATE TABLE test (id DEFAULT '')").unwrap();
    parse_sql("CREATE TABLE test (id COLLATE NOCASE)").unwrap();
    parse_sql("CREATE TABLE test (id REFERENCES fktable(id))").unwrap();
    parse_sql("CREATE TABLE test (id REFERENCES fktable(id) ON DELETE CASCADE)").unwrap();
}

#[test]
fn test_table_constraints() {
    parse_sql("CREATE TABLE test (id, CONSTRAINT pk PRIMARY KEY (id))")
        .expect("PK constraint supported");
    parse_sql("CREATE TABLE test (id, UNIQUE (id))").expect("UNIQUE constraint supported");
    parse_sql("CREATE TABLE test (id, CHECK (id > 0))").expect("CHECK constraint supported");
    parse_sql("CREATE TABLE test (id, FOREIGN KEY (id) REFERENCES fktable(id))")
        .expect("FK constaint with one column reference supported");
    parse_sql("CREATE TABLE test (id, FOREIGN KEY (id) REFERENCES fktable)")
        .expect("FK constaint with no column reference supported");
    parse_sql("CREATE TABLE test (id, FOREIGN KEY (id) REFERENCES fktable(id) DEFERRABLE \
               INITIALLY DEFERRED)")
            .expect("FK constraint with defer clause supported");
}

#[test]
fn test_drop_table() {
    parse_sql("DROP TABLE test").unwrap();
    parse_sql("DROP TABLE main.test").unwrap();

    parse_sql("DROP TABLE IF EXISTS test").unwrap();

    assert!(parse_sql("DROP TABLE").is_err(),
            "error expected when no table name is specified");
}

#[test]
fn test_create_view() {
    parse_sql("CREATE VIEW test AS SELECT 1").unwrap();
    parse_sql("CREATE VIEW test (id) AS SELECT 1").unwrap();
    parse_sql("CREATE VIEW main.test AS SELECT 1").unwrap();

    parse_sql("CREATE VIEW IF NOT EXISTS test AS SELECT 1").unwrap();

    assert!(parse_sql("CREATE VIEW AS SELECT 1").is_err(),
            "error expected when no view name is specified");
}

#[test]
fn test_drop_view() {
    parse_sql("DROP VIEW test").unwrap();
    parse_sql("DROP VIEW main.test").unwrap();

    parse_sql("DROP VIEW IF EXISTS test").unwrap();

    assert!(parse_sql("DROP VIEW").is_err(),
            "error expected when no view name is specified");
}

#[test]
fn test_select() {
    parse_sql("SELECT 1").unwrap();
    parse_sql("SELECT 1, 'test'").unwrap();

    parse_sql("SELECT * FROM test WHERE 1").unwrap();
    parse_sql("SELECT * FROM test WHERE 1 GROUP BY id HAVING count(*) > 1").unwrap();
    parse_sql("SELECT * FROM test ORDER BY 1").unwrap();
    parse_sql("SELECT * FROM test ORDER BY 1, id").unwrap();
    parse_sql("SELECT * FROM test LIMIT 1").unwrap();

    assert!(parse_sql("SELECT 1 FROM WHERE 1").is_err(),
            "error expected when no table name is specified");
}

#[test]
fn test_expression() {
    // TODO
}

#[test]
fn test_delete() {
    parse_sql("DELETE FROM test").unwrap();
    parse_sql("DELETE FROM main.test").unwrap();

    parse_sql("DELETE FROM test WHERE 1").unwrap();
    parse_sql("DELETE FROM test ORDER BY id").unwrap();
    parse_sql("DELETE FROM test LIMIT 1").unwrap();

    assert!(parse_sql("DELETE FROM").is_err(),
            "error expected when no table name is specified");
}

#[test]
fn test_update() {
    parse_sql("UPDATE test SET id = 1").unwrap();
    parse_sql("UPDATE main.test SET id = 1").unwrap();
    parse_sql("UPDATE test SET id = 1, name = 'test'").unwrap();

    parse_sql("UPDATE test SET id = 1 WHERE 1").unwrap();
    parse_sql("UPDATE test SET id = 1 ORDER BY id").unwrap();
    parse_sql("UPDATE test SET id = 1 LIMIT 1").unwrap();

    assert!(parse_sql("UPDATE SET id = 1").is_err(),
            "error expected when no table name is specified");
}

#[test]
fn test_insert() {
    parse_sql("INSERT INTO test VALUES (1)").unwrap();
    parse_sql("INSERT INTO main.test VALUES (1)").unwrap();
    parse_sql("INSERT INTO test VALUES (1, 'test')").unwrap();

    parse_sql("INSERT INTO test (id) VALUES (1)").unwrap();
    parse_sql("INSERT INTO test (id, name) VALUES (1, 'test')").unwrap();

    parse_sql("INSERT INTO test SELECT 1").unwrap();
    parse_sql("INSERT INTO test (id) SELECT 1").unwrap();
    parse_sql("INSERT INTO test (id, name) SELECT 1, 'test'").unwrap();

    parse_sql("INSERT INTO test DEFAULT VALUES").unwrap();
    parse_sql("INSERT INTO test (id, name) DEFAULT VALUES").unwrap();

    parse_sql("REPLACE INTO test VALUES (1)").unwrap();
    parse_sql("INSERT OR IGNORE INTO test VALUES (1)").unwrap();

    parse_sql("UPDATE test SET id = 1 WHERE 1").unwrap();
    parse_sql("UPDATE test SET id = 1 ORDER BY id").unwrap();
    parse_sql("UPDATE test SET id = 1 LIMIT 1").unwrap();

    assert!(parse_sql("INSERT INTO DEFAULT VALUES").is_err(),
            "error expected when no table name is specified");
}

#[test]
fn test_create_index() {
    parse_sql("CREATE INDEX idx ON test (name)").unwrap();
    parse_sql("CREATE INDEX main.idx ON test (name)").unwrap();

    parse_sql("CREATE INDEX idx ON test (id, name)").unwrap();
    parse_sql("CREATE UNIQUE INDEX idx ON test (name)").unwrap();
    parse_sql("CREATE INDEX idx ON test (name) WHERE 1").unwrap();

    parse_sql("CREATE INDEX IF NOT EXISTS idx ON test (name)").unwrap();

    assert!(parse_sql("CREATE INDEX ON test (name)").is_err(),
            "error expected when no index name is specified");
    assert!(parse_sql("CREATE INDEX idx ON (name)").is_err(),
            "error expected when no table name is specified");
    assert!(parse_sql("CREATE INDEX idx ON test ()").is_err(),
            "error expected when no column name is specified");
}

#[test]
fn test_drop_index() {
    parse_sql("DROP INDEX idx").unwrap();
    parse_sql("DROP INDEX main.idx").unwrap();

    parse_sql("DROP INDEX IF EXISTS idx").unwrap();

    assert!(parse_sql("DROP INDEX").is_err(),
            "error expected when no index name is specified");
}

#[test]
fn test_vacuum() {
    parse_sql("VACUUM").unwrap();
    parse_sql("VACUUM main").unwrap();
}

#[test]
fn test_pragma() {
    parse_sql("PRAGMA name").unwrap();
    parse_sql("PRAGMA main.name").unwrap();
    parse_sql("PRAGMA name('test')").unwrap();

    parse_sql("PRAGMA name=1").unwrap();

    assert!(parse_sql("PRAGMA").is_err(),
            "error expected when no pragma name is specified");
}

#[test]
fn test_create_trigger() {
    parse_sql("CREATE TRIGGER trgr UPDATE ON test BEGIN SELECT 1; END").unwrap();
    parse_sql("CREATE TRIGGER main.trgr BEFORE UPDATE ON test BEGIN SELECT 1; END").unwrap();

    // FIXME parse_sql("CREATE TRIGGER trgr BEFORE UPDATE ON test BEGIN SELECT RAISE(ABORT, '...') WHERE NEW.name <> OLD.name; END").unwrap();
    parse_sql("CREATE TRIGGER IF NOT EXISTS trgr UPDATE ON test BEGIN SELECT 1; END").unwrap();

    assert!(parse_sql("CREATE TRIGGER UPDATE ON test BEGIN SELECT 1; END").is_err(),
            "error expected when no trigger name is specified");
    assert!(parse_sql("CREATE TRIGGER trgr UPDATE ON BEGIN SELECT 1; END").is_err(),
            "error expected when no table name is specified");
    assert!(parse_sql("CREATE TRIGGER trgr UPDATE test ON BEGIN SELECT 1 FROM main.test; END")
                .is_err(),
            "error expected when qualified table name is specified");
}

#[test]
fn test_drop_trigger() {
    parse_sql("DROP TRIGGER trgr").unwrap();
    parse_sql("DROP TRIGGER main.trgr").unwrap();

    parse_sql("DROP TRIGGER IF EXISTS trgr").unwrap();

    assert!(parse_sql("DROP TRIGGER").is_err(),
            "error expected when no trigger name is specified");
}

#[test]
fn test_attach() {
    parse_sql("ATTACH 'test.db' AS aux").unwrap();
    parse_sql("ATTACH DATABASE 'test.db' AS aux").unwrap();

    assert!(parse_sql("ATTACH AS aux").is_err(),
            "error expected when no file name is specified");
    assert!(parse_sql("ATTACH 'test.db' AS").is_err(),
            "error expected when no alias is specified");
}

#[test]
fn test_detach() {
    parse_sql("DETACH aux").unwrap();
    parse_sql("DETACH DATABASE aux").unwrap();

    assert!(parse_sql("DETACH").is_err(),
            "error expected when no alias is specified");
}

#[test]
fn test_reindex() {
    parse_sql("REINDEX").unwrap();
    parse_sql("REINDEX test").unwrap();
    parse_sql("REINDEX main.test").unwrap();
}

#[test]
fn test_analyze() {
    parse_sql("ANALYZE").unwrap();
    parse_sql("ANALYZE test").unwrap();
    parse_sql("ANALYZE main.test").unwrap();
}

#[test]
fn test_alter_table() {
    parse_sql("ALTER TABLE test RENAME TO new").unwrap();
    parse_sql("ALTER TABLE main.test RENAME TO new").unwrap();
    parse_sql("ALTER TABLE test ADD new").unwrap();
    parse_sql("ALTER TABLE test ADD COLUMN new").unwrap();

    assert!(parse_sql("ALTER TABLE RENAME TO new").is_err(),
            "error expected when no table name is specified");
}

#[test]
fn test_create_virtual_table() {
    parse_sql("CREATE VIRTUAL TABLE test USING mod").unwrap();
    parse_sql("CREATE VIRTUAL TABLE main.test USING mod").unwrap();
    parse_sql("CREATE VIRTUAL TABLE test USING mod()").unwrap();
    parse_sql("CREATE VIRTUAL TABLE test USING mod('arg')").unwrap();

    assert!(parse_sql("CREATE VIRTUAL TABLE test USING").is_err(),
            "error expected when no module name is specified");
}
