# CLAUDE.md - Amazon Q Developer CLI

This document provides a comprehensive overview of the Amazon Q Developer CLI project for AI assistants working with this codebase.

## Project Overview

**Amazon Q Developer CLI** (formerly CodeWhisperer CLI) is a sophisticated Rust-based command-line interface that brings AI-powered development assistance directly to the terminal. It enables developers to interact with Amazon Q's AI capabilities through a chat interface, execute agent-based workflows, and leverage local knowledge management.

- **Version**: 1.19.7
- **Language**: Rust (Edition 2024, Toolchain 1.87.0)
- **License**: MIT OR Apache-2.0
- **Main Binary**: `q` (compiled from `chat_cli`)

## Architecture Overview

### High-Level Structure

```
┌─────────────────────────────────────────────────┐
│          chat-cli (Main CLI Binary)              │
│  Command parsing, auth, telemetry, DB            │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────┴──────────────────────────────────┐
│              agent (Agent Runtime)               │
│  Tool execution, permissions, MCP integration    │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────┴──────────────────────────────────┐
│         chat-cli-ui (Terminal UI)                │
│  TUI components using ratatui                    │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────┴──────────────────────────────────┐
│      AWS Service Client Crates                   │
│  CodeWhisperer, Q Developer, Telemetry           │
└──────────────────────────────────────────────────┘
```

## Workspace Structure

This is a Cargo workspace with 10 member crates:

### Core Crates

1. **`crates/chat-cli/`** - Main binary (~15,534 LOC)
   - Entry point: `src/main.rs`
   - Command definitions: `src/cli/`
   - Authentication: `src/auth/`
   - Database: `src/database/` (SQLite)
   - API clients: `src/api_client/`
   - Telemetry: `src/telemetry/`

2. **`crates/agent/`** - Agent execution engine
   - Agent runtime: `src/agent/mod.rs`
   - Built-in tools: `src/agent/tools/`
   - Permission system: `src/agent/permissions/`
   - MCP integration: `src/agent/tools/mcp.rs`
   - Hooks: `src/agent/hooks/`

3. **`crates/chat-cli-ui/`** - Terminal UI components
   - Built on ratatui framework
   - Platform-specific event handling

### AWS Client Crates (Auto-Generated)

4. **`crates/amzn-codewhisperer-client/`** - CodeWhisperer API client
5. **`crates/amzn-codewhisperer-streaming-client/`** - Streaming variant
6. **`crates/amzn-qdeveloper-streaming-client/`** - Q Developer streaming
7. **`crates/amzn-consolas-client/`** - Internal AWS service client
8. **`crates/amzn-toolkit-telemetry-client/`** - Telemetry service

### Supporting Crates

9. **`crates/aws-toolkit-telemetry-definitions/`** - Telemetry schema
   - Build script generates types from `telemetry_definitions.json`

10. **`crates/semantic-search-client/`** - Local semantic search
    - BM25 (lexical) + HNSW/all-MiniLM-L6-v2 (semantic)
    - PDF text extraction
    - Powers `/knowledge` feature

## Key Features

### 1. Agent System
- **Configuration**: JSON-based agent definitions in `~/.q/agents/*.json`
- **Tools**: Built-in (fs_read, fs_write, execute_cmd, grep, etc.) + MCP tools
- **Permissions**: Declarative allow/deny rules with glob patterns
- **Hooks**: SessionStart, SessionEnd (extensible)
- **Resources**: File and directory injection into agent context

### 2. Model Context Protocol (MCP)
- Supports MCP servers via `rmcp` crate
- Multiple transports: SSE, HTTP streaming, child process
- Tool aliasing and namespacing
- Resource and prompt exposure

### 3. Knowledge Management (Beta)
- Local-first knowledge base
- Dual indexing: BM25 (fast) + semantic embeddings (quality)
- Background indexing
- SQLite storage

### 4. Authentication
- AWS SSO integration
- Identity Center (IdC) support
- Profile management
- Token persistence

## Development Workflow

### Prerequisites
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup toolchain install nightly
cargo install typos-cli
```

### Common Commands
```bash
# Run the CLI
cargo run --bin chat_cli

# Run with subcommand
cargo run --bin chat_cli -- login
cargo run --bin chat_cli -- chat

# Testing
cargo test

# Linting
cargo clippy

# Formatting (nightly required)
cargo +nightly fmt

# Spell checking
typos
```

### Build System
- **Build scripts**: `scripts/build.py`, `scripts/build-macos.sh`
- **Cross-compilation**: Configured in `Cross.toml`
- **Code generation**: Build-time (telemetry types, feed.json)
- **Release profile**: Debug symbols enabled, incremental builds, no LTO

## Important Patterns

### 1. Agent Configuration as Data
Agents are defined in JSON, not code. Example structure:
```json
{
  "name": "my-agent",
  "prompt": "You are a helpful assistant...",
  "tools": ["fs_read", "fs_write"],
  "permissions": {
    "fs_read": {"allow": ["src/**"]},
    "execute_cmd": {"deny": ["*"]}
  }
}
```

### 2. Permission-Based Security
- Tools require explicit permission grants
- Supports auto-approve, prompt, auto-deny modes
- Glob patterns for path matching
- Regex for command matching

### 3. Streaming Architecture
- Event-driven responses from AWS services
- Progressive terminal rendering
- Cancellable operations (Ctrl+C handling)

### 4. Workspace Dependency Management
- Single source of truth in root `Cargo.toml`
- Shared versions via `[workspace.dependencies]`
- Unified linting rules

### 5. Error Handling
- `eyre` for error propagation
- `color-eyre` for beautiful terminal errors
- `thiserror` for custom error types

## Key Technologies

### Core Stack
- **Async Runtime**: Tokio (multi-threaded)
- **CLI Framework**: clap 4.5
- **Terminal UI**: ratatui 0.29 + crossterm 0.28
- **HTTP Client**: reqwest 0.12 (rustls-tls)
- **Database**: rusqlite 0.32 + r2d2 (connection pooling)

### AWS Integration
- **SDK**: aws-sdk-* crates
- **Smithy**: aws-smithy-* runtime
- **Streaming**: tokio-tungstenite, SSE

### AI/ML (Semantic Search)
- **Framework**: candle-core 0.9
- **Model**: all-MiniLM-L6-v2 (sentence transformers)
- **Vector Search**: hnsw_rs 0.3
- **Lexical Search**: bm25 2.3

### MCP Integration
- **Library**: rmcp 0.8
- **Transports**: SSE, HTTP streamable, child process
- **Middleware**: Tower

## File Navigation Guide

### Finding Components

| What | Where |
|------|-------|
| Main entry point | `crates/chat-cli/src/main.rs` |
| CLI commands | `crates/chat-cli/src/cli/*.rs` |
| Agent runtime | `crates/agent/src/agent/mod.rs` |
| Built-in tools | `crates/agent/src/agent/tools/*.rs` |
| MCP integration | `crates/agent/src/agent/tools/mcp.rs` |
| Authentication | `crates/chat-cli/src/auth/*.rs` |
| Database schema | `crates/chat-cli/src/database/*.rs` |
| Telemetry | `crates/chat-cli/src/telemetry/*.rs` |
| Terminal UI | `crates/chat-cli-ui/src/*.rs` |
| Knowledge base | `crates/semantic-search-client/src/*.rs` |

### Configuration Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Workspace and dependency configuration |
| `rust-toolchain.toml` | Rust version specification (1.87.0) |
| `Cross.toml` | Cross-compilation configuration |
| `deny.toml` | Dependency license and security checks |
| `typos.toml` | Spell checker configuration |
| `book.toml` | Documentation book (mdBook) |

## Common Tasks

### Adding a New Built-in Tool
1. Define tool in `crates/agent/src/agent/tools/*.rs`
2. Implement tool trait
3. Register in tool registry
4. Add permission schema
5. Document in agent configuration

### Adding a New CLI Command
1. Define command in `crates/chat-cli/src/cli/*.rs`
2. Add to clap command enum
3. Implement execute method
4. Add telemetry events
5. Update documentation

### Modifying API Clients
- **DON'T** edit auto-generated clients directly
- **DO** update Smithy models and regenerate
- Clients are in `crates/amzn-*-client/`

### Adding Telemetry Events
1. Update `crates/aws-toolkit-telemetry-definitions/telemetry_definitions.json`
2. Build script regenerates types
3. Use generated types in `crates/chat-cli/src/telemetry/`

## Testing Strategy

### Test Types
- **Unit tests**: In-module tests with `#[cfg(test)]`
- **Integration tests**: `tests/` directories in each crate
- **Snapshot tests**: Using `insta` crate
- **Benchmarks**: Using `criterion` crate

### Running Tests
```bash
# All tests
cargo test

# Specific crate
cargo test -p chat-cli

# Specific test
cargo test test_name

# With logging
RUST_LOG=debug cargo test

# Update snapshots
cargo insta review
```

## Debugging Tips

### Logging
- Uses `tracing` crate
- Set `RUST_LOG=debug` for verbose output
- Logs written to `~/.q/logs/`

### Database Inspection
```bash
# SQLite DB location
~/.q/db/q.db

# Inspect schema
sqlite3 ~/.q/db/q.db ".schema"
```

### Agent Configuration
```bash
# User agents directory
~/.q/agents/

# Default agent config
~/.q/agents/default.json
```

## Security Considerations

### Built-in Protections
- **Command execution**: Requires explicit permission
- **File system access**: Glob-based allow/deny rules
- **Network requests**: Via AWS SDK only (no arbitrary HTTP)
- **Token storage**: Platform-specific secure storage (Keychain, etc.)

### Vulnerability Prevention
- No arbitrary code execution without user approval
- Path traversal protection in file operations
- Input validation on all external data
- Dependency scanning with `cargo deny`

## Platform-Specific Notes

### macOS
- Keychain integration for secure storage
- App bundle generation (`Info.plist`)
- Code signing and notarization
- Uses `objc2` for native integrations

### Linux
- XDG directory standards
- No semantic search on ARM (Candle limitation)
- AppImage and Debian packaging

### Windows
- Windows registry for settings
- Windows-specific terminal handling
- MSVC compilation

## Release Process

1. Version bump in `Cargo.toml` (workspace-level)
2. Update `CHANGELOG.md`
3. Run full test suite
4. Build platform-specific artifacts
5. Sign and notarize (macOS)
6. Upload to release infrastructure
7. Update documentation

## Useful Resources

- **Main README**: `/README.md`
- **Contributing Guide**: `/CONTRIBUTING.md`
- **Security Policy**: `/SECURITY.md`
- **Documentation**: `/docs/`
- **Scripts**: `/scripts/`

## Performance Considerations

- **Memory allocator**: Uses `mimalloc` globally
- **Parallelism**: `rayon` for data-parallel operations
- **Async**: Tokio multi-threaded runtime
- **Database**: Connection pooling with r2d2
- **Incremental builds**: Enabled in release profile

## Common Pitfalls

1. **Don't edit generated clients** - They'll be overwritten
2. **Use workspace dependencies** - Don't add versions to crate `Cargo.toml`
3. **Test on all platforms** - Platform-specific code paths exist
4. **Check licenses** - `cargo deny` enforces license policies
5. **Format with nightly** - `cargo +nightly fmt` required
6. **Update telemetry schemas** - Not just code
7. **Respect permission system** - Don't bypass tool permissions

## Getting Help

- **Issues**: File at `https://github.com/aws/amazon-q-developer-cli/issues`
- **Documentation**: See `/docs/` directory
- **Code comments**: Most modules have doc comments
- **Tests**: Often serve as usage examples

---

*This document is maintained for AI assistants. Last updated: 2025-11-17*
