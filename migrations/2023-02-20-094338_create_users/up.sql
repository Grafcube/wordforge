CREATE EXTENSION citext;

CREATE TABLE
  USERS (
    id INT PRIMARY KEY,
    preferred_username CITEXT UNIQUE NOT NULL,
    NAME TEXT NOT NULL,
    summary TEXT NOT NULL DEFAULT '',
    inbox TEXT NOT NULL,
    followers TEXT[] NOT NULL DEFAULT '{}' CHECK (ARRAY_POSITION(followers, NULL) IS NULL),
    FOLLOWING TEXT[] NOT NULL DEFAULT '{}' CHECK (ARRAY_POSITION(FOLLOWING, NULL) IS NULL),
    public_key TEXT NOT NULL,
    private_key TEXT,
    published TIMESTAMP NOT NULL
  );
