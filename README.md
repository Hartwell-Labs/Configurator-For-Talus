<div align="center">

# Configurator for Talus

**A full-screen TUI that writes the `config.conf` your Talus agent reads at start-up.**

[![this is not the agent](https://img.shields.io/badge/this%20repo%20is%20NOT-Talus%20itself-e11d48?style=flat-square)](#-wait--where-is-talus)
[![configures](https://img.shields.io/badge/configures-Talus%20v0.8.x-4f46e5?style=flat-square)](https://github.com/Hartwell-Labs/talus-process-monitor)
[![license](https://img.shields.io/badge/license-MIT-0f172a?style=flat-square)](LICENSE)
[![binary](https://img.shields.io/badge/binary-talus--config-64748b?style=flat-square)](#install)
[![deps](https://img.shields.io/badge/network-none-059669?style=flat-square)](#security)

</div>

---

## ⚠️ Wait — where is Talus?

This repository is **not** Talus. It contains **no eBPF, no monitor, no detection, no
agent** — only a configurator, a text editor with a nice face.

> [!IMPORTANT]
> **Looking for the real thing?**
> The Talus agent — eBPF process monitor, ransomware detection, neural engine, audit
> chain, sandbox — lives here:
>
> ### → **[github.com/Hartwell-Labs/talus-process-monitor](https://github.com/Hartwell-Labs/talus-process-monitor)**
>
> You need that agent for this configurator to be useful. On its own, `talus-config`
> edits a file that nothing reads.

This repo is one binary, `talus-config`, that edits one file:

```text
~/.config/talus/config.conf          (or $TALUS_CONFIG_DIR/talus/config.conf)
```

That file is what the Talus monitor loads at every start. That is the entire product.

---

## What it looks like

```text
talus  configurator   —   values are saved once, Talus reads them at start-up
  /home/you/.config/talus/config.conf   saved
╭ SETTINGS ────────────────────────────────────────────────────────╮╭ PREVIEW ───────────────────────────────────────╮
│  KEY               VALUE                SCOPE                    ││  talus — navy background                       │
│› background        navy                 ui                       ││  1 284 events/s   up 12m                       │
│  alert_threshold   —                    monitor                  ││                                                │
│  output            —                    monitor                  ││  cargo    /src/main.rs                         │
│  filter_ext        —                    monitor                  ││  firefox  /home/x/.cache/…                     │
│  top_files         —                    monitor                  ││  rsync    /data/backup.tar                     │
│  ignore_comm       —                    monitor                  ││                                                │
│  show_ignored      —                    monitor                  ││  python3  /etc/passwd                          │
│  memlp             —                    monitor                  ││                                                │
│  memlp_checkpoint  —                    monitor                  ││  ████ ███ ███ ███ ███ ███                      │
│  bpf               —                    monitor                  ││  ok     warn   alert  media  focus  info       │
│  web               —                    enterprise               ││                                                │
│  auto_kill         —                    enterprise               ││                                                │
│  kafka_brokers     —                    enterprise               ││                                                │
│  kafka_topic       —                    enterprise               ││                                                │
│  clickhouse        —                    enterprise               ││                                                │
│  memgraph          —                    enterprise               ││                                                │
│                                                                  │╰────────────────────────────────────────────────╯
│                                                                  │╭ SELECTED ──────────────────────────────────────╮
│                                                                  ││  background   [ui]                             │
│                                                                  ││  TUI background (black, gray, white, navy)     │
│                                                                  ││                                                │
│                                                                  ││  value: navy                                   │
│                                                                  ││  allowed: black, gray, white, navy             │
╰──────────────────────────────────────────────────────────────────╯╰────────────────────────────────────────────────╯
  ↑/↓ select   enter edit   ←/→ change   u default   s save   r reset   ? keys   q quit
```

The **PREVIEW** pane is not a static mock-up: it is painted with the palette you are
choosing. Press `←`/`→` on `background` and the *whole* configurator — panels, text,
selection, status bar — repaints in the candidate theme, so you see black, gray, white
and navy side by side by pressing a key.

Built with [FrankenTUI](https://crates.io/crates/ftui), the same kernel the Talus
dashboard draws with. Not a wizard, not a questionnaire, not `dialog(1)`.

---

## Why this exists

Talus is configured with command-line flags. A flag typed into a shell lasts until that
shell closes, so a systemd unit ends up repeating every option forever — and the TUI
background has no flag at all. Editing `config.conf` by hand works, but it is a file
with sixteen keys, value constraints and a typo-intolerant parser.

So there is a configurator. Three rules shaped it:

| Rule | Consequence |
|---|---|
| **It runs once, not at boot** | Nothing in the agent launches it. A service restart picks up the saved values on its own. |
| **It never runs as root** | It only writes your own config file. No capabilities, no eBPF, no device access. |
| **Flags still win** | The file is a *default layer*. `talus-monitor --alert-threshold 5` overrides the file for that run. |

---

## Install

### Download the release binary

```bash
curl -fLO https://github.com/Hartwell-Labs/Configurator-For-Talus/releases/latest/download/talus-config
curl -fLO https://github.com/Hartwell-Labs/Configurator-For-Talus/releases/latest/download/talus-config.sha256
sha256sum -c talus-config.sha256
sudo install -m0755 talus-config /usr/local/bin/talus-config
```

### From source

```bash
git clone https://github.com/Hartwell-Labs/Configurator-For-Talus
cd Configurator-For-Talus
cargo build --release
sudo install -m0755 target/release/talus-config /usr/local/bin/talus-config
```

### With cargo

```bash
cargo install --git https://github.com/Hartwell-Labs/Configurator-For-Talus
```

### Requirements

Linux (any arch with a Rust toolchain and a terminal). No runtime dependencies, no
daemon, no config file needed before first launch. A static-ish single binary of a few
megabytes.

---

## Use

```bash
talus-config
```

| Key | Action |
|---|---|
| `↑` `↓` / `k` `j` / wheel | move the selection |
| `enter` | edit the value inline · toggle a switch · step through a fixed list |
| `←` `→` | change a fixed-list value (and repaint the preview for `background`) |
| `u` | restore the selected option to the Talus default (removes the key) |
| `s` | save the file |
| `r` | delete the file and return everything to defaults — asks `y` first |
| `?` | key map |
| `q` | quit; if anything is unsaved, a second `q` is required |
| mouse | click a row to select, click the panes, scroll the list |

Everything is also mouse-driven: the confirmations and editors included.

Nothing touches disk until `s`. Validation runs against a **copy** of the settings, so a
typo leaves the editor open with the reason in the status bar and changes nothing:

```text
  ✗ alert_threshold: expected a number, got `fifty`
  ✗ background: unknown background 'blurple' (expected one of: black, gray, white, navy)
  ✗ top_files: expected 1..=64, got 80
  ✗ web: expected HOST:PORT (for example 127.0.0.1:8080), got `8080`
```

Unknown keys in the file are a hard error too — silently ignoring a typo in a security
agent's configuration would leave you believing an option is in force when it is not.

---

## What it configures

`ui` options affect the Talus dashboard; `monitor` options affect the MIT agent;
`enterprise` options are read by the Enterprise build. The file format is shared, so the
configurator lists all of them — in the MIT agent the Enterprise keys are simply inert.

| Key | Scope | Accepts | Effect |
|---|---|---|---|
| `background` | ui | `black`, `gray`, `white`, `navy` | Whole TUI palette (default: `navy`) |
| `alert_threshold` | monitor | `0`–`1000000` | Opens/second before a verdict; `0` disables alerts |
| `output` | monitor | `tui`, `json`, `plain` | Output mode when no output flag is passed |
| `filter_ext` | monitor | `pdf`, `docx`, … | Only show file events with this extension |
| `top_files` | monitor | `1`–`64` | Rows in the TOP FILES panel |
| `ignore_comm` | monitor | comma list | Process names whose burst alerts are suppressed |
| `show_ignored` | monitor | `true`, `false` | Keep those processes visible in the tree |
| `memlp` | monitor | `true`, `false` | Enable the MeMLP neural engine (MIT, fully functional) |
| `memlp_checkpoint` | monitor | path | Model checkpoint (default `~/.local/share/talus/memlp.json`) |
| `bpf` | monitor | path | Load a specific eBPF object instead of the embedded one |
| `web` | enterprise | `HOST:PORT` | Dashboard bind address |
| `auto_kill` | enterprise | `true`, `false` | SIGKILL on a verdict |
| `kafka_brokers` | enterprise | host list | Kafka sink |
| `kafka_topic` | enterprise | name | Kafka topic |
| `clickhouse` | enterprise | URL | ClickHouse sink |
| `memgraph` | enterprise | URL | MemGraph sink |

Boolean values accept `true/false`, `yes/no`, `on/off` and `1/0`.

---

## Where the settings live

```text
┌──────────────────────────────┐   writes    ┌────────────────────────────────────┐
│ talus-config                 │────────────►│ ~/.config/talus/config.conf        │
│ (this repository)            │  0600 file  │ # key = value, hand-editable       │
└──────────────────────────────┘  0700 dir   └────────────────┬───────────────────┘
                                                              │ read at start-up
                                                              ▼
                                              ┌────────────────────────────────────┐
                                              │ talus-monitor — the actual agent   │
                                              │ github.com/Hartwell-Labs/          │
                                              │   talus-process-monitor            │
                                              └────────────────────────────────────┘
```

```ini
# Talus configuration — written by `talus-config`.
# Read at start-up; command-line flags override these values.
background = black
alert_threshold = 80
ignore_comm = rsync, cc1plus
memlp = true
```

Precedence, highest first:

1. flags on the command line (`--alert-threshold 5`)
2. `config.conf` (this configurator, or `talus config set`)
3. Talus built-in defaults

Restart the agent (or the service) to apply changes — the file is read once at start-up,
never polled:

```bash
sudo systemctl restart talus-monitor
```

### Scripting it

The same file can be driven without the TUI, through the agent's own subcommand:

```bash
talus config show                  # every key, value, scope and the file path
talus config path                  # just the path
talus config set background=black alert_threshold=100
talus config unset auto_kill
talus config reset --yes
```

On a terminal, plain `talus config` opens **this** screen.

---

## Security

A configurator that touches a security agent is worth being paranoid about. Scope:

- **No network.** No HTTP client, no telemetry, no update check, no account. The
  dependency list is `anyhow`, `dirs` and FrankenTUI.
- **No privileges.** It never asks for root, never loads eBPF, never touches `/proc`.
  Run it as your user; a file written by root would be the agent's problem, not yours.
- **Restrictive permissions.** The config directory is created `0700` and the file
  `0600`; writes are atomic (temp file in the same directory, then rename), so an
  interrupted save cannot leave a half-written config.
- **Fail closed on garbage.** Unknown keys, unknown values and out-of-range numbers are
  rejected — by the configurator *and* again by the agent's parser.
- **Small.** Three source files, one binary, zero `unsafe`, zero `unwrap` on user input.

---

## FAQ

**Is this Talus?**
No. This edits a text file; it detects nothing. Talus is
[here](https://github.com/Hartwell-Labs/talus-process-monitor).

**Do I need this to run Talus?**
No. Talus runs fine without a config file, and you can write it by hand or with
`talus config set`.

**Does it need root?**
No — and it should not have it.

**Does it start the agent or run at boot?**
No. It is a one-shot editor. Nothing in Talus invokes it.

**Why a separate repository instead of a folder in the agent?**
So the agent stays one binary with no editor in it, so the configurator can be released,
documented and installed on its own, and so people looking for a GUI do not have to
build a kernel-facing agent. The source files are shared with the agent's tree; this is
their upstream home.

**Windows / macOS?**
No. Talus is a Linux eBPF agent, and this writes its Linux config path.

**Can I break my agent with it?**
You can point `bpf` at a bad object or set `auto_kill = true` — the same as typing the
flag. The configurator validates types and ranges, not your judgement.

---

## Development

```bash
cargo build            # debug binary at target/debug/talus-config
cargo test             # unit tests: parsing, validation, rendering, key handling
cargo clippy --all-targets
cargo fmt --check
```

```text
src/main.rs          entry point: mod config, mod theme, mod config_tui
src/config.rs        Config, the file format, validation, atomic save
src/theme.rs         four palettes (black, gray, white, navy)
src/config_tui.rs    the TUI: layout, editor, key map, mouse, preview
```

Good first issues: more palettes, a diff view of unsaved changes, per-option inline
help from the agent's `--help`.

---

## License

MIT — see [LICENSE](LICENSE). Same license, same author as Talus. The configurability
is free; so is the agent.

<div align="center">

**[Talus — the actual agent](https://github.com/Hartwell-Labs/talus-process-monitor)** ·
[Releases](https://github.com/Hartwell-Labs/Configurator-For-Talus/releases) ·
[Hartwell Labs](https://hartwell-labs.pl)

</div>
