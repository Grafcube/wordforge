create table
  novels (
    apub_id text primary key,
    preferred_username uuid not null,
    title text not null,
    summary text not null default '',
    authors text[] not null,
    genre text not null,
    tags text[] not null,
    language text not null,
    sensitive boolean not null,
    inbox text not null,
    outbox text not null,
    followers text[] not null default '{}' check (array_position(followers, null) is null),
    following text[] not null default '{}' check (array_position(following, null) is null),
    public_key text not null,
    private_key text,
    published timestamptz not null default now(),
    last_refresh timestamp not null default now()
  );

create table
  author_roles (
    id text not null,
    author text not null,
    role text not null,
    primary key (id, author)
  )
