# term-8: CHIP-8 Terminal Emulator

A high-fidelity CHIP-8 emulator with an interactive debugging interface, designed to run classic 1970s games directly in your terminal while providing deep insight into how they work.

## Overview

term-8 is not just an emulator; it's an educational tool that transforms vintage gaming into an interactive learning experience. The Inspector Mode allows you to pause execution at any moment and examine the virtual machine's internal state, making it invaluable for understanding computer architecture and retro programming.

### Key Features

- **Complete CHIP-8 Implementation**: All 35 original opcodes accurately emulated
- **Inspector Mode**: Multi-panel debugging interface with real-time CPU state visualization
- **Live Disassembler**: See upcoming instructions translated to human-readable assembly
- **Single-Step Execution**: Step through programs one instruction at a time
- **Checkpoint System**: Save and restore emulator state for experimentation
- **CRT Display Effects**: Authentic retro aesthetics with scanlines and pixel ghosting
- **60 FPS Performance**: Smooth gameplay with accurate timing

## Quick Start

### Installation

**Prerequisites**: Rust 1.70 or higher ([install from rustup.rs](https://rustup.rs/))

```bash
git clone https://github.com/yourusername/term-8.git
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

The Inspector Mode is term-8's defining feature, providing a comprehensive view of the emulator's internal state.

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

### Project Structure

```
term-8/
├── src/
│   ├── main.rs           - Entry point and CLI
│   ├── emulator.rs       - Core CHIP-8 virtual machine
│   ├── opcodes.rs        - Instruction implementations
│   ├── ui.rs             - Terminal user interface
│   └── disassembler.rs   - Assembly translation
├── roms/
│   ├── ibm_logo.ch8      - Test ROM
│   ├── pong.ch8          - Classic Pong
│   └── tetris.ch8        - Tetris implementation
├── documentation/        - Internal documentation
├── README.md             - This file
├── INSTALL.md            - Installation instructions
└── LICENSE               - MIT License
```

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

term-8 is designed for learning and teaching:

- **Computer Architecture**: Observe how CPU, memory, and I/O interact
- **Assembly Language**: See high-level game logic translated to machine instructions
- **Debugging Techniques**: Practice step-through debugging and state inspection
- **Retro Computing**: Understand 1970s programming constraints and solutions
- **Emulator Development**: Study the source code to learn emulation principles

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

---

**term-8** - Bringing 1970s computing to the modern terminal
