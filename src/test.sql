-- name: select_something
SELECT things
FROM place
WHERE niceness > 5;

-- name: insert_something
INSERT INTO place (name)
VALUES ($1)
RETURNING id;
