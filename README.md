# Overview

As a software engineer focused on building full-stack, cloud-connected applications,
I wanted to expand my skills into systems-level programming with a language that
prioritizes memory safety and performance. Learning Rust has been a goal of mine
because of its growing presence in backend infrastructure, WebAssembly, and CLI
tooling — areas I want to move into as I develop my career.

RootedCLI is a Rust command-line tool that connects to the existing Rooted Upright
AWS backend (API Gateway, Lambda, DynamoDB, Cognito) and provides a terminal-based
interface for managing plant care. It completes a three-platform portfolio story
alongside the existing Rooted Upright web app and Flutter mobile app, all sharing
the same backend with very different clients. The CLI authenticates via AWS Cognito,
fetches a user's plant collection, displays care instructions, logs care events for
individual plants or all plants at once, and checks whether a specific plant is
overdue for watering based on a seasonal threshold.

I wrote this software to demonstrate Rust's core language features in a real-world
context rather than an isolated exercise. The CLI solves a genuine problem: logging
care for 30+ named plants one at a time through a mobile app is tedious. A single
terminal command can log care for every plant simultaneously, something neither the
web nor mobile app currently supports.

[Software Demo Video - RootedCLI](https://youtu.be/jx2Z2NPax2c)

# Development Environment

- Windows 11, PowerShell terminal
- Visual Studio Code with the rust-analyzer extension
- Rust toolchain via rustup (rustc 1.96.0, cargo 1.96.0)
- Microsoft C++ Build Tools (MSVC linker, required for Rust on Windows)
- AWS Console (Cognito, DynamoDB, API Gateway) for backend verification

**Language:** Rust (2024 edition)

**Libraries (crates):**
- `serde` + `serde_json` — JSON serialization and deserialization
- `reqwest` — async HTTP client for API calls
- `tokio` — async runtime (drives .await throughout the program)
- `base64` — decoding the Cognito JWT payload to extract the user sub
- `uuid` — generating unique IDs for new care log entries
- `chrono` — date parsing and arithmetic for the overdue check

**Testing:**
The Rooted Upright web app is not currently publicly deployed, so self-registration
is not yet available. Use the following test credentials to run the application:

* Email: testuser@rootedupright.com
* Password: RootedUpright1!

# Useful Websites

- [The Rust Programming Language (Official Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Let's Get Rusty (YouTube)](https://www.youtube.com/@letsgetrusty)
- [Serde Documentation](https://serde.rs/)
- [reqwest Documentation](https://docs.rs/reqwest)
- [tokio Documentation](https://tokio.rs/)
- [AWS Cognito InitiateAuth API Reference](https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_InitiateAuth.html)

# Future Work

- Implement overdue check for all plants, not just Andromeda, by fetching
  care logs concurrently using tokio's join_all for better performance
- Add real interactive plant selection for the overdue check rather than
  a hardcoded plant ID
- Expand seasonal watering thresholds per plant species or user-defined
  care schedules rather than a single global threshold
- Add a `due` command that lists all plants currently overdue for any
  care type, not just watering
- Support filtering the plant list by care type, overdue status, or species