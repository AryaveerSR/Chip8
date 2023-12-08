<div style="text-align: center;" align="center">

# `Chip-8 Emulator`

## Yet another chip-8 emulator (in rust)

[![MIT](https://img.shields.io/crates/l/bitvec.svg?style=for-the-badge)](#License)

</div>

1. [About](#about)
2. [Features](#features)
3. [File Structure](#file-structure)
4. [Resources](#resources)
5. [Screenshots](#screenshots)
6. [License](#license)

## About

This is my attempt at making a `Chip-8` emulator in rust. Chip-8 is apparantly the "Hello, World!" of emulators.

## Features

- Implements all (original) CHIP-8 instructions
- A simple commandline tool to launch the emulator, with optional configuration

## File Structure

- `src/`
  - [`main.rs`](src/main.rs): Entry point for the application. Contains code to interface with the GUI library.
  - [`lib.rs`](src/lib.rs): Implementation of the CHIP-8 Emulator.
  - [`structs.rs`](src/structs.rs): Contains all other structs used, including QOL and multi-threading ones.
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

The test ROMs under [/roms/tests](roms/tests/) are sourced from https://github.com/Timendus/chip8-test-suite and are accordingly licensed under the [GNU GPL v3](https://github.com/Timendus/chip8-test-suite/blob/main/LICENSE).
