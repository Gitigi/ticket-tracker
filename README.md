# ticket-tracker

A small ticket tracker

- **Backend**: Rust ([axum](https://github.com/tokio-rs/axum) + [sqlx](https://github.com/launchbadge/sqlx)) on PostgreSQL, port `3001`
- **Frontend**: React ([TanStack Start](https://tanstack.com/start) / Router / Query + Tailwind CSS v4) in `web/`, port `3000`


## Decisions & Tradeoffs

- Clean architecture for our api - separates our core business logic from external dependency (framework, db). This takes a lot of code as we have to write interfaces that act as our boundaries. But the benefit is our code is more modular, and allows more flexibilty in changing it. As we can make changes in one place without affect other as long as the interface is consistent. Enable greater reusability of code, as if we were to add a GRPC endpoint it will be easy. We can also easily switch external dependecy.

- Though I have written the api in Rust, depending with the use case of our application I would have choosen a javascript fullstack framework. Rust type system makes it easy to model our core business model and since we are to expose some api I went with it. But if for our use case we were the only one consuming the api through our web app I would have just used a fullstack javascript framework with server functions.

### Stuff that I skipped

- tracing and proper logging
- if this was to be deploy I would have included the terraform to handle our infrature configuration
- Better UI/UX - like instead of having to go to a form to update status, it could be a dropdown in place


## Running the application

### With Docker (everything)

`.env` sets `DATABASE_URL`

```sh
cp .env.example .env
```

```sh
docker compose up --build
```

This starts Postgres, the API on <http://localhost:3001> and the web app on <http://localhost:3000>. Docker is the only thing you need installed. Both the API and the web app reload when you change code:

Useful commands:

```sh
docker compose logs -f api             # follow API output
docker compose exec api cargo test     # run the tests inside the container
docker compose down                    # stop (add -v to also wipe the DB and build caches)
```

### With a local toolchain

#### Prerequisites

- Docker
- Optional: [`sqlx-cli`](https://crates.io/crates/sqlx-cli) for managing migrations: `cargo install sqlx-cli --no-default-features --features rustls,postgres`



### Building without a database

If you want to build without a live database (for example in CI), generate the offline query metadata and commit it:

```sh
cargo sqlx prepare   # writes .sqlx/
```

Without `DATABASE_URL`, or with `SQLX_OFFLINE=true`, the macros use `.sqlx/` instead. Rerun `cargo sqlx prepare` after changing any query or the schema.

## API

All routes are under `/api/tickets`. Enum values are `Open | InProgress | Closed` for status and `Low | Medium | High` for priority.

| Method  | Path                 | Body / query                                              | Response                         |
|---------|----------------------|-----------------------------------------------------------|----------------------------------|
| `POST`  | `/api/tickets`       | `{ "title", "description"?, "priority" }`                 | `201` + created ticket (status `Open`) |
| `GET`   | `/api/tickets`       | `?status=…&limit=…&page=…` (all optional, `page` is 1-based) | `200` + array, oldest first   |
| `GET`   | `/api/tickets/{id}`  |                                                           | `200` + ticket                   |
| `PATCH` | `/api/tickets/{id}`  | any of `{ "title", "description", "priority", "status" }` | `200`; only the given fields change |
| `GET`   | `/api/tickets/stats` |                                                           | `200` + `{ "open", "in_progress", "closed" }` |

Example:

```sh
curl -X POST localhost:3001/api/tickets \
  -H 'content-type: application/json' \
  -d '{"title":"Printer jammed","priority":"High"}'
```

## Codebase overview

The backend follows a layered (hexagonal) architecture. Dependencies point inward:

- `domain` depends on nothing.
- `application` depends on `domain` and defines the interfaces (traits) it needs.
- `adapters` implement those traits (Postgres, in-memory) and expose them over HTTP.
- `infra` wires everything together.

```
.
├── Cargo.toml
├── docker-compose.yml          # db + api + web for local development
├── Dockerfile.dev              # API dev image (watchexec + cargo run)
├── .env.example                # DATABASE_URL template
├── migrations/                 # sqlx migrations (applied on startup)
├── src/
│   ├── main.rs                 # loads .env, builds app state, starts the server
│   ├── lib.rs
│   ├── domain/                 # pure business types, no framework code
│   │   ├── entities/ticket.rs  #   Ticket
│   │   └── values.rs           #   TicketStatus, TicketPriority
│   ├── application/            # use cases and the ports they depend on
│   │   ├── use_cases/ticket.rs #   TicketUseCases + TicketRepository trait
│   │   ├── dto.rs              #   request/response shapes (CreateTicket, UpdateTicket, ListTicket, …)
│   │   └── app_error.rs        #   AppError / AppResult
│   ├── adapters/
│   │   ├── http/               # axum: routes, AppState, AppError → HTTP response mapping
│   │   └── repository/         # TicketRepository implementations
│   │       ├── ticket.rs       #   PostgresRepository (sqlx, compile-time checked queries)
│   │       └── memory_ticket.rs#   MemoryRepository (in-memory, handy for experiments)
│   └── infra/
│       ├── db.rs               # connection pool + migrations
│       ├── setup.rs            # picks the repository and builds AppState
│       └── app.rs              # router, CORS
├── tests/
│   └── ticket_api.rs           # end-to-end API tests against Postgres
└── web/                        # React frontend
    ├── Dockerfile.dev          # web dev image (Vite dev server)
    └── src/
        ├── routes/             # file-based routes (TanStack Router)
        │   ├── __root.tsx      #   app shell
        │   ├── index.tsx       #   ticket list
        │   ├── create.tsx      #   new ticket form
        │   └── update.$ticketId.tsx  # edit ticket
        ├── router.tsx
        ├── routeTree.gen.ts    # generated, do not edit
        └── styles.css          # Tailwind v4 entry point
```

### Adding Migration

- **Adding a field or query**: add a migration (`cargo sqlx migrate add <name>`),
