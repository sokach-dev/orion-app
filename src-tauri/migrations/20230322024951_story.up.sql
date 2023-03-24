-- Add up migration script here

-- create a story table
CREATE TABLE IF NOT EXISTS story (
    id INTEGER PRIMARY KEY,
    words TEXT NOT NULL,
    content TEXT NOT NULL,
    read_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT DEFAULT (DATETIME('now', 'localtime'))
);

CREATE INDEX story_words_idx ON story (words);
CREATE INDEX story_read_count_idx ON story (read_count);

