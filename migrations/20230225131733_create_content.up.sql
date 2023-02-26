create table
  content (
    id uuid primary key,
    -- preferred_username citext unique not null,
    preferred_username text unique not null,
    name text not null,
    summary text not null default '',
    followers text[] not null default '{}' check (array_position(followers, null) is null),
    following text[] not null default '{}' check (array_position(following, null) is null),
    published timestamptz not null
  );
