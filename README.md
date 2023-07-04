# Wordforge ActivityPub

> 🚧 This is still in active development. Contributions are welcome! 🚧

<a href="https://codeberg.org/Grafcube/wordforge">
    <img alt="Get it on Codeberg" src="https://get-it-on.codeberg.org/get-it-on-white-on-black.svg" height="60">
</a>

A place where anyone can write novels using markdown. Designed with federation
using ActivityPub.

## Requirements

- ActivityPub protocol
- Rust
  - `actix-web`
  - `activitypub_federation`
  - `sqlx`
  - `leptos`
- Podman
- PostgreSQL
- Redis
- Tailwind CSS

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
    - [ ] Reading books
    - [x] Set language
    - [ ] Editing and deleting
    - [x] Federate books
    - [x] Chapters
      - [x] List chapters
      - [x] Create chapters
      - [ ] Edit chapters
      - [ ] Delete chapters
      - [ ] Write content
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
- [ ] Landing site with relay for discovery
- [ ] Analytics
- [ ] Instance organized and federated events
- [ ] Payment methods
- [ ] Selling printed copies

## Development

1. Ensure that `rustup`, `sqlx-cli`, `cargo-leptos` and `podman-compose` are available.

2. Configure the `.env` file.

   ```sh
   cp .env.example .env
   ```

3. Generate a key for cookie signing (TODO: Better way to use keys).

   ```sh
   openssl genrsa -out server/src/cookie.key 2048
   ```

4. Start the PostgreSQL and Redis server.

   ```sh
   podman-compose up -d
   ```

5. Run migrations.

   ```sh
   sqlx migrate run
   ```

6. Start the server.

   Watch mode:

   ```sh
   cargo leptos watch
   ```

   Serve mode:

   ```sh
   cargo leptos serve
   ```
