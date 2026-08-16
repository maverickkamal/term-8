# term-8: CHIP-8 Terminal Emulator

term-8 is a Rust-based CHIP-8 emulator for the terminal. It runs classic CHIP-8 ROMs and includes an inspector mode for stepping through execution, viewing CPU state, and debugging programs.

## Overview

The project focuses on a faithful CHIP-8 implementation and a practical terminal debugger. You can run games at normal speed, pause at any time, and inspect registers, stack state, timers, and upcoming instructions.

<img width="1902" height="958" alt="Screenshot 2025-10-20 005950" src="https://github.com/user-attachments/assets/60f71757-619b-4501-899c-35c93dc8c406" />

<img width="1461" height="847" alt="Screenshot 2025-10-19 012156" src="https://github.com/user-attachments/assets/393fc837-cf51-4a98-bbfc-1e90dfbb3c22" />


### Features

- **CHIP-8 implementation**: Supports all 35 original opcodes
- **Inspector mode**: Multi-panel debugging interface with live CPU state
- **Live disassembler**: Shows upcoming instructions in readable assembly
- **Single-step execution**: Execute one instruction at a time
- **Checkpoint support**: Save and restore emulator state
- **CRT display effects**: Optional scanline and pixel ghosting effects
- **60 FPS timing**: Runs with accurate frame and timer behavior

## Quick Start

### Installation

**Prerequisites**: Rust 1.70 or higher ([install from rustup.rs](https://rustup.rs/))

```bash
git clone https://github.com/maverickkamal/term-8.git
cd term-8
cargo build --release
```

**Windows Users**: See [INSTALL.md](INSTALL.md) for platform-specific setup instructions.

### Running Games

```bash
cargo run --release -- roms/pong.ch8
cargo run --release -- roms/tetris.ch8
cargo run --release -- roms/ibm_logo.ch8
```

Adjust emulation speed with the `--speed` flag (default: 10 cycles per frame):

```bash
cargo run --release -- roms/pong.ch8 --speed 15
```

## Controls

### Game Input

The CHIP-8's hexadecimal keypad is mapped to modern keyboards:

```
CHIP-8 Keypad        Keyboard
┌───┬───┬───┬───┐    ┌───┬───┬───┬───┐
│ 1 │ 2 │ 3 │ C │    │ 1 │ 2 │ 3 │ 4 │
├───┼───┼───┼───┤    ├───┼───┼───┼───┤
│ 4 │ 5 │ 6 │ D │    │ Q │ W │ E │ R │
├───┼───┼───┼───┤    ├───┼───┼───┼───┤
│ 7 │ 8 │ 9 │ E │    │ A │ S │ D │ F │
├───┼───┼───┼───┤    ├───┼───┼───┼───┤
│ A │ 0 │ B │ F │    │ Z │ X │ C │ V │
└───┴───┴───┴───┘    └───┴───┴───┴───┘
```

### System Controls

| Key | Action |
|-----|--------|
| ESC | Exit emulator |
| P | Pause/Resume execution |
| I | Toggle Inspector Mode |

### Inspector Mode Controls

| Key | Action |
|-----|--------|
| S | Execute single instruction (step) |
| C | Continue execution |
| R | Reset emulator |
| K | Save checkpoint |
| L | Load checkpoint |
| I | Exit Inspector Mode |
| ESC | Exit emulator |

## Inspector Mode

Inspector mode provides a full view of emulator state during execution.

### Interface Layout

```
┌─────────────────────┬──────────────────────┐
│                     │  CPU State           │
│   CHIP-8 Display    │  V0-VF registers     │
│   64x32 pixels      │  I, PC, SP           │
│   with CRT effects  │  Delay/Sound timers  │
│                     ├──────────────────────┤
│                     │  Stack               │
│                     │  16 levels           │
├─────────────────────┼──────────────────────┤
│  Disassembly        │  Controls            │
│  Next 12 opcodes    │  S: Step             │
│  0x0234: 8450       │  C: Continue         │
│  ADD V4, V5         │  R: Reset            │
│  ...                │  K/L: Checkpoint     │
└─────────────────────┴────────────────────────┘
```

### Usage

1. Press `I` during gameplay to enter Inspector Mode
2. Examine CPU registers, stack, and upcoming instructions
3. Press `S` to execute one instruction and observe state changes
4. Use `K` to save state, experiment, and `L` to restore
5. Press `C` to resume normal execution

## Technical Specifications

### CHIP-8 Architecture

- **Memory**: 4KB (0x000-0xFFF)
- **Display**: 64x32 monochrome
- **Registers**: 16 general-purpose (V0-VF)
- **Special Registers**: Index (I), Program Counter (PC), Stack Pointer (SP)
- **Stack**: 16 levels for subroutine calls
- **Timers**: 60Hz delay and sound timers
- **Input**: 16-key hexadecimal keypad

### Implementation

**Language**: Rust

**Dependencies**:
- `ratatui`: Terminal UI framework
- `crossterm`: Cross-platform terminal control
- `rand`: Random number generation
- `clap`: Command-line argument parsing

## Available ROMs

term-8 includes three public domain ROMs for testing:

1. **IBM Logo** (`ibm_logo.ch8`): Displays the IBM logo, useful for testing basic functionality
2. **Pong** (`pong.ch8`): Single-player Pong (1: move up, Q: move down)
3. **Tetris** (`tetris.ch8`): Classic Tetris (Q: rotate, W: left, E: right, A: drop)

For additional ROMs, see:
- [chip8-roms](https://github.com/kripod/chip8-roms) - Public domain game collection
- [CHIP-8 Archive](https://johnearnest.github.io/chip8Archive/) - Curated ROM library
- [Test Suite](https://github.com/Timendus/chip8-test-suite) - Comprehensive tests

## Educational Applications

term-8 is useful for learning and teaching:

- **Computer Architecture**: Observe CPU, memory, and I/O behavior
- **Assembly Language**: Inspect CHIP-8 instructions during execution
- **Debugging Techniques**: Practice stepping and state inspection
- **Retro Computing**: Explore 1970s-era virtual machine constraints
- **Emulator Development**: Study a complete Rust emulator codebase

## Building from Source

### All Platforms

```bash
cargo build --release
```

The compiled binary will be in `target/release/`.

### Platform-Specific Notes

- **Linux/macOS**: Build works out of the box
- **Windows**: Requires MinGW-w64 toolchain or WSL2

See [INSTALL.md](INSTALL.md) for detailed platform-specific instructions.

## Performance

term-8 runs at 60 frames per second with configurable CPU speed. The default setting (10 cycles per frame, 600 Hz) works well for most games. Adjust with `--speed` for faster/slower execution.

## Terminal Compatibility

term-8 works with any modern terminal emulator that supports:
- Unicode characters (for display rendering)
- ANSI color codes
- Terminal size detection

Tested terminals include: Windows Terminal, iTerm2, Alacritty, GNOME Terminal, Konsole, and macOS Terminal.app.

## Known Limitations

- No audio output (CHIP-8 sound timer is tracked but not played)
- Input uses per-frame polling rather than true key up/down events
- Display scales to terminal size but maintains 2:1 aspect ratio

## Contributing

Contributions are welcome. Potential improvements:

- Enhanced CRT effects (screen curvature, bloom)
- SUPER-CHIP and XO-CHIP compatibility
- ROM file browser interface
- Built-in assembler
- Save state to disk

## License

MIT License - see LICENSE file for details.

## References

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Guide to Making a CHIP-8 Emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [CHIP-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)

## Further Reading

- [INSTALL.md](INSTALL.md) - Detailed installation instructions for all platforms
- [roms/README.md](roms/README.md) - Information about ROMs and where to find more
