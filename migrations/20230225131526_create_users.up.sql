create table
  users (
    apub_id text primary key,
    preferred_username text unique not null,
    name text not null,
    summary text not null default '',
    inbox text not null,
    outbox text not null,
    followers text[] not null default '{}' check (array_position(followers, null) is null),
    following text[] not null default '{}' check (array_position(following, null) is null),
    public_key text not null,
    private_key text,
    published timestamptz not null default now(),
    last_refresh timestamp not null default now(),
    email text unique not null,
    password text not null
  );
