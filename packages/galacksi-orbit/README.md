Galacksi Orbit
================================================================================
[![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/galacksi-orbit.svg
[crates.io]: https://crates.io/crates/galacksi-orbit

*A multiplayer space shooter set in the Galacksi universe.*

Standalone Installation
--------------------------------------------------------------------------------
*None of this is necessary if you've purchsed using Steam, Microsoft Store, or Apple Store.*
1. Install [Rust](https://rustup.rs)
2. Install Git
   - Windows
     - [Chocolatey](https://chocolatey.org/install) (CLI): `choco install git`
     - and/or [GitHub Desktop](https://desktop.github.com/download/) (GUI)
3. Clone the repository: `git clone https://github.com/asmov/galacksi-orbit.git`
4. Build the project: `cargo build --release`
5. Run the game with Cargo: `cargo run --release --bin galacksi-orbit`
6. Install the standalone game: `cargo install --path packages/galacksi-orbit`

To update the standalone game:
1. Pull updates: `git pull --rebase`
2. Clean the build (optional): `cargo clean`
2. Perform installation steps 4-6.

*Remove the `--release` arguments to run in debug mode.*


License (AGPL3)
--------------------------------------------------------------------------------
Galacksi Orbit: Multiplayer space shooter set in the Galacksi universe  
Copyright (C) 2025 [Asmov LLC](https://asmov.software)

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a [copy](./LICENSE-AGPL-3.txt) of the GNU Affero General Public License
along with this program.  If not, see https://www.gnu.org/licenses/.


Trademark
--------------------------------------------------------------------------------
*Galacksi* is a trademark of Asmov LLC.
