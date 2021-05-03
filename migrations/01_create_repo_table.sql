CREATE table IF NOT EXISTS repo (
name TEXT NOT NULL,
org TEXT NOT NULL,
createdAt TEXT,
lastrun TEXT,
PRIMARY KEY(org, name),
FOREIGN KEY(org)
    REFERENCES org (name)
);
