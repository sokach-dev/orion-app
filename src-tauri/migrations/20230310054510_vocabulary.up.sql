-- Add up migration script here

-- create vocabulary table
CREATE TABLE vocabulary (
    id INTEGER PRIMARY KEY,
    word TEXT NOT NULL, -- word
    soundmark TEXT NOT NULL, -- 音标
    roots TEXT, -- 词根词缀
    paraphrase TEXT NOT NULL, -- 中文释义
    collocations TEXT, -- 常用搭配
    synonyms TEXT, -- 同义词
    examples TEXT, -- 例句
    created_at TEXT DEFAULT (DATETIME('now', 'localtime')),
    updated_at TEXT DEFAULT (DATETIME('now', 'localtime'))
); 

CREATE UNIQUE INDEX word_idx ON vocabulary (word);
