# Wordforge ActivityPub

> 🚧 This is still in active development. Contributions are welcome! 🚧

A place where anyone can write novels using markdown. Designed with federation
using ActivityPub.

## Requirements

- ActivityPub protocol
- Rust
  - `actix-web`
  - `activitypub_federation`
  - `sqlx`
- Podman
- PostgreSQL
- Redis
- SvelteKit
- Tailwind CSS
- Typescript

## TODO and Plans

- [ ] Accounts
  - [x] Creation and Authentication
  - [ ] TOTP 2FA
  - [ ] Editing user info
  - [ ] Profile pictures etc.
  - [ ] Deleted users
  - [ ] Migrating accounts
  - [ ] Reading lists (Subscribed, Read, Want to Read, Dropped, Custom)
- [x] Webfinger
- [ ] NodeInfo
- [ ] Mail server
  - [ ] User email verification
  - [ ] Updates
- [ ] RSS Feed
- [ ] Books
  - [ ] Novels
    - [x] Creating books
    - [x] Set language
    - [ ] Editing and deleting
    - [x] Federate books
  - [ ] Comic books
    - [ ] [libacbf](https://codeberg.org/Grafcube/libacbf)
  - [ ] Compiling into volumes
  - [ ] Per chapter discussions and bookmarks
- [ ] Community
  - [ ] 5-Star review system
  - [ ] Discussion tab
- [ ] API access
  - [ ] Token generation
  - [ ] Scopes
- [ ] Admin dashboard and moderation
- [ ] Analytics
- [ ] Instance organized and federated events
- [ ] Payment methods
- [ ] Selling printed copies

## Development

1. Ensure that `rustup`, `sqlx-cli` and `podman-compose` are available.

2. Configure the `.env` file.

3. Start the PostgreSQL and Redis server.

```sh
podman-compose up -d
```

4. Run migrations.

```sh
sqlx migrate run
```

5. Start the server.

```sh
cargo run
```
