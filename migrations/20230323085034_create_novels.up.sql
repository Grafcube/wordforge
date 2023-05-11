create table
  novels (
    apub_id text primary key,
    preferred_username uuid not null,
    title text not null,
    summary text not null default '',
    genre text not null,
    tags text[] not null,
    language text not null,
    sensitive boolean not null,
    inbox text not null,
    outbox text not null,
    public_key text not null,
    private_key text,
    published timestamptz not null default now(),
    last_refresh timestamp not null default now()
  );

create table
  author_roles (
    id text not null,
    author text not null,
    role text not null default 'None',
    primary key (id, author)
  );

create table
  chapters (
    apub_id text primary key,
    audience text not null,
    sequence int not null,
    title text not null,
    summary text not null default '',
    sensitive boolean not null,
    content text not null default '',
    published timestamptz not null default now(),
    updated timestamptz default null,
    last_refresh timestamp not null default now()
  );
