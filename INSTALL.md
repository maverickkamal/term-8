# Installation Instructions

## Platform-Specific Setup

### Linux

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build --release

# Run
cargo run --release -- roms/ibm_logo.ch8
```

Linux systems have all required build tools by default. No additional setup needed.

---

### macOS

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build --release

# Run
cargo run --release -- roms/ibm_logo.ch8
```

macOS includes necessary build tools with Xcode Command Line Tools.

---

### Windows

Windows requires additional setup due to dependency requirements. Choose one of the following methods:

#### Method 1: WSL2 (Recommended)

Windows Subsystem for Linux provides a complete Linux environment:

1. Install WSL2 (if not already installed):
   ```powershell
   wsl --install -d Ubuntu
   ```

2. In the Ubuntu terminal:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. Navigate to project directory:
   ```bash
   cd "/mnt/c/Users/YOUR_USERNAME/path/to/term-8"
   ```

4. Build and run:
   ```bash
   cargo build --release
   cargo run --release -- roms/ibm_logo.ch8
   ```

#### Method 2: MinGW-w64

Install the MinGW-w64 toolchain for native Windows builds:

1. Download MSYS2 from https://www.msys2.org/
2. Install to default location (C:\msys64)
3. Open MSYS2 terminal and run:
   ```bash
   pacman -S mingw-w64-x86_64-toolchain
   ```
   (Type 'Y' to confirm)

4. Add to Windows PATH:
   - Search "Environment Variables" in Windows
   - Edit "Path" in System Variables
   - Add: `C:\msys64\mingw64\bin`
   - Click OK

5. Restart PowerShell, then:
   ```powershell
   cargo build --release
   cargo run --release -- roms/ibm_logo.ch8
   ```

---

## Verification

After installation, verify the emulator works:

```bash
cargo run --release -- roms/ibm_logo.ch8
```

You should see the IBM logo displayed in your terminal. Press ESC to exit.

---

## Troubleshooting

### Windows: "dlltool.exe not found"

This indicates MinGW-w64 is not installed or not in PATH.

**Solution**: Follow Method 1 (WSL2) or Method 2 (MinGW-w64) above.

### Any Platform: "rustc not found"

Rust is not installed.

**Solution**: Install from https://rustup.rs/

### Windows: PATH not updating

**Solution**: Restart PowerShell or reboot Windows after modifying PATH.

---

## Build Flags

The `--release` flag is recommended for optimal performance:

```bash
cargo build --release        # Optimized build
cargo build                  # Debug build (slower, more error info)
```

---

## Next Steps

After successful installation, see README.md for:
- Complete feature documentation
- Control schemes
- Inspector Mode usage
- Available ROMs

For ROM sources, see roms/README.md.
