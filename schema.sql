CREATE TABLE queue (
    url TEXT PRIMARY KEY
);

CREATE TABLE pages (
    url TEXT PRIMARY KEY,
    content TEXT
);

CREATE TABLE links (
    from_url TEXT,
    to_url TEXT,
    PRIMARY KEY (from_url, to_url)
);
