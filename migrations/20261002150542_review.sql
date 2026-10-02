CREATE TABLE reviews (
                         id UUID NOT NULL,
                         username TEXT NOT NULL,
                         novel_id UUID NOT NULL,
                         parent_id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
                         child_id UUID REFERENCES reviews(parent_id),
                         content TEXT NOT NULL,
                         created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                         rating NUMERIC
);
