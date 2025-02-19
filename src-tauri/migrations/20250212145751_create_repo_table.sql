-- Add migration script here
CREATE TABLE IF NOT EXISTS repo (
    id TEXT PRIMARY KEY, 
    full_path TEXT NOT NULL, 
    name TEXT NOT NULL, 
    avatar_url TEXT
)