-- create extension citext;
create table
  users (
    id serial primary key,
    -- preferred_username citext unique not null,
    preferred_username text unique not null,
    name text not null,
    summary text not null default '',
    followers text[] not null default '{}' check (array_position(followers, null) is null),
    following text[] not null default '{}' check (array_position(following, null) is null),
    public_key text not null,
    private_key text,
    published timestamptz not null default now(),
    -- email citext not null
    email text not null,
    password text not null
  );
