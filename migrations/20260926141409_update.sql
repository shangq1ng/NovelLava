ALTER TABLE series
  ADD COLUMN series_type_slug TEXT REFERENCES series_type(slug);

CREATE TABLE series_authors (
  series_id   UUID NOT NULL REFERENCES series(id) ON DELETE CASCADE,
  author_slug TEXT NOT NULL REFERENCES authors(slug),
  role        TEXT,                         -- 'author' | 'artist' (optional)
  PRIMARY KEY (series_id, author_slug)
);
CREATE INDEX series_authors_author_idx ON series_authors (author_slug);

CREATE TABLE series_genres (
  series_id  UUID NOT NULL REFERENCES series(id) ON DELETE CASCADE,
  genre_slug TEXT NOT NULL REFERENCES genres(slug),
  PRIMARY KEY (series_id, genre_slug)
);
CREATE INDEX series_genres_genre_idx ON series_genres (genre_slug);

CREATE TABLE series_tags (
  series_id UUID NOT NULL REFERENCES series(id) ON DELETE CASCADE,
  tag_slug  TEXT NOT NULL REFERENCES tags(slug),
  PRIMARY KEY (series_id, tag_slug)
);
CREATE INDEX series_tags_tag_idx ON series_tags (tag_slug);
