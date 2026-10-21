# Security Policy

## Reporting a vulnerability

If you discover a security vulnerability in Neo, please report it responsibly:

- **Preferred:** use GitHub's private vulnerability reporting for this repository — **Security → Report a vulnerability** on the `neo-browser` repository page (opens `https://github.com/rishabhhgit/neo-browser/security/advisories/new`).
- If private reporting is unavailable, open a minimal public issue titled `[SECURITY]` that does **not** include exploit details, and ask for a private contact channel.

Please include: affected version/commit, operating system, reproduction steps, and the impact you observed.

## Scope

Neo is a derivative of the [Servo](https://github.com/servo/servo) browser engine. Vulnerabilities may be:

1. **In Neo's shell** (`ports/servoshell/`) — preferences, bookmarks/history storage, internal `neo:` pages, chrome UI, WebDriver glue → reported here first.
2. **In the Servo engine** (`components/`, or upstream crates) — these will be triaged here and, when appropriate, coordinated with upstream Servo's own security process (https://github.com/servo/servo/security/advisories).

## Security-relevant behavior to keep in mind

- The WebDriver server (`--webdriver`) binds **all interfaces** (`0.0.0.0`) — only enable it on trusted networks.
- `--ignore-certificate-errors` disables TLS validation and is intended for local testing only.
- Neo implements **no OS-level site sandbox**; see the Security and privacy section of the README.
- There is currently **no bug-bounty program** for this project.

## Supported versions

This project tracks the `main` branch. Security fixes are applied to `main` only; there are no versioned release branches yet.
