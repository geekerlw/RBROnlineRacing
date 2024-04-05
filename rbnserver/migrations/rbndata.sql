CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    passwd TEXT NOT NULL default 'simrallycn',
    license TEXT NOT NULL default 'Rookie',
    score INTEGER default 0
);