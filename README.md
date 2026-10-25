# Question Bank

A full-stack Rust web application for creating and organizing academic questions by class, subject, and chapter. Built with Leptos and Axum, it renders pages on the server and hydrates them with WebAssembly in the browser.

## Features

- Sign up and log in with a username or email, using bcrypt password hashing and database-backed cookie sessions.
- Create and edit classes, subjects, and chapters in a hierarchical academic structure.
- Create and edit multiple-choice questions (MCQ) with answer options and correct-answer selection, or CQ questions with written answers.
- Assign Easy, Medium, or Hard difficulty to questions.
- Write questions and answers with Markdown formatting, tables, and LaTeX math, with a rendered preview and formula insertion tools.
- Browse questions by class, subject, and chapter, with chapter question counts.
- Switch between light and dark themes.

## Technology

| Layer | Implementation |
| --- | --- |
| UI and rendering | Leptos 0.8, Leptos Router, WebAssembly hydration |
| HTTP server | Axum 0.8 and Tokio |
| Database | PostgreSQL 17, SQLx 0.8, SQL migrations |
| Styling | Tailwind CSS 4 and Singlestage components |
| Content rendering | pulldown-cmark and RaTeX SVG math rendering |
| Testing | Rust unit/property tests, API test sources, Playwright |
| Packaging | cargo-leptos, Docker, Docker Compose |

## Run with Docker Compose

From the repository root, with Docker and Docker Compose installed:

```sh
docker compose up --build -d
```

Open **http://localhost:8888**. Compose builds the application and starts PostgreSQL, waits for the database health check, and applies the SQL files in `migrations/` when the database is first initialized.

```sh
docker compose logs -f app
docker compose down
```

The supplied Compose configuration is for local use. It uses development database credentials, does not publish PostgreSQL to the host, and does not declare a named database volume. Add an explicit persistent volume for data you need to retain across database container replacement. Initialization scripts run only against an empty database; later schema changes need a separate migration step.

The Dockerfile downloads Linux x86-64 tooling. On ARM hosts, use an amd64 build environment/emulation or adapt the downloaded tools for ARM.

## Local development

### Prerequisites

- Rust and Cargo via rustup. The Docker build uses Rust 1.93; the repository does not pin a local toolchain.
- The `wasm32-unknown-unknown` Rust target and `cargo-leptos`.
- Node.js and npm for the JavaScript dependencies and browser tests.
- PostgreSQL 17, or Docker to run the development database.
- The PostgreSQL command-line client (`psql`) and SQLx CLI.

Install the Rust tooling and project dependencies:

```sh
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
cargo install sqlx-cli --version '~0.8' --no-default-features --features rustls,postgres --locked
npm ci
```

### Initialize the database

The helper script starts a PostgreSQL 17 container named `question_bank_postgres`, publishes port `5432`, waits for it to become available, creates the database, and runs the migrations:

```sh
bash scripts/init_db.sh
```

If PostgreSQL is already running locally, skip container creation:

```sh
SKIP_DOCKER=1 bash scripts/init_db.sh
```

The script accepts `POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_DB`, and `POSTGRES_PORT`. Their defaults are `postgres`, `password`, `question_bank`, and `5432`. If you change them, also update the application's `APP_DATABASE__*` settings described below.

The Compose database is separate from this setup and has no host port mapping, so `docker compose up db` alone does not provide a database for a host-run application.

### Start the application

Run these commands from the repository root:

```sh
export DATABASE_URL='postgres://postgres:password@localhost:5432/question_bank'
cargo leptos watch --split
```

Open **http://127.0.0.1:3000**. The development reload port is `3001`.

`DATABASE_URL` is used by SQLx tooling and compile-time query checking. The running application reads its database connection from YAML configuration and `APP_DATABASE__*` environment variables instead. Keep both configurations pointed at the intended database.

### First use

1. Visit `/signup` to create an account, then log in at `/`.
2. Open **Academic Setting** (`/academic-setting`) and add a class, a subject within it, and a chapter within that subject.
3. Open the question browser (`/view`) and select the class, subject, and chapter.
4. Add an MCQ or CQ question, choose its difficulty, enter its options or answer, preview the content, and save it.
5. Return to the question browser to view or edit saved questions.

The question editor uses `/question?class=<id>&subject=<id>&chapter=<id>` for creation and `/question?id=<id>` for editing. Opening `/question` without context redirects to the question browser.

## Configuration

Application settings load in this order, with later values overriding earlier ones:

1. `config/base.yaml`
2. `config/local.yaml` by default, or `config/production.yaml` when `APP_ENVIRONMENT=production`
3. Environment variables prefixed with `APP_`, using `__` for nested fields

| Variable | Local default | Purpose |
| --- | --- | --- |
| `APP_ENVIRONMENT` | `local` | Select `local` or `production` configuration |
| `APP_APPLICATION__HOST` | `127.0.0.1` | Server bind address; production defaults to `0.0.0.0` |
| `APP_APPLICATION__PORT` | `3000` | Server listening port |
| `APP_DATABASE__HOST` | `localhost` | PostgreSQL host |
| `APP_DATABASE__PORT` | `5432` | PostgreSQL port |
| `APP_DATABASE__USERNAME` | `postgres` | PostgreSQL username |
| `APP_DATABASE__PASSWORD` | `password` | PostgreSQL password |
| `APP_DATABASE__DATABASE_NAME` | `question_bank` | Database name |
| `APP_DATABASE__REQUIRE_SSL` | `false` | Require PostgreSQL TLS; production defaults to `true` |

For example:

```sh
export APP_DATABASE__HOST=127.0.0.1
export APP_DATABASE__PASSWORD='your-database-password'
export APP_APPLICATION__PORT=3000
```

The application's loader reads the process environment; it does not load `.env` itself. Export runtime overrides in your shell or supply them through your deployment environment. The server binds using `APP_APPLICATION__HOST` and `APP_APPLICATION__PORT`.

Leptos build settings, asset paths, and browser test commands live in [`Cargo.toml`](Cargo.toml). Keep its `site-addr` consistent with the application address during development.

## Database migrations and SQLx

The application does not apply migrations at startup. For a database managed with SQLx, apply schema updates before starting the updated application:

```sh
export DATABASE_URL='postgres://postgres:password@localhost:5432/question_bank'
sqlx migrate run
```

The repository includes `.sqlx/` query metadata for offline compilation:

```sh
SQLX_OFFLINE=true cargo leptos build --release --split
```

Offline compilation still requires a database at runtime. After changing SQL queries or schema, migrate a development database and refresh the metadata:

```sh
cargo sqlx prepare -- --features ssr
```

Compose initializes its database by executing raw SQL files, without recording SQLx migration history. Do not apply the entire migration set again with SQLx against that initialized schema without first reconciling its migration history.

## Testing

Run Rust library tests using the server feature and cached SQLx metadata:

```sh
SQLX_OFFLINE=true cargo test --lib --features ssr
```

API tests under `tests/api/` cover authentication, the health route, and academic create, read, and update operations. Their helper creates a separate database per test and applies migrations, so the database user needs permission to create databases. With PostgreSQL available and the application settings configured, run:

```sh
SQLX_OFFLINE=true cargo test --test api --features test-fullstack
```

The helper does not drop its generated databases after the tests; use a development PostgreSQL instance.

For browser tests:

```sh
cd end2end
npm ci
npx playwright install
npm test
```

Playwright starts `cargo leptos watch --split`, or reuses an existing local server, at `http://127.0.0.1:3000`. Initialize the development database first and export the environment settings needed by the server. Authentication setup creates a test account and saves its session for the browser projects. Chromium, Firefox, and WebKit are configured.

Other browser test commands:

```sh
npm run test:chromium
npm run test:headed
npm run test:debug
npm run report
```

## Release build and deployment

Build the server and browser assets:

```sh
SQLX_OFFLINE=true cargo leptos build --release --split
```

The outputs are:

- `target/release/leptos-question-bank`: server executable
- `target/site/`: generated CSS, JavaScript, WebAssembly, and static assets

For deployment without a Rust toolchain, copy the executable, generated site directory, `Cargo.toml`, and `config/` to the destination:

```text
app/
├── leptos-question-bank
├── Cargo.toml
├── config/
│   ├── base.yaml
│   ├── local.yaml
│   └── production.yaml
└── site/
```

Run from that directory so the server can locate its configuration:

```sh
export APP_ENVIRONMENT=production
export APP_DATABASE__HOST=your-database-host
export APP_DATABASE__USERNAME=your-database-user
export APP_DATABASE__PASSWORD='your-database-password'
export APP_DATABASE__DATABASE_NAME=question_bank
export LEPTOS_SITE_ROOT=site
./leptos-question-bank
```

Production configuration binds to `0.0.0.0:3000` and requires database TLS. Apply migrations separately before launch. The included Dockerfile packages the same runtime files and settings.

The Dockerfile currently pins `wasm-bindgen-cli` to `0.2.125`, while `Cargo.lock` contains `wasm-bindgen` `0.2.122`. If a Docker build reports a schema/version mismatch, align the CLI and crate versions before rebuilding.

## Project structure

```text
src/
├── app.rs                 # HTML shell and application routes
├── main.rs                # Server entry point
├── lib.rs                 # Axum setup, shared context, and browser hydration
├── configuration.rs       # YAML and environment configuration
├── components/            # Layout, theme switcher, and editor components
├── domain/                # Data models and input validation
├── pages/                 # Authentication, academic settings, and questions
├── server_function/       # Server functions and database operations
└── util/                  # Authentication, errors, Markdown, and math helpers
config/                    # Base, local, and production settings
migrations/                # PostgreSQL schema migrations
.sqlx/                     # SQLx offline query metadata
scripts/                   # Development database setup helpers
style/                     # Tailwind and application CSS
public/                    # Static assets
tests/api/                 # API integration tests
end2end/                   # Playwright configuration and browser tests
```

See [`CHANGELOG.md`](CHANGELOG.md) for release history.

## License

Released under the Unlicense. See [`LICENSE`](LICENSE).
