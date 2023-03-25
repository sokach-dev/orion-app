-- Add up migration script here
CREATE TABLE IF NOT EXISTS dialog (
    id INTEGER PRIMARY KEY,
    person TEXT NOT NULL, -- me or dic or openai
    content TEXT NOT NULL, -- content
    created_at TEXT DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT DEFAULT (DATETIME('now', 'localtime'))
);

CREATE INDEX dialog_person ON dialog (person);
