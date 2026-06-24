# Rust Learning Plan → Second Brain / Blog Hybrid App

A phased plan: learn just enough Rust from *The Book* to be dangerous, then build the app in increasing layers of complexity. Don't try to "finish" The Book before coding — alternate.

---

## Phase 0: Setup (Day 1)

- Install via `rustup` (not your OS package manager)
- `cargo new hello_rust && cd hello_rust && cargo run`
- Install `rust-analyzer` in your editor (VS Code or similar) — this makes a huge difference in learning speed via inline type hints and error explanations

---

## Phase 1: Core Language (The Book, Ch. 1–6)

**Goal:** comfortable with basic syntax, ownership, and control flow.

- Ch 1–2: Setup, "Guessing Game" project (do it — small interactive programs build muscle memory fast)
- Ch 3: Variables, data types, functions, control flow
- Ch 4: **Ownership, borrowing, references, slices** — the big one. Read it twice. Do every example by hand, don't copy-paste.
- Ch 5: Structs (you'll use these constantly — your `Article`, `Tag` types)
- Ch 6: Enums and `match` — Rust's `Option`/`Result` pattern starts here, foundational for error handling later

**Checkpoint exercise:** write a small CLI program that stores a `Vec<Article>` in memory (no database yet), lets you add an article via stdin, and prints all articles. Forces you to deal with ownership of strings/vectors without a database as a crutch.

---

## Phase 2: Project Structure & Error Handling (Ch. 7–9, 13)

- Ch 7: Modules, packages, crates — how to organize code into files (you'll want `models.rs`, `routes.rs`, `db.rs` etc.)
- Ch 8: Common collections — `Vec`, `HashMap`, `String` in depth
- Ch 9: Error handling — `Result`, `panic!`, the `?` operator. This is what you'll use everywhere in the API.
- Ch 13: Iterators and closures (skim now, revisit later — used heavily once you're querying/transforming data)

**Checkpoint exercise:** extend your CLI tool to use `Result` properly for "article not found" instead of crashing.

---

## Phase 3: Traits, Generics, Lifetimes (Ch. 10)

- Ch 10: Generics, traits, lifetimes — don't aim for full mastery yet, just recognition. You'll see `impl Trait`, `&'a str`, and generic bounds constantly in framework code (Axum handlers, sqlx queries) even before you write your own.

---

## Phase 4: Testing & Smart Pointers (Ch. 11–12, 15)

- Ch 11: Writing tests (`#[test]`, `assert_eq!`) — start testing your CLI tool now, good habit before the app gets complex
- Ch 12: Build a small CLI project (the book's `minigrep`) — good real practice with file I/O and error propagation
- Ch 15: Smart pointers (`Box`, `Rc`) — skim, you likely won't need these heavily for a web app, but recognition matters

---

## Phase 5: Concurrency & Async (Ch. 16, + async book supplement)

- Ch 16: Concurrency basics (threads, channels) — gives you the mental model
- **Note:** The Book's async coverage is thin. Once you hit this point, switch to the [Async Book](https://rust-lang.github.io/async-book/) or just learn async-in-context once you start using Tokio/Axum in Phase 7. Async will make more sense with a real use case (handling HTTP requests) than in the abstract.

You can stop "reading The Book front to back" here — Ch 17+ (OOP patterns, patterns/matching deep dive, advanced features) is worth skimming later as reference, not required before building.

---

## Phase 6: Build the App — Step 1: CLI/local version (no web yet)

**Goal:** validate your data model before adding HTTP/DB complexity.

- Define `Article`, `Tag` structs
- Store articles as markdown files on disk with frontmatter (title, tags, date) — like a static site generator would
- Write a small CLI: list articles, search by tag, search by keyword (simple `.contains()` is fine for now)

This avoids database setup friction while you're still shaky on ownership, and gives you a working "second brain" core before any backend exists.

---

## Phase 7: Build the App — Step 2: Real backend

- Swap file storage for **Postgres + sqlx**
- Build the schema: `articles`, `tags`, `article_tags` join table
- Stand up **Axum**: routes for `GET /articles`, `GET /articles/:slug`, `POST /articles`, `GET /search?q=`
- Add Postgres full-text search (`tsvector`) for the search endpoint
- Markdown rendering server-side with `pulldown-cmark`
- Basic auth for write endpoints (just you posting articles) — simple API key or session check is enough at this stage, don't over-engineer auth yet

**This is where async finally clicks** — Tokio + Axum handlers will make `.await` make sense because you're using it for a real reason (non-blocking DB calls).

---

## Phase 8: Build the App — Step 3: React frontend

- Two views: public blog layout (`/blog`, `/blog/:slug`) and your private dashboard (`/search`, `/dashboard`)
- `react-markdown` if you're sending raw markdown, or just render HTML from the server if you converted it there
- Image upload: simple endpoint that saves files and returns a URL to embed

---

## Phase 9: Iterate

Only after Phases 6–8 work end-to-end:
- Backlinks between articles (recursive queries)
- Tags UI, filtering
- Tantivy if Postgres FTS search quality starts to bug you
- MCP server wrapper exposing `search_notes()` / `get_article()` as tools, reusing your existing Axum logic

---

## Rough Time Expectations (very approximate, side-project pace)

| Phase | Focus | Rough time |
|---|---|---|
| 0–4 | Core Rust + Book | 2–4 weeks |
| 5 | Concurrency/async intro | a few days, revisit in 7 |
| 6 | CLI version of app | 1 week |
| 7 | Axum + Postgres backend | 2–3 weeks |
| 8 | React frontend | 1–2 weeks |
| 9 | Polish/extend | ongoing |

Don't treat this as a strict schedule — the point of Phase 6 especially is to get a working thing fast and let real bugs teach you more than reading ever will.