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

    assert!(parse_sql("BEGIN tx").is_err(), "error expected when transaction name is specified without `TRANSACTION` keyword preceding");
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

    assert!(parse_sql("COMMIT tx").is_err(), "error expected when transaction name is specified without `TRANSACTION` keyword preceding");
}

#[test]
fn test_rollback() {
    parse_sql("ROLLBACK").unwrap();

    parse_sql("ROLLBACK TRANSACTION").unwrap();

    parse_sql("ROLLBACK TRANSACTION tx").unwrap();
    parse_sql("ROLLBACK TRANSACTION `tx`").unwrap();
    parse_sql("ROLLBACK TRANSACTION \"tx\"").unwrap();
    parse_sql("ROLLBACK TRANSACTION [tx]").unwrap();

    assert!(parse_sql("ROLLBACK tx").is_err(), "error expected when transaction name is specified without `TRANSACTION` keyword preceding");
}

#[test]
fn test_savepoint() {
    parse_sql("SAVEPOINT sp").unwrap();

    assert!(parse_sql("SAVEPOINT").is_err(), "error expected when no savepoint name is specified");
}

#[test]
fn test_release() {
    parse_sql("RELEASE sp").unwrap();
    parse_sql("RELEASE SAVEPOINT sp").unwrap();

    parse_sql("ROLLBACK TO SAVEPOINT sp").unwrap();
    parse_sql("ROLLBACK TO sp").unwrap();

    assert!(parse_sql("RELEASE").is_err(), "error expected when no savepoint name is specified");
    assert!(parse_sql("ROLLBACK TO").is_err(), "error expected when no savepoint name is specified");
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

    assert!(parse_sql("CREATE TABLE test").is_err(), "error expected when no column list is specified");
    assert!(parse_sql("CREATE TABLE test ()").is_err(), "error expected when no column is specified");
    assert!(parse_sql("CREATE TABLE test (PRIMARY KEY (id))").is_err(), "error expected when only table constraint is specified");
}

#[test]
fn test_drop_table() {
    parse_sql("DROP TABLE test").unwrap();
    parse_sql("DROP TABLE main.test").unwrap();

    parse_sql("DROP TABLE IF EXISTS test").unwrap();

    assert!(parse_sql("DROP TABLE").is_err(), "error expected when no table name is specified");
}

#[test]
fn test_create_view() {
    parse_sql("CREATE VIEW test AS SELECT 1").unwrap();
    parse_sql("CREATE VIEW test (id) AS SELECT 1").unwrap();
    parse_sql("CREATE VIEW main.test AS SELECT 1").unwrap();

    parse_sql("CREATE TABLE IF NOT EXISTS test AS SELECT 1").unwrap();

    assert!(parse_sql("CREATE VIEW AS SELECT 1").is_err(), "error expected when no view name is specified");
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

    assert!(parse_sql("SELECT 1 FROM WHERE 1").is_err(), "error expected when no table name is specified");
}

#[test]
fn test_delete() {
    parse_sql("DELETE FROM test").unwrap();
    parse_sql("DELETE FROM main.test").unwrap();

    parse_sql("DELETE FROM test WHERE 1").unwrap();
    parse_sql("DELETE FROM test ORDER BY id").unwrap();
    parse_sql("DELETE FROM test LIMIT 1").unwrap();

    assert!(parse_sql("DELETE FROM").is_err(), "error expected when no table name is specified");
}

#[test]
fn test_update() {
    parse_sql("UPDATE test SET id = 1").unwrap();
    parse_sql("UPDATE main.test SET id = 1").unwrap();
    parse_sql("UPDATE main.test SET id = 1, name = 'test'").unwrap();

    parse_sql("UPDATE test SET id = 1 WHERE 1").unwrap();
    parse_sql("UPDATE test SET id = 1 ORDER BY id").unwrap();
    parse_sql("UPDATE test SET id = 1 LIMIT 1").unwrap();

    assert!(parse_sql("UPDATE SET id = 1").is_err(), "error expected when no table name is specified");
}
