# Midimon — Midi Monitoring Plugin

## Building

After installing [Rust](https://rustup.rs/), you can compile Midimon as follows:

```shell
cargo xtask bundle midimon --release
```

## Functional Requirements

1. Record a history of MIDI events
2. View a log of past MIDI events
   - Display all available information
   - Timing, Channel, VoiceID ...
   - Priority is NoteOn and NoteOff.

- [x] Display NoteOn and NoteOff in GUI
- [x] Record NoteOn and NoteOff live
- [x] Transmit new note events to GUI
- [ ] Update GUI regularly...

## Sources

- [NIH-plug](https://github.com/robbert-vdh/nih-plug) — API-agnostic audio plugin framework written in Rust.
- [Vizia](https://github.com/vizia/vizia) — Declarative GUI framework for the Rust programming language.
