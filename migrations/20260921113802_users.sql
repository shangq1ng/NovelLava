CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    provider TEXT NOT NULL,
    sub TEXT,
    email TEXT NOT NULL UNIQUE,
    password varchar(10),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

