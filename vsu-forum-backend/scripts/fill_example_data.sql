CREATE OR REPLACE FUNCTION random_between(low INT ,high INT) 
   RETURNS INT AS
$$
BEGIN
   RETURN floor(random()* (high-low + 1) + low);
END;
$$ language 'plpgsql' STRICT;

INSERT INTO users (login, password_hash)
SELECT FORMAT('user%s', t.i), 'password'::bytea
FROM generate_series(1, 10000) as t(i);

INSERT INTO topics_categories (name) VALUES
    ('anime'),
    ('films'),
    ('girls'),
    ('games');

INSERT INTO topics (author_id, category_id, name)
SELECT random_between(1, 1000), random_between(1, 4), FORMAT('topic%s', t.i)
FROM generate_series(1, 1000) as t(i);

INSERT INTO posts (topic_id, author_id, text)
SELECT t.i, random_between(1, 1000), 'text'
FROM generate_series(1, 1000) as t(i);

INSERT INTO reports (author_id, reported_user_id)
SELECT t.i, random_between(1, 1000)
FROM generate_series(1, 1000) as t(i);

INSERT INTO bookmarks (user_id, topic_id)
SELECT t.i, random_between(1, 1000)
FROM generate_series(1, 1000) as t(i);

INSERT INTO attachments (message_id, link)
SELECT t.i, 'https://link.com/file/assaf'
FROM generate_series(1, 1000) as t(i);

INSERT INTO reactions (message_id, author_id, reaction)
SELECT t.i, t.i, 'smile'
FROM generate_series(1, 1000) as t(i);