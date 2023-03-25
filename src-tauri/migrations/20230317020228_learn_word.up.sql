-- Add up migration script here

CREATE TABLE IF NOT EXISTS learn_word (
    id INTEGER PRIMARY KEY,
    word TEXT NOT NULL,
    vocabulary_id INTEGER,
    paraphrase TEXT NOT NULL,
    learn_count  INTEGER NOT NULL DEFAULT 0,
    next_learn_at INTEGER,
    created_at TEXT DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT DEFAULT (DATETIME('now', 'localtime'))
);

CREATE UNIQUE INDEX learn_word_word_idx ON learn_word (word);
CREATE INDEX learn_word_next_learn_at_idx ON learn_word (next_learn_at);

