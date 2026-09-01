# TgLib

A lightweight terminal UI library for Rust.

TgLib is designed to make building terminal-based applications easier by providing simple abstractions for terminal management, text, colours, and components.

> **Current version: 0.1.0**

## Features

* Terminal size detection
* Cross-platform terminal support
* Screen clearing
* Cursor positioning
* Coloured text
* Background colours
* Text components
* Simple terminal UI structure
* Windows and Unix support

## Requirements

* Rust
* Cargo
* Git

## Installation & Running

### Windows — PowerShell

```powershell
git clone https://github.com/enes73546/TgLib.git; cd TgLib; cargo add windows; cargo run
```

### Linux / macOS — Bash

```bash
git clone https://github.com/enes73546/TgLib.git && cd TgLib && cargo add libc && cargo run
```

## Platform Dependencies

TgLib uses platform-specific libraries to interact with the terminal.

### Windows

Windows support uses the `windows` crate.

If you are building TgLib on Windows, install it with:

```powershell
cargo add windows
```

### Unix / Linux / macOS

Unix-based systems use the `libc` crate.

Install it with:

```bash
cargo add libc
```

`libc` is not part of the Rust standard library, so it must be included as a Cargo dependency.

## Basic Example - Found in main.rs

```rust
use t_glib::Terminal;
use t_glib::components::Label;
use t_glib::colors::*;
use t_glib::types::Text;

fn main() {
    let terminal = Terminal::new();

    terminal.enter();
    terminal.clear_screen();

    let text = Label {
        pos: (40, 3),
        fore_color: GREEN,
        back_color: BG_BLACK,
        text: Text::from_str("Welcome to TgLib! | Version 0.1.0"),
    };

    terminal.printc(&text);

    terminal.sleep(std::time::Duration::from_hours(24)); // This keeps the prompt out your way until ^C

    terminal.leave();
}
```

## Project Structure

```text
src/
├── lib.rs
├── main.rs
├── terminal.rs
├── types.rs
├── colors.rs
├── components/
│   ├── mod.rs
│   └── label.rs
└── platform/
    ├── mod.rs
    ├── windows.rs
    └── unix.rs
```

## Colours

TgLib provides ANSI colour constants such as:

```rust
use t_glib::colors::*;

GREEN
RED
BLUE
WHITE
BG_BLACK
BG_RED
RESET
```

More colours and styles are available in the `colors` module.

## Terminal Coordinates

TgLib uses `(x, y)` for positions.

For example:

```rust
pos: (40, 3)
```

means:

```text
x = 40
y = 3
```

The coordinate system is used when positioning components inside the terminal.

## Status

TgLib is currently in **early development (0.1.0)**.

The API may change significantly as more terminal UI functionality is added.

## Contributing

Contributions, suggestions, and bug reports are welcome.

If you find a bug or have an idea for a feature, feel free to open an issue or pull request.

## License

License information will be added in a future release.
