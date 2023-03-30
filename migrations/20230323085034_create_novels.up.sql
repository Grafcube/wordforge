create extension if not exists "uuid-ossp";

create table
  novels (
    id uuid default uuid_generate_v4 () primary key,
    title text not null,
    summary text not null default '',
    authors text[] not null,
    genre text not null,
    tags text[] not null,
    language text not null,
    content_warning boolean not null,
    followers text[] not null default '{}' check (array_position(followers, null) is null),
    following text[] not null default '{}' check (array_position(following, null) is null),
    public_key text not null,
    private_key text,
    published timestamptz not null default now(),
    last_refresh timestamp not null default now()
  );

create table
  author_roles (
    id uuid not null,
    author text not null,
    role text not null,
    primary key (id, author)
  )
