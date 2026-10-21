# Contributing to Neo

Thanks for your interest in contributing to Neo — an independent browser built in Rust on the Servo engine.

## Ways to contribute

- **Bug reports** — open an issue with the bug-report template. Include your OS, the exact build command you used, reproduction steps, and any `RUST_LOG` output (e.g. `RUST_LOG=info neo.exe`).
- **Feature requests** — use the feature-request template. If it targets one of the limitations listed in the README (downloads, find-in-page, session restore, …), say so explicitly.
- **Documentation** — README fixes, screenshots (see the Screenshots section of the README), and code comments are all welcome.
- **Code** — bug fixes, features, and engine work.

## Development setup

Follow the **Installation and setup** section of [README.md](README.md#-installation-and-setup). The short version on Windows:

```powershell
# from an "x64 Native Tools Command Prompt for VS 2022"
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
$env:GSTREAMER_1_0_ROOT_MSVC_X86_64 = "C:\gstreamer\1.0\msvc_x86_64"
$env:PKG_CONFIG_PATH = "$env:GSTREAMER_1_0_ROOT_MSVC_X86_64\lib\pkgconfig"

cargo build --release -p servoshell
target\release\neo.exe
```

## Before you open a pull request

1. **Format your code:**

   ```powershell
   cargo fmt
   ```

   Formatting configuration lives in [`rustfmt.toml`](rustfmt.toml).

2. **Run the unit tests:**

   ```powershell
   cargo test --release -p servoshell
   ```

   A healthy tree reports `9 passed; 0 failed` (plus an empty binary-target suite). Add or update tests when you change URL parsing, preference handling, or other tested behavior (`ports/servoshell/test.rs`).

3. **Tidy checks** (optional but appreciated) — the upstream Servo tidy tooling is configured via [`servo-tidy.toml`](servo-tidy.toml):

   ```powershell
   python mach test-tidy
   ```

4. **Keep the diff focused.** One logical change per PR. Do not mix unrelated reformatting with behavior changes.

5. **Fill in** [PULL_REQUEST_TEMPLATE.md](PULL_REQUEST_TEMPLATE.md): describe the change, how you tested it, and link any issue it fixes.

## Where to start

| Area | Good first places |
| --- | --- |
| Browser shell (UI, shortcuts, panels) | `ports/servoshell/desktop/gui.rs`, `headed_window.rs` |
| Address bar / startup URLs | `ports/servoshell/parser.rs` (+ tests) |
| Bookmarks & history | `ports/servoshell/browser_data.rs` |
| Preferences / CLI flags | `ports/servoshell/prefs.rs` |
| Internal pages (`neo:`) | `ports/servoshell/desktop/protocols/` |
| Engine (DOM, layout, net, media) | `components/` — read the [Servo Book](https://book.servo.org/) first |

Issues labeled `good-first-issue` are the friendliest entry points.

## Reporting security issues

Please do **not** open a public issue for security vulnerabilities. See [SECURITY.md](SECURITY.md).

## Code of conduct

This project follows [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Be respectful and constructive.

## License

By contributing, you agree that your contributions will be licensed under the Mozilla Public License 2.0 (see [LICENSE](LICENSE)), the same license as the rest of the project and its upstream, Servo.
