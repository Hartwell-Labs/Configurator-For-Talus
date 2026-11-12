//! `talus-config` — the interactive configurator for Talus.
//!
//! **This is not the agent.** It does not load eBPF, does not watch processes
//! and needs no privileges. It edits one text file that the Talus monitor reads
//! when it starts:
//!
//! ```text
//! ~/.config/talus/config.conf        (or $TALUS_CONFIG_DIR/talus/config.conf)
//! ```
//!
//! The agent itself lives at
//! <https://github.com/Hartwell-Labs/talus-process-monitor>.

// `config.rs`, `theme.rs` and `config_tui.rs` are the same files the Talus
// agent builds. They carry a few helpers only the dashboard calls (the global
// palette in `theme`, the `print` table in `config`); keeping the copies
// byte-identical makes updates a plain copy instead of a merge, so the unused
// ones are allowed here rather than deleted.
#![allow(dead_code)]

mod config;
mod config_tui;
mod theme;

fn main() -> anyhow::Result<()> {
    config_tui::run()
}
