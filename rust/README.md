# Rust Track: Under the Hood

This directory contains the Rust implementations for the **Agentic AI** and **Machine Learning** series.  
The projects are organized as a **Cargo workspace** so multiple lessons and experiments can share dependencies, build artifacts, and environment configuration cleanly.

## Workspace Structure

The workspace is split into two main tracks:

- `agentic-ai/` — agent architectures, tool calling, reasoning loops, and applied LLM workflows
- `ml/` — machine learning fundamentals and lower-level implementations using Rust-first tooling

Current structure:

```text
rust/
├── agentic-ai/
│   ├── setup/
│   └── what-is-agent/
├── ml/
│   └── placeholder/
├── Cargo.toml
├── Cargo.lock
└── README.md
````

## Why a Cargo Workspace?

Using a workspace gives us a few nice benefits:

* **Shared dependencies** — crate versions can be defined once at the root
* **Shared build artifacts** — all projects use the same `target/` directory
* **Cleaner project organization** — each lesson can live as its own crate
* **Shared environment setup** — a single root `.env` file can be used across projects

## Dependency Management

Workspace-level dependencies are defined in the root `Cargo.toml` under:

```toml
[workspace.dependencies]
```

For example:

```toml
[workspace.dependencies]
tokio = { version = "1.0", features = ["full"] }
dotenvy = "0.15"
serde = { version = "1.0", features = ["derive"] }
```

### Add a new shared dependency

Add it manually to the root `Cargo.toml`:

```toml
[workspace.dependencies]
anyhow = "1"
```

Then, inside the specific crate that needs it, enable it like this:

```toml
[dependencies]
anyhow.workspace = true
```

### Add a dependency to a specific project

If you want to add a dependency directly to one crate, use:

```bash
cargo add <crate_name> -p <project-name>
```

Example:

```bash
cargo add anyhow -p setup
```

If that crate is already defined in `[workspace.dependencies]`, Cargo may wire it up using `workspace = true` depending on your setup. If not, you can update the crate’s `Cargo.toml` manually.

## Running Projects

You do **not** need to `cd` into each subfolder.
Run any project from this `rust/` directory with the package flag:

```bash
cargo run -p <project-name>
```

### Examples

#### 1. Verify setup

Checks your environment configuration and API connectivity.

```bash
cargo run -p setup
```

#### 2. Run the “What is an Agent?” example

```bash
cargo run -p what-is-agent
```

## Environment Variables

This workspace uses a shared `.env` file stored in the root `rust/` directory.

### Create the file

```bash
touch .env
```

### Add your API key

```env
GEMINI_API_KEY=your_actual_key_here
```

## Important Note About `.env` Paths

When loading environment files, the path should be understood relative to the **working directory / `Cargo.toml` you are running from**, not relative to `main.rs`.

For example:

* If you run a package from the root workspace using `cargo run -p setup`, the effective base is the `rust/` workspace root
* So any explicit `.env` path should be written relative to the root `Cargo.toml`
* It should **not** be written relative to `src/main.rs`

In many cases, using `dotenvy::dotenv()` is enough, because it automatically searches upward for a `.env` file.

## Hardware and Environment

The Rust track is primarily developed and tested on **macOS (Apple Silicon)**.
Rust itself is cross-platform, but local benchmarks and some tool behavior may have been validated first on ARM64 macOS systems.

## Notes

* All workspace members share the same lockfile: `Cargo.lock`
* All crates share the same build output directory: `target/`
* New lessons can be added as new crates under `agentic-ai/` or `ml/`

---

Built for the AI Engineering community at [aiunderthehood.com](https://aiunderthehood.com)

