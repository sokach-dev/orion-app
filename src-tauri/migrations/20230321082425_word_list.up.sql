-- Add up migration script here

CREATE TABLE IF NOT EXISTS word_list (
    id INTEGER PRIMARY KEY,
    word TEXT NOT NULL, -- word
    paraphrase TEXT NOT NULL, -- 中文释义
    classification TEXT NOT NULL DEFAULT 'unknown',
    created_at TEXT DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT DEFAULT (DATETIME('now', 'localtime'))
);

CREATE INDEX word_list_classification_idx ON word_list (classification);
CREATE INDEX word_list_word_idx ON word_list (word);
