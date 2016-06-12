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

    assert!(parse_sql("BEGIN tx").is_err());
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

    assert!(parse_sql("COMMIT tx").is_err());
}

#[test]
fn test_rollback() {
    parse_sql("ROLLBACK").unwrap();

    parse_sql("ROLLBACK TRANSACTION").unwrap();

    parse_sql("ROLLBACK TRANSACTION tx").unwrap();
    parse_sql("ROLLBACK TRANSACTION `tx`").unwrap();
    parse_sql("ROLLBACK TRANSACTION \"tx\"").unwrap();
    parse_sql("ROLLBACK TRANSACTION [tx]").unwrap();

    assert!(parse_sql("ROLLBACK tx").is_err());
}

#[test]
fn test_savepoint() {
    parse_sql("SAVEPOINT sp").unwrap();

    assert!(parse_sql("SAVEPOINT").is_err());
}

#[test]
fn test_release() {
    parse_sql("RELEASE sp").unwrap();
    parse_sql("RELEASE SAVEPOINT sp").unwrap();

    parse_sql("ROLLBACK TO SAVEPOINT sp").unwrap();
    parse_sql("ROLLBACK TO sp").unwrap();

    assert!(parse_sql("RELEASE").is_err());
    assert!(parse_sql("ROLLBACK TO").is_err());
}
