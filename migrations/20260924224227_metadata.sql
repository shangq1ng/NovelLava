CREATE TYPE content_type AS ENUM ('everyone', 'teen', 'mature', 'adult');
CREATE TYPE title_status AS ENUM ('OnGoing', 'Completed', 'Hiatus', 'Cancelled', 'Dropped');

CREATE TABLE series (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    description TEXT,
    rating NUMERIC(3,2),
    release_date DATE,
    status title_status,
    content_rating content_type,
    updated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT titles_rating_range CHECK (rating IS NULL OR rating BETWEEN 0 AND 5)
);

CREATE TABLE authors (
  slug TEXT PRIMARY KEY,
  name TEXT NOT NULL  
);

CREATE TABLE genres (
  slug TEXT PRIMARY KEY,
  name TEXT NOT NULL  
);

CREATE TABLE tags (
  slug TEXT PRIMARY KEY,
  name TEXT NOT NULL  
);

CREATE TABLE series_type (
  slug TEXT PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE chapters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug TEXT NOT NULL,
    title TEXT NOT NULL,
    number NUMERIC,
    s3_path TEXT NOT NULL,
    updated_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);













CREATE TABLE covers (
  s3_path TEXT NOT NULL,
  content_type TEXT
);
