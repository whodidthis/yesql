yesql
=====

In appreciation of the real [Yesql](https://github.com/krisajenkins/yesql)]. 
This is just a library you can insert sql queries to places with at compile time.

sql_query!
=====

The `sql_query!` macro will search given file for given query by its name
and slot that in. **Don't forget to terminate the sql queries!**

```sql
-- name: find_user
SELECT *
FROM users
WHERE name = $1;

-- name: create_user
INSERT INTO users (name)
VALUES ($1);
```

```rust
#![feature(phase)]

...
#[phase(syntax)]
extern crate yesql;

fn main() {
    ...
    let stmt = conn.prepare(sql_query("users.sql", "create_user"))
```
    
