<div style="text-align: center;" align="center">

# `Chip-8 Emulator`

## Yet another chip-8 emulator (in rust)

[![Duolingo](https://img.shields.io/crates/l/bitvec.svg?style=for-the-badge)](LICENSE)

</div>

1. [About](#about)
2. [Goals](#goals)
3. [Future Goals](#future-goals)
4. [File Structure](#file-structure)
5. [Resources](#resources)
6. [Screenshots](#screenshots)
7. [License](#license)

## About

This is my attempt at making a `Chip-8` emulator in rust. Chip-8 is apparantly the "Hello, World!" of emulators.

## Goals

- Implement all (original) CHIP-8 instructions
- A user-friendly command-line interface

## Future Goals

- Any kind of support for CHIP-48 or SUPER-CHIP extensions
- GUI interface to load, save ROMs etc.

## File Structure

- `src/`
  - [`main.rs`](src/main.rs): Entry point for the application. Contains code to interface with the GUI library.
  - [`lib.rs`](src/lib.rs): Implementation of the CHIP-8 Emulator.
  - [`structs.rs`](src/structs.rs): Contains all other structs used, including QOL ones.
  - [`helpers.rs`](src/helpers.rs): Helper functions.
- `roms/`
  - `tests`: ROMs to test the implementation of the emulator. Taken from https://github.com/Timendus/chip8-test-suite
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

## Screenshots

- IBM Logo
  ![image](https://github.com/AryaveerSR/Chip8/assets/51504825/449355e3-ef4e-42eb-bbed-7ecb791fab3b)

- Flight Runner
  ![image](https://github.com/AryaveerSR/Chip8/assets/51504825/daa79e42-7147-4ecd-af97-d6fb525605ef)

- Keypad
  ![image](https://github.com/AryaveerSR/Chip8/assets/51504825/6962f0e2-8671-4258-b1e4-46746ac7b25c)

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
