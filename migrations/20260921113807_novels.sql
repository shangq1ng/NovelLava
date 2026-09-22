CREATE TABLE novels (
  id UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
  author TEXT NOT NULL,
  title TEXT NOT NULL,
  cover_path TEXT,
  description TEXT,
  tags TEXT,
  storage_path TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
