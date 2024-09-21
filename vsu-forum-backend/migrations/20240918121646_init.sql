create table users (
    id INTEGER NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    login text NOT NULL ,
    password_hash BYTEA NOT NULL 
);

create table topics (
    id INTEGER NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    author_id INTEGER NOT NULL references users(id),
    name text NOT NULL
);

create table posts (
    id INTEGER NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    topic_id INTEGER NOT NULL references topics(id),
    author_id INTEGER NOT NULL references users(id),
    text text NOT NULL
);