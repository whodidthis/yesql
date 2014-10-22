#![feature(phase)]

#[phase(plugin)]
extern crate yesql;

#[test]
fn find_first() {
    let function = sql_query!("test.sql", "select_something");
    assert_eq!("SELECT things
FROM place
WHERE niceness > 5", function);
}

#[test]
fn find_other() {
    let function = sql_query!("test.sql", "insert_something");
    assert_eq!("INSERT INTO place (name)
VALUES ($1)
RETURNING id", function);
}
