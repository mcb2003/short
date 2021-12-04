CREATE TABLE links (
    id UUID PRIMARY KEY DEFAULT GEN_RANDOM_UUID(),

    slug TEXT UNIQUE,
    uri TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX links_by_slug ON links(slug);
