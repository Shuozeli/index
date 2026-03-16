# Project Index

A catalog of my open-source projects.

## Active Projects (2026)

### Serialization & Schema Tools

| Project | Language | Description |
|---------|----------|-------------|
| [protobuf-rs](https://github.com/Shuozeli/protobuf-rs) | Rust | Pure Rust Protocol Buffers compiler with recursive descent parser, semantic analysis, and byte-for-byte compatible output with `protoc` |
| [flatbuffers-rs](https://github.com/Shuozeli/flatbuffers-rs) | Rust | Pure Rust drop-in replacement for the FlatBuffers compiler (`flatc`) with Rust and TypeScript codegen |
| [protoviewer-lib](https://github.com/Shuozeli/protoviewer-lib) | Rust | Interactive protobuf binary encoding visualizer with hex view, structure tree, and decoded JSON (WASM + native) |
| [fbsviewer](https://github.com/Shuozeli/fbsviewer) | JavaScript | Browser-based FlatBuffers binary inspector with color-coded hex view and structure tree |
| [fbsviewer-lib](https://github.com/Shuozeli/fbsviewer-lib) | Rust | Core library for the FlatBuffers binary visualizer (WASM + native) |

### Data & Database

| Project | Language | Description |
|---------|----------|-------------|
| [quiver-orm](https://github.com/Shuozeli/quiver-orm) | Rust | Arrow-native ORM where schema types map 1:1 to Apache Arrow types, with ADBC connectivity and multi-format codegen from `.quiver` schema files |
| [arrow-adbc-rs](https://github.com/Shuozeli/arrow-adbc-rs) | Rust | Clean-room Rust ADBC (Arrow Database Connectivity) with drivers for FlightSQL, SQLite, PostgreSQL, and MySQL |
| [prisma-rs](https://github.com/Shuozeli/prisma-rs) | Rust | Pure Rust drop-in replacement for Prisma ORM with query engine, multiple database drivers, and migration engine |

### Developer Tools

| Project | Language | Description |
|---------|----------|-------------|
| [issue-tracker-lite](https://github.com/Shuozeli/issue-tracker-lite) | Rust | Rebuild of Google Issue Tracker as a Rust gRPC server + CLI client + React web UI, backed by SQLite via Quiver ORM |
| [pwright](https://github.com/Shuozeli/pwright) | Rust | Lightweight CLI for browser automation via Chrome DevTools Protocol using a snapshot-act-snapshot workflow |
| [grpcurl-rs](https://github.com/Shuozeli/grpcurl-rs) | Rust | Rust port of grpcurl -- CLI for interacting with gRPC servers, supporting reflection, all four RPC types, and TLS/mTLS |
| [beu](https://github.com/Shuozeli/beu) | Rust | Persistent session memory CLI for AI coding agents (Claude Code, Gemini CLI, etc.) backed by local SQLite |