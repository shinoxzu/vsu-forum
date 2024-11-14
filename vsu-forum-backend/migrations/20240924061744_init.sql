create table users (
    id BIGINT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    login text NOT NULL UNIQUE,
    password_hash BYTEA NOT NULL
);

create table topics_categories (
    id BIGINT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL UNIQUE
);

create table topics (
    id BIGINT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    author_id BIGINT NOT NULL references users(id),
    category_id BIGINT NOT NULL references topics_categories(id),
    name text NOT NULL
);

create table posts (
    id BIGINT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    topic_id BIGINT NOT NULL references topics(id),
    author_id BIGINT NOT NULL references users(id),
    text text NOT NULL
);

create table reports (
    id BIGINT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    author_id BIGINT NOT NULL references users(id),
    reported_user_id BIGINT NOT NULL references users(id),
    UNIQUE(author_id, reported_user_id)
);

create table bookmarks (
    user_id BIGINT NOT NULL references users(id),
    topic_id BIGINT NOT NULL references topics(id),
    PRIMARY KEY(user_id, topic_id)
);

create table attachments (
    post_id BIGINT NOT NULL references posts(id),
    link TEXT NOT NULL,
    PRIMARY KEY(post_id, link)
);

create table reactions (
    post_id BIGINT NOT NULL references posts(id),
    author_id BIGINT NOT NULL references users(id),
    reaction TEXT NOT NULL,
    PRIMARY KEY(post_id, author_id, reaction)
);
