
# Cricket

[![Crates.io](https://img.shields.io/crates/v/cricket)]()
[![License](https://img.shields.io/badge/license-MIT-blue.svg)]()

Cricket is an open-source, Rust-based domain-specific language (DSL) designed for expressive, text-based music composition. It enables composers and developers to write structured music definitions in plain text files, which are then compiled into MIDI (and optionally WAV).

---

## Features

- **Simple and expressive grammar** for defining Instruments, Patterns, Sections, and Songs.
- **Modular design:** Compose music by combining reusable patterns and sections.
- **Rust core library (`musiqlang`)** with lexer, parser, AST, semantic analysis, and MIDI codegen powered by [`midly`](https://docs.rs/midly).
- **CLI tool (`cricket`)** supporting MIDI generation and optional WAV rendering with FluidSynth.
- Support for **custom SoundFonts** via CLI option.
- Planned expansions: polyphonic composition, live playback, LSP's for IDE's(VSCode, Nvim, JetBrains extensions) and collaborative web tooling.
---

## Getting Started -Not ready

This project is still on the start phase. Please come back soon to test the cricket.

### Prerequisites

- Rust toolchain ([rustup](https://rustup.rs/))
- FluidSynth installed (for WAV generation)
- Optional: Custom SoundFont files (`.sf2`) for richer sound rendering

### Installation

Clone the repository:

```bash
git clone https://github.com/yourusername/cricket.git
cd cricket
