CREATE TABLE chapters (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    chapter_title TEXT NOT NULL,
    chapter_number FLOAT NOT NULL, -- chapter number (eg; Chapter -> 12.4 <- in it)
    chapter_path TEXT NOT NULL, -- chapter's path
    word_count INTEGER,
    published_at TIMESTAMPTZ, -- Original publish date
    scraped_at TIMESTAMPTZ -- date when we "acquired" it,
    );
