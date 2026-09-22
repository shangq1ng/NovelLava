CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    provider TEXT NOT NULL,
    sub TEXT,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT,
    email TEXT NOT NULL UNIQUE,
    password TEXT,
    avatar_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

