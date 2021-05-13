CREATE table IF NOT EXISTS repoQuery (
    name TEXT NOT NULL,
    org TEXT NOT NULL,
    stars INTEGER,
    languages TEXT,
    topics TEXT,
    createdAt TEXT,
    updatedAt TEXT,
    lastrun TEXT,
    PRIMARY KEY(org, name, lastrun),
    FOREIGN KEY(org)
        REFERENCES org (name)
);
