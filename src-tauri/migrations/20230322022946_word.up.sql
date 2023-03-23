-- Add up migration script here

-- vocabulary list
CREATE TABLE word (
  id INTEGER PRIMARY KEY,
  word VARCHAR(125) NOT NULL,
  soundmark VARCHAR(125),
  roots VARCHAR(125),
  paraphrase VARCHAR(255) NOT NULL,
  collocations VARCHAR(255),
  synonyms VARCHAR(255),
  examples text,
  classification VARCHAR(125),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX word_word_idx ON word (word);
CREATE INDEX classification_idx ON word (classification);