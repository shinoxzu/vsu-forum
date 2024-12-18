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
    author_id BIGINT NOT NULL references users(id) ON DELETE CASCADE,
    category_id BIGINT NOT NULL references topics_categories(id) ON DELETE CASCADE,
    name text NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create table posts (
    id BIGINT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    topic_id BIGINT NOT NULL references topics(id) ON DELETE CASCADE,
    author_id BIGINT NOT NULL references users(id) ON DELETE CASCADE,
    text text NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create table reports (
    id BIGINT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    author_id BIGINT NOT NULL references users(id) ON DELETE CASCADE,
    reported_user_id BIGINT NOT NULL references users(id) ON DELETE CASCADE,
    reason text NOT NULL
);

create table bookmarks (
    user_id BIGINT NOT NULL references users(id) ON DELETE CASCADE,
    topic_id BIGINT NOT NULL references topics(id) ON DELETE CASCADE,
    PRIMARY KEY(user_id, topic_id)
);

create table available_reactions (
    id BIGINT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    reaction TEXT NOT NULL
);

create table reactions (
    post_id BIGINT NOT NULL references posts(id) ON DELETE CASCADE,
    author_id BIGINT NOT NULL references users(id) ON DELETE CASCADE,
    reaction_id BIGINT NOT NULL references available_reactions(id) ON DELETE CASCADE,
    PRIMARY KEY(post_id, author_id, reaction_id)
);
