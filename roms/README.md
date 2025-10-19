# CHIP-8 ROMs

## Included ROMs

This directory contains three public domain CHIP-8 ROMs:

1. **ibm_logo.ch8** - IBM logo display test
2. **pong.ch8** - Single-player Pong game  
3. **tetris.ch8** - Tetris implementation

## Running ROMs

```bash
cargo run --release -- roms/ibm_logo.ch8
cargo run --release -- roms/pong.ch8
cargo run --release -- roms/tetris.ch8
```

## Finding More ROMs

### Public Domain Collections

- [chip8-roms](https://github.com/kripod/chip8-roms) - Large collection of games
- [CHIP-8 Archive](https://johnearnest.github.io/chip8Archive/) - Curated library with descriptions
- [Test Suite](https://github.com/Timendus/chip8-test-suite) - Comprehensive test ROMs

### Recommended Games

- **Space Invaders** - Classic arcade game
- **Breakout** - Atari Breakout clone
- **Cave** - Avoid obstacles game
- **Connect 4** - Two-player strategy
- **Missile** - Shoot falling missiles

## ROM Format

CHIP-8 ROMs are raw binary files loaded into memory at address 0x200. Most ROMs are 512 bytes to 3.5KB in size.

## Controls by Game

### Pong
- 1: Move paddle up
- Q: Move paddle down

### Tetris  
- Q: Rotate piece
- W: Move left
- E: Move right
- A: Drop piece

Game controls vary by ROM. Use Inspector Mode (press I) to observe which keys the program checks.

## Creating ROMs

To create your own CHIP-8 programs:

1. Write assembly code using CHIP-8 instruction set
2. Assemble with tools like [Octo](https://github.com/JohnEarnest/Octo)
3. Test with term-8's Inspector Mode for debugging

See [Cowgod's CHIP-8 Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) for complete instruction set documentation.
