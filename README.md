<div style="text-align: center;" align="center">

# `Chip-8 Emulator`

## Yet another chip-8 emulator (in rust)

[![Duolingo](https://img.shields.io/crates/l/bitvec.svg?style=for-the-badge)](LICENSE)

</div>

1. [About](#about)
2. [Goals](#goals)
3. [Non-goals](#non-goals)
4. [File Structure](#file-structure)
5. [Resources](#resources)
6. [License](#license)

## About

This is my attempt at making a `Chip-8` emulator in rust. Chip-8 is apparantly the "Hello, World!" of emulators.

## Goals

- Implement all (original) CHIP-8 instructions
- A user-friendly command-line interface

## Non-goals

- Any kind of support for CHIP-48 or SUPER-CHIP extensions
- GUI interface to load, save ROMs etc.

## File Structure

- `src/`
  - `main.rs`: Entry point for the application. Contains code to interface with the GUI library.
  - `lib.rs`: Implementation of the CHIP-8 Emulator.
  - `structs.rs`: Contains all other structs used, including QOL ones.
  - `helpers.rs`: Helper functions.
- `roms/`
  - `tests/`: ROMs to test the implementation of the emulator. Taken from https://github.com/Timendus/chip8-test-suite
  - `games/`: Games :-)

PS: I think I overcommented the code a bit

## Resources

- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
  <br>
  A concise reference for instructions
- https://tobiasvl.github.io/blog/write-a-chip-8-emulator
  <br>
  Covers most ambiguous cases
- https://github.com/Timendus/chip8-test-suite
  <br>
  Includes test ROMs to verify your implementation

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
