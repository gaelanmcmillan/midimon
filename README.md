# Midimon

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
