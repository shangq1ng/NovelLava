CREATE TABLE novels (
  id UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
  title TEXT NOT NULL,
  description TEXT,
  tags TEXT,
  content_id TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);