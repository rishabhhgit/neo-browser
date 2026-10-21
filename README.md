<p align="center">
  <img src="resources/neo_logo.png" width="128" alt="Neo logo">
</p>

<h1 align="center">Neo</h1>

<p align="center">
  <strong>An independent browser — built in Rust on the Servo engine.</strong>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MPL--2.0-blue.svg" alt="License: MPL-2.0"></a>
  <img src="https://img.shields.io/badge/Rust-1.97.1-orange.svg" alt="Rust toolchain 1.97.1">
  <img src="https://img.shields.io/badge/platform-Windows%20•%20Linux%20•%20macOS-lightgrey.svg" alt="Platforms">
</p>

---

Neo is a desktop web browser whose engine is **neither Chromium, WebKit, nor Gecko**. It is built on the [Servo](https://github.com/servo/servo) engine — an independent, Rust-first browser engine — and packaged as a polished desktop application with its own identity, defaults, and user interface.

**Neo exists because browser diversity matters.** Every mainstream browser today traces back to one of three engines. Neo explores what a small, readable, fully open-source browser looks like when the whole stack — DOM, CSS, layout, networking, compositing, media, and UI — is written in Rust and easy to hack on.

> **Status:** Neo is real, buildable, and runnable software — not a mockup. It loads real websites, plays adaptive video streams, and ships working tabs, bookmarks, history, and preferences. It is also honest about its limits: some web platform features that mainstream browsers ship (see [Limitations](#-limitations)) are not implemented yet.

**Who it is for:** developers who want a readable browser codebase to learn from or build on, and enthusiasts who want to try a non-Chromium browser with a clean UI.

**Disclaimer:** Neo is an independent project derived from the Servo open-source project. It is not affiliated with, or endorsed by, Mozilla, the Servo Project, or Google.

---

## Table of contents

- [Feature overview](#feature-overview)
- [Limitations](#-limitations)
- [Screenshots](#-screenshots)
- [Technology stack](#-technology-stack)
- [Installation and setup](#-installation-and-setup)
- [Running Neo locally](#-running-neo-locally)
- [Project structure](#-project-structure)
- [Architecture and implementation](#-architecture-and-implementation)
- [Usage guide](#-usage-guide)
- [Security and privacy](#-security-and-privacy)
- [Testing and troubleshooting](#-testing-and-troubleshooting)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [License](#-license)
- [Acknowledgments](#-acknowledgments)

**Status legend used below:**  🚧 --> not implemented (planned or out of scope for now). Anything with no marker is implemented and was verified during development of this release.

---

## Feature overview

### Tabs and navigation

| Feature | What it does |
| --- | --- |
| Tab strip | Open tabs are listed in a strip with page titles, favicons, a loading indicator, and a per-tab close button. |
| Keyboard tab control | `Ctrl+T` new tab, `Ctrl+W` close tab, `Ctrl+1…8` jump to tab, `Ctrl+9` last tab, `Ctrl+PageUp/PageDown` cycle tabs. |
| Address bar | Accepts full URLs, bare domains (`example.com` → `https://example.com`), local file paths, or falls back to a web search using the configured search engine. Covered by unit tests in `ports/servoshell/test.rs`. |
| Back / forward / reload | Toolbar buttons plus `Alt+←` / `Alt+→`, mouse back/forward buttons, `Ctrl+R` and `F5`. |
| Reload all tabs | Toolbar "reload all" command re-loads every open tab. |
| Smart startup URL | Launching `neo.exe <url-or-query>` opens that argument; with no argument it opens the configured homepage (default: `https://www.google.com`). |
| New tab page | Neo-branded new tab (`neo:newtab`) with the Neo logo, a Google search box, and a Home link. Opened by the toolbar/tabs new-tab button and `Ctrl+T`. |

### Bookmarks and history

| Feature | What it does |
| --- | --- |
| Bookmark toggle | Star button in the toolbar bookmarks/un-bookmarks the current page. |
| Bookmarks side panel | Toggle panel listing saved bookmarks; clicking one navigates. |
| Persistent bookmarks | Stored as JSON in Neo's config directory (`bookmarks.json`). |
| Browsing history | Visits are recorded automatically (URL + title + time). |
| History side panel | Toggle panel with a **live text filter** over your history; click an entry to visit it. |
| Persistent history | Stored as JSON in Neo's config directory (`history.json`). |

### Settings and preferences

| Feature | What it does |
| --- | --- |
| Preferences page | `neo:preferences` — toggles for experimental web-platform features, an HTTP-cache switch, and a **user-agent selector** (Neo / Firefox / Chrome presets plus a free-form custom value). |
| Preferences toggle | Toolbar `?` button instantly enables/disables the experimental preference set. |
| Raw config view | `neo:config` renders the active preference JSON. |
| Homepage + search engine | Preferences `homepage` and `searchpage` (with the `%s` query placeholder). Defaults: Google for both. |
| Configurable via CLI | Preferences can be overridden on the command line (`--prefs-file`, and documented `shell.*` preference keys) — see `--help`. |
| Command-line switches | Layout thread count (`-y`), headless mode (`-z`), devtools (`--devtools`), experimental features (`--enable-experimental-web-platform-features`), certificate-error override, hosts-file override, device-pixel-ratio override, and more. |

### Media and graphics

| Feature | What it does |
| --- | --- |
| HTML5 `<video>` / `<audio>` | Media playback through a GStreamer backend integrated into the engine. |
| Direct files | Progressive files such as MP4 play via `<video src="…mp4">`. |
| HLS adaptive streaming | Apple-style `.m3u8` master playlists play through `<video>` — **without needing Media Source Extensions**. Verified with Apple's 1080p `img_bipbop` test stream. |
| DASH adaptive streaming | `.mpd` manifests play through `<video>` (verified with the Akamai `bbb_30fps` test stream). |
| Multiple simultaneous streams | Verified: three concurrent `<video>` elements (HLS + DASH + MP4) all reach `readyState = 4` and advance playback time. |
| GPU compositing | WebRender compositing over OpenGL/ANGLE (observed running on Direct3D 11 via ANGLE). |
| WebGL / WebGPU (Experimental) / WebXR | Enabled by default in the feature set; maturity varies by site — these are the engine's experimental web features. |
| Gamepad API (Experimental) | Gamepad feature enabled by default (`gilrs` backend). |
| Rendering debug overlays | `F9` WebRender frame capture, `F10` render-target debug, `F11` texture-cache debug, `F12` profiler overlay. |

### Developer tooling and automation

| Feature | What it does |
| --- | --- |
| W3C WebDriver | `--webdriver <port>` starts a WebDriver server (session creation, navigation, JS execution, window handles, screenshots). Any W3C client (Selenium, `chromedriver`-style tools, plain HTTP) can drive Neo. Used extensively during development of this fork. |
| Remote DevTools | `--devtools <address:port>` (e.g. `127.0.0.1:7000`) starts Servo's remote devtools server. Feature is compiled in by default; not smoke-tested in the current build. |
| Headless mode 🚧 | `-z` / `--headless` flag exists in the CLI (inherited from Servo). |
| Logging | Standard `env_logger` filtering, e.g. `RUST_LOG=info` (verified) or module filters such as `RUST_LOG=net=debug`. |

### Interface and platform

| Feature | What it does |
| --- | --- |
| Custom chrome | Toolbar, tab strip, side panels, and dialogs are rendered with `egui` inside the shell — distinct from the page content, which Servo renders. |
| Site dialogs | Alert/confirm/prompt boxes, file pickers, HTTP authentication dialogs, permission requests, select-element popups, color pickers, and right-click context menus are all implemented in the shell UI. |
| `neo:` internal pages | `neo:newtab`, `neo:preferences`, `neo:config`, `neo:experimental-preferences`, `neo:default-user-agent` — served by a built-in protocol handler. |
| Branding | `neo.exe`, Neo window titles, Neo application identity on Windows, Neo icon set (`resources/neo.ico`, `resources/neo_logo.png`). |
| Accessibility | AccessKit integration is compiled into the shell (screen-reader tree exposure comes from the engine's accessibility support). |
| WebDriver-compatible control of dialogs | Embedder controls are virtualized under WebDriver so automation can accept prompts. |

---

## 🚧 Limitations

Neo is under active development. This is the **complete, honest list** of everything Neo cannot do today, everything that is rough, and every boundary it has — collected in one place so this README never over-promises. Planned fixes are tracked in the [Roadmap](#-roadmap).

### Web platform and media

- **No YouTube playback.** YouTube's player depends on Media Source Extensions (see next bullet) and cannot work in this engine today.
- **No Media Source Extensions (MSE).** Sites whose players feed bytes through MSE will not play video. This is an engine-level gap in the script layer requiring substantial work — not a configuration problem.
- **JavaScript player libraries therefore fail.** `hls.js`, `dash.js`, Shaka Player, and similar MSE-driven libraries do not work — even though Neo's *native* `<video>` handling of `.m3u8`/`.mpd` URLs (no MSE involved) is fully functional.
- **No Encrypted Media Extensions (EME) / DRM.** Netflix, Disney+, Spotify web player, and every Widevine/FairPlay-protected stream cannot play protected content in any build of this engine.
- **No WebRTC for web pages.** The engine tree carries experimental WebRTC plumbing, but `RTCPeerConnection` is not exposed to page script — video calls, getUserMedia-based apps, and WebRTC data channels do not work.
- **No WebExtensions.** There is no extension system: ad blockers, password managers, and other add-ons cannot be installed.
- **Experimental web features are immature.** WebGPU 🧪, WebXR 🧪, Gamepad API 🧪, and AccessKit accessibility 🧪 are compiled in and enabled by default, but compatibility varies by site and none are production-grade.

### Missing everyday browser features

- **No download manager.** Navigating to a file downloads nothing to disk; the HTML `download` attribute on links is an explicit `TODO` (`components/script/dom/html/textual/htmlanchorelement.rs`).
- **No find-in-page** (`Ctrl+F`) — no such feature exists in the shell or engine integration.
- **No session/tab restore.** Open tabs are not saved or re-opened across restarts.
- **No password manager, autofill, or form-data capture**, and **no account or sync system** of any kind.
- **No cookie or site-data UI.** Cookies work in the network stack (and are resettable via WebDriver), but there is no user-facing inspector.
- **No search-engine picker UI.** The search engine is changed through preferences or the CLI (`searchpage`), not from a menu.
- **No tab pinning**, tab groups, or tab drag-reordering.
- **No printing.** There is no print dialog or print path in the shell.
- **No built-in PDF viewer.** PDF links are not rendered (no PDFium integration).
- **No spellcheck.** No inline or dictionary spellchecking in form fields or elsewhere.
- **No multiple-window management UI.** Popups/`target=_blank` windows are handled by the embedder, but there is no "new window" command or window manager in the chrome.

### Reliability and behavior quirks

- **Minimize-while-playing media is under investigation.** Minimizing the window logs a `0×0` surface-resize warning (`Unable to resize to size under 1x1` from `paint_api`); long-running behavior with active video in a minimized window is still being tested.
- **Heavy pages can briefly spike memory** while textures queue up; normal browsing stays in the low hundreds of MB, but the renderer has no strict texture-queue backpressure yet.
- **Headless mode (`-z`) is untested in this fork.** The flag exists (inherited from Servo) but no verification has been performed here.

### Platform, testing, and project scope

- **Windows is the only verified platform.** Built and tested on **Windows 11 x86_64 (MSVC)** only; Servo's macOS/Linux/Android code paths are retained but untested in this fork.
- **Linux config directory is still upstream's** `~/.config/servo/default/` — not yet renamed to Neo.
- **The WPT suite ships but has not been run or re-baselined** for this fork, and there is **no CI yet** — claims here rest on manual verification and the shell's unit tests only.
- **Remote DevTools is compiled in but not smoke-tested** in the current build.
- **Site compatibility is not systematically tested.** Verified against Google (homepage/search/new tab), Apple and Akamai HLS/DASH test streams, and general pages; no broad site-by-site status pass has been done.
- **Builds are unsigned** — occasional Windows Defender/AV heuristics may flag fresh builds; build from source as shown above.

### Security boundaries

(Expanded in [Security and privacy](#-security-and-privacy) — listed here so the "everything" list is complete:)

- **No Chromium-style OS sandbox** — Neo relies on Rust memory safety and engine architecture, not OS-level site isolation.
- **The WebDriver server binds `0.0.0.0`** (all interfaces) when enabled — safe only on trusted networks.
- **Defaults contact Google** — the default homepage and search engine are Google, so a stock launch talks to Google.

---

## 📸 Screenshots

Captured from the **actual build** running on Windows 11 — no mockups or fabricated imagery.

**New tab page (`neo:newtab`) — Neo logo and Google search box:**

![Neo new tab page](docs/screenshots/new-tab.png)

**Default homepage on first launch (Google):**

![Neo Google homepage](docs/screenshots/google-homepage.png)

**A real website (Wikipedia) — full rendering with favicon and live tab title:**

![Wikipedia article rendered in Neo](docs/screenshots/web-page.png)

The project logo is embedded at the top of this README: [`resources/neo_logo.png`](resources/neo_logo.png). To contribute more screenshots (e.g. `neo:preferences`, side panels, video playback): run Neo, capture the window, save PNGs under `docs/screenshots/`, and reference them here.

---

## 🧰 Technology stack

| Layer | Technology |
| --- | --- |
| Language | **Rust** (toolchain pinned to **1.97.1** via `rust-toolchain.toml`) |
| Browser engine | **Servo** — this repository contains the engine (`components/`) and the application shell (`ports/servoshell/`) |
| JavaScript engine | **SpiderMonkey** (fetched as a build dependency of Servo's `js` crate) |
| CSS / style system | **Stylo** (Servo's selector-matching / cascade engine, consumed as the `style` crate) |
| DOM & scripting | `components/script` — DOM implementation, event loop, JS bindings |
| Networking | `components/net` — HTTP loader, cookies, HSTS, cache |
| Compositor | **WebRender** (GPU renderer) over **surfman** / **ANGLE** (OpenGL ES on Direct3D 11 on Windows) |
| Media playback | **GStreamer** (`media-gstreamer` feature) — demux/decoding for MP4, HLS, DASH |
| Browser chrome UI | **egui** (toolbar, tabs, side panels, dialogs) in a **winit** event loop |
| CLI parsing | **bpaf** |
| Persistence | `serde` / `serde_json` — `prefs.json`, `history.json`, `bookmarks.json` |
| Bindings | `bindgen` (requires `LIBCLANG_PATH` pointing at LLVM libclang) |
| Build system | **Cargo** (primary, verified) + bundled upstream **mach** Python tooling (optional) |
| Test suites | Rust unit tests (`ports/servoshell/test.rs`, `prefs` tests) + upstream **WPT** suite under `tests/` |

---

## 🛠 Installation and setup

### Prerequisites (Windows, primary platform)

1. **Rust toolchain** — install [rustup](https://rustup.rs/); the pinned toolchain (`1.97.1` plus `rustfmt`, `clippy`, etc.) is installed automatically on first build.
2. **Visual Studio 2022 Build Tools** with the *Desktop development with C++* workload (MSVC v143 + a Windows 10/11 SDK).
3. **LLVM** — provides `libclang` (for `bindgen`) and `lld-link`. Install from [llvm.org](https://llvm.org/releases/) and note its install path.
4. **GStreamer 1.x** — MSVC x86_64 *development* and *runtime* installers from [gstreamer.freedesktop.org](https://gstreamer.freedesktop.org/download/). Note the install path (e.g. `C:\gstreamer\1.0\msvc_x86_64`).
5. **Python 3** — only needed if you use the optional `mach` tooling (tidy/lint helpers).

### Clone

```powershell
git clone https://github.com/rishabhhgit/neo-browser.git
cd neo-browser
```

### Build with Cargo (verified path)

Open an **"x64 Native Tools Command Prompt for VS 2022"** (this pre-sets `INCLUDE`/`LIB`/`PATH` for MSVC), or a plain PowerShell with the variables below set yourself:

```powershell
# Point these at YOUR install locations — do not copy these example paths blindly.
$env:LIBCLANG_PATH       = "C:\Program Files\LLVM\bin"
$env:GSTREAMER_1_0_ROOT_MSVC_X86_64 = "C:\gstreamer\1.0\msvc_x86_64"
$env:PKG_CONFIG_PATH     = "$env:GSTREAMER_1_0_ROOT_MSVC_X86_64\lib\pkgconfig"

cargo build --release -p servoshell
```

Notes:

- Default features (`default_base`, `jemalloc`, `media-gstreamer`) are exactly what Neo ships with. The longer equivalent form is `cargo build --release -p servoshell --no-default-features --features default_base,jemalloc,media-gstreamer`.
- If `lld-link.exe` is not on `PATH`, either add LLVM's `bin` directory or set `$env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER` to the full path of MSVC's `link.exe`.
- Optional: set `$env:CARGO_TARGET_DIR` to relocate build output (development builds of Neo used an external target directory to keep the source tree clean).

### Build with mach (optional, upstream Servo tooling)

```powershell
./mach build --release
```

This uses Servo's bundled Python build orchestration, which also provisions bundled dependencies (including GStreamer/ANGLE under `target/dependencies/`). Requires Python 3 and network access on first run.

---

## ▶️ Running Neo locally

```powershell
# after a cargo build (adjust if you set CARGO_TARGET_DIR)
target\release\neo.exe

# open a specific page at startup
target\release\neo.exe https://example.com

# start the WebDriver automation endpoint on port 4444
target\release\neo.exe --webdriver 4444

# all CLI options
target\release\neo.exe --help
```

**Expected result:** a window opens on the homepage (Google by default) with the tab strip, address bar, toolbar (back/forward/reload, star, history/bookmarks panels), and the Neo new-tab page available from `Ctrl+T`.

### Runtime libraries

A `cargo`-built `neo.exe` needs these **at runtime**:

- **GStreamer runtime** — the `bin` directory of your GStreamer install on `PATH`, or its DLLs copied next to `neo.exe`. HLS/DASH additionally need the corresponding plugin DLLs (the plugins used during development of this fork were: `gsthls`, `gstdash`, `gstfmp4`, `gstmpegtsdemux`, `gstsoup`, plus their transitive dependencies such as `soup-2.4-1.dll`, `sqlite3-0.dll`, `psl-5.dll`).
- **ANGLE libraries** — `libEGL.dll` / `libGLESv2.dll` (provided by the mach dependency bundle or copied next to the executable).

### Common setup issues

| Symptom | Fix |
| --- | --- |
| `failed to run custom build command` / `pkg-config` errors for GStreamer | Set `GSTREAMER_1_0_ROOT_MSVC_X86_64` and `PKG_CONFIG_PATH` (see above); use the **MSVC** GStreamer build, not MinGW. |
| `libclang` / bindgen errors | Set `LIBCLANG_PATH` to LLVM's `bin` directory. |
| `link.exe not found` or LNK errors | Build from the *x64 Native Tools* prompt, or set `CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER`. |
| `Access is denied` replacing `neo.exe` during build | A Neo instance is still running — close it and rebuild. |
| Window opens but video never plays | GStreamer runtime DLLs/plugins are missing — install the GStreamer runtime or copy its `bin` contents next to `neo.exe`. |
| Black/blank rendering, GL errors | Ensure the ANGLE DLLs sit next to `neo.exe` (or on `PATH`). |
| Windows Defender / AV quarantine | Unsigned debug builds occasionally trip heuristics; build locally from source as shown. |

---

## 📁 Project structure

```
neo-browser/
├── Cargo.toml, Cargo.lock        # Cargo workspace definition + locked dependency graph
├── rust-toolchain.toml           # pinned Rust 1.97.1 toolchain + components
├── rustfmt.toml, servo-tidy.toml # formatting / lint configuration
├── mach, mach.ps1, mach.bat      # optional upstream Python build entry points
├── LICENSE, CODE_OF_CONDUCT.md,
│   CONTRIBUTING.md, SECURITY.md  # project metadata (MPL-2.0)
│
├── ports/
│   └── servoshell/               # ★ Neo's browser application (compiles to `neo.exe`)
│       ├── main.rs               #   process entry point (binary target `neo`)
│       ├── lib.rs                #   shell library: logging, panic hook, orchestration
│       ├── prefs.rs              #   CLI flags (bpaf) + preferences incl. Neo defaults
│       ├── parser.rs             #   address-bar input → URL/search resolution
│       ├── browser_data.rs       #   bookmarks.json / history.json persistence
│       ├── window.rs             #   windows, tab (webview) collection, UI commands
│       ├── running_app_state.rs  #   app-wide state, webview lifecycle, delegates
│       ├── webdriver.rs          #   WebDriver embedder-side message handling
│       └── desktop/
│           ├── app.rs            #   winit ApplicationHandler, `neo:` protocol registration
│           ├── headed_window.rs  #   window creation, input, keyboard shortcuts
│           ├── gui.rs            #   egui toolbar / tab strip / side panels / dialogs
│           ├── protocols/        #   `neo:`, `resource:`, URL-info protocol handlers
│           └── event_loop.rs     #   main event loop glue
│
├── components/                   # ★ Servo engine (first-party crates)
│   ├── script/                   #   DOM implementation, events, JS bindings
│   ├── script_bindings/          #   generated binding glue
│   ├── layout/                   #   layout engine
│   ├── net/                      #   HTTP stack, cookies, HSTS, caching
│   ├── constellation/            #   pipelines/tabs orchestration ("browser process" core)
│   ├── paint/                    #   painting & webview rendering glue
│   ├── media/                    #   GStreamer player backends (HLS/DASH/MP4)
│   ├── webdriver_server/         #   W3C WebDriver HTTP server
│   ├── servo/                    #   embedder-facing API used by the shell
│   ├── webgl/, webgpu/, webvtt/, webxr/, canvas/, svg/, xpath/, …
│   └── devtools/, metrics/, profile/, fonts/, storage/, …
│
├── resources/                    # Neo branding + shipped web resources
│   ├── neo.ico, neo_64.png,
│   ├── neo_logo.png              #   logo used in-app and in this README
│   └── resource_protocol/        #   newtab.html, preferences.html, css (served via resource:)
│
├── tests/                        # upstream Servo test infrastructure
│   └── wpt/                      #   Web Platform Tests suite + reftests (large)
├── python/                       # mach / servo-tidy tooling
├── docs/                         # additional documentation (incl. upstream Servo docs)
├── support/, ffi/, third_party/  # support crates, FFI shims, vendored helpers
└── .github/                      # issue & pull-request templates
```

---

## 🏗 Architecture and implementation


Key ideas:

- **The shell is the browser.** `ports/servoshell` is Neo's application: it owns windows, tabs, chrome rendering (egui), keyboard shortcuts, persistence, and the `neo:` protocol. The engine never draws the toolbar; it only renders page content inside the shell's windows.
- **Webviews map to tabs.** Each tab is a top-level `WebView` in a `WebViewCollection` (`ports/servoshell/window.rs`). Tab UI commands (`Go`, `Back`, `Forward`, `Reload`, `NewWebView`, `CloseWebView`) flow through a single command queue.
- **`neo:` protocol.** A custom protocol handler (`desktop/protocols/servo.rs`) maps shortcuts such as `neo:newtab` and `neo:preferences` onto bundled `resource:` files (`resources/resource_protocol/*.html`). Privileged paths (`config`, `preferences`) are separated from generally fetchable ones.
- **Preferences live in two layers.** Layer 1: `bpaf` CLI flags parsed in `prefs.rs`. Layer 2: JSON preference files in the config directory (Windows: `%APPDATA%\Neo\prefs.json`), merged at startup. Neo's own defaults (homepage = Google, search = Google) are defined in `prefs.rs`.
- **Address-bar logic is testable and tested.** `parser.rs` resolves input to a URL in this order: valid URL → local file path → domain-like string (`https://` prepended) → search-engine template substitution (`%s`). Unit tests live in `ports/servoshell/test.rs`.
- **Media pipeline.** GStreamer elements demux/decode streams into frame buffers; frames are handed to the script/media layer as `VideoFrameUpdated` events, damage the page, and are composited by WebRender. This is why **HLS/DASH work without MSE**: streams are handled natively, not through JS-driven byte ranges.
- **WebDriver.** `components/webdriver_server` speaks W3C WebDriver over HTTP; messages are dispatched into the embedder (`ports/servoshell/webdriver.rs`) and into the constellation/script threads.

---

## 🕹 Usage guide

### Everyday controls

| Action | Control |
| --- | --- |
| New tab | `Ctrl+T` (or the new-tab button) |
| Close tab | `Ctrl+W` (or the tab's ✕) |
| Switch tabs | `Ctrl+1…9`, `Ctrl+PageUp` / `Ctrl+PageDown` |
| Go back / forward | `Alt+←` / `Alt+→`, toolbar buttons, mouse back/forward buttons |
| Reload | `Ctrl+R` or `F5` (toolbar reload reloads all tabs) |
| Zoom in / out / reset | `Ctrl+=` (or `Ctrl++`), `Ctrl+-`, `Ctrl+0` (±10 % steps) |
| Quit | `Ctrl+Q` |
| Bookmark current page | Toolbar star toggle |
| Show history / bookmarks panels | Toolbar clock / bookmark buttons |
| Search your history | Open the History panel and type in its filter box |
| Open preferences | Navigate to `neo:preferences` in the address bar |
| Open raw config | Navigate to `neo:config` |
| WebRender debugging | `F9` capture · `F10` render targets · `F11` texture cache · `F12` profiler |

### Command-line reference (selected)

| Flag | Purpose |
| --- | --- |
| `[URL]` | Startup URL or bare search query |
| `--webdriver <port>` | Enable the W3C WebDriver server (see security note) |
| `--devtools <addr:port>` | Start the remote devtools server (e.g. `127.0.0.1:7000`) |
| `--enable-experimental-web-platform-features` | Turn on Servo's experimental preference set at startup |
| `-y, --layout-threads <n>` | Number of layout threads (e.g. `-y 2` on small machines) |
| `-z, --headless` | Headless mode |
| `--ignore-certificate-errors` | ⚠ Disable TLS certificate validation (testing only) |
| `--host-file <path>` | Custom hosts file (unless `HOST_FILE` env var is set) |
| `--device-pixel-ratio <f>` | Force a device pixel ratio |
| `-x, --exit` | Exit after the page reaches a stable painted state (scripted testing) |
| `--prefs-file <path>` | Merge additional preferences from a JSON file |

Run `neo.exe --help` for the complete, authoritative list.

### Where Neo stores your data

| Platform | Config directory |
| --- | --- |
| Windows | `%APPDATA%\Neo\` — `prefs.json`, `history.json`, `bookmarks.json`, HTTP cache |
| macOS | `~/Library/Application Support/Neo/` |
| Linux | `~/.config/servo/default/` (upstream Servo path — not yet renamed in this fork) |

---

## 🔐 Security and privacy

Verified behavior — with clear boundaries:

**What Neo does:**

- **No telemetry, analytics, or crash-reporting of its own.** No such service is referenced anywhere in the shell or network stack.
- **Your data stays local.** History, bookmarks, and preferences are plain JSON files in the config directory above. There is no account system and no sync.
- **TLS certificate validation is on by default.** `--ignore-certificate-errors` must be passed explicitly to disable it (never do this for normal browsing).
- **Standard web isolation building blocks:** cookies are partitioned per the network stack's cookie storage, and HTTPS/HSTS handling is implemented (`components/net`).

**What you should know:**

- ⚠ **Neo's default homepage and search engine are Google**, so a default launch contacts Google. Change `homepage`/`searchpage` in preferences, or pass a startup URL, to avoid this.
- ⚠ **The WebDriver server binds `0.0.0.0`** (all interfaces) when `--webdriver` is enabled. Only enable it on trusted networks, behind a firewall, or bind it to a safe environment. Disable it when you are not automating.
- ⚠ **There is no Chromium-style OS sandbox.** Neo relies on Rust's memory-safety guarantees and engine architecture, not on an OS-level site isolation sandbox. Treat browsing untrusted sites with appropriate caution.
- ⚠ **Neo is a derivative of Servo** and inherits its security properties and its bug surface. Report issues here first (see [SECURITY.md](SECURITY.md)); engine-level issues may be coordinated upstream with the Servo project.

---

## 🧪 Testing and troubleshooting

### Unit tests (verified on this codebase)

```powershell
cargo test --release -p servoshell
```

**Actual result (Windows 11, x86_64, MSVC, release profile):** `9 passed; 0 failed; 0 ignored` covering address-bar/URL parsing and preference/CLI handling (`test::test_cmdline_and_location_bar_url`, `prefs::test_servoshell_cmd`, …), plus `0` tests in the binary target and `0` doc-tests.

> Use `--release` to reuse the already-built dependency graph; a debug-profile test build compiles the entire engine a second time.

### Web platform tests (upstream suite)

Servo's WPT infrastructure lives in `tests/wpt/` and is driven by the upstream tooling:

```powershell
./mach test-wpt tests/wpt/mozilla/tests/mozilla.rs   # example subset
```

This requires the Python `mach` environment to be set up. WPT has **not** been run or re-baselined for this fork, and no CI claims are made about it.

### Manual smoke checklist (what was actually verified for this release)

1. Launch with no arguments → window opens on `https://www.google.com` (page title `Google`).
2. `Ctrl+T` → Neo new tab renders logo + Google search box; searching navigates to Google results.
3. Navigate to an HLS URL inside a `<video>` element → playback starts, `currentTime` advances.
4. Navigate to a DASH `.mpd` inside `<video>` → playback starts (readyState 4).
5. Three videos (HLS + DASH + MP4) playing simultaneously → all advance.
6. Bookmark a page, restart → bookmark persists; history panel filters entries.

### Debug logging

```powershell
$env:RUST_LOG = "info"      # or e.g. "net=debug,script=debug"
target\release\neo.exe
```

### Known rough edges

The complete list — including the minimize warning, memory-spike behavior, and every other gap — lives in one place: [Limitations](#-limitations).

---

## 🗺 Roadmap

Planned work — **none of this is implemented yet**:

- [ ] **Session restore** — persist and re-open tabs across restarts.
- [ ] **Find in page** (`Ctrl+F`) with match highlighting.
- [ ] **Downloads** — at minimum honor the HTML `download` attribute; then a download manager UI.
- [ ] **Minimize/occlusion robustness** — zero-size surface handling and media behavior while minimized.
- [ ] **Site compatibility pass** — structured testing against popular sites (Wikipedia, GitHub, Hacker News, news sites) with a public status page.
- [ ] **CI** — build + unit-test workflow for Windows (and Linux, if capacity allows).
- [ ] **Release binaries** — tagged releases with prebuilt `neo.exe` on the Releases page.
- [ ] **MSE evaluation** — scoping study for Media Source Extensions in the script layer (large effort; no commitment yet).
- [ ] **Cookie/site-data UI** — a basic inspector for cookies and site storage.
- [ ] **Search-engine picker** — a UI for the existing `searchpage` preference (currently pref/CLI only).

---

## 🤝 Contributing

Contributions are welcome — bug reports, fixes, docs, and features alike.

1. **Report bugs** via [Issues](https://github.com/rishabhhgit/neo-browser/issues) using the bug-report template. Include OS version, build command, `RUST_LOG` output if relevant, and reproduction steps.
2. **Suggest features** via the feature-request template — especially notes on which limitation in this README you'd like to see lifted.
3. **Submit pull requests:**
   - Fork, branch, and keep changes focused.
   - Run `cargo fmt` (config in `rustfmt.toml`) and the project's tidy checks (`servo-tidy.toml`, via `python mach test-tidy` if your Python environment is set up).
   - Run `cargo test --release -p servoshell` and describe the results in the PR.
   - Fill in [PULL_REQUEST_TEMPLATE.md](PULL_REQUEST_TEMPLATE.md).
4. **Engine-level work** (script/layout/net) is the heavy end of the pool — read `docs/` and the [Servo Book](https://book.servo.org/) first; issues labeled `servo-engine` will be used for cross-cutting engine tasks.

Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before participating.

---

## 📄 License

This project is licensed under the **Mozilla Public License 2.0** — see [LICENSE](LICENSE).

Neo is a **derivative work of the Servo Project** (https://servo.org/), also MPL-2.0. All original Servo source files retain their MPL headers; modifications made for Neo are distributed under the same license. Portions of this repository (WPT test material, `third_party/`) carry their own upstream licenses as marked in those files.

---

## 🙏 Acknowledgments

Neo stands on the shoulders of:

- **[Servo](https://github.com/servo/servo)** — the browser engine this project is built on, developed by the Servo community and its contributors.
- **[Mozilla](https://www.mozilla.org/)** — originator of Stylo, SpiderMonkey, and much of the technology Servo continues.
- **[WebRender](https://github.com/servo/webrender)** — the GPU compositor.
- **[GStreamer](https://gstreamer.freedesktop.org/)** — the media pipeline behind Neo's video playback.
- **[egui](https://github.com/emilk/egui)** — the immediate-mode GUI library used for Neo's browser chrome.
- **[winit](https://github.com/rust-windowing/winit)**, **[surfman](https://github.com/servo/surfman)**, **[ANGLE](https://chromium.googlesource.com/angle/angle)**, **[bpaf](https://github.com/pacak/bpaf)**, and the hundreds of crates in `Cargo.lock`.
- The **Web Platform Tests** project and its contributors.

**Links:** [Repository](https://github.com/rishabhhgit/neo-browser) · [Issues](https://github.com/rishabhhgit/neo-browser/issues) · [Releases](https://github.com/rishabhhgit/neo-browser/releases) · [Servo upstream](https://github.com/servo/servo) · [Servo Book](https://book.servo.org/)

---

<p align="center">
  <strong>Neo is an independent browser because the web deserves more than three engines.</strong><br>
  Build it, run it, break it, and tell us what it should become next.
</p>
