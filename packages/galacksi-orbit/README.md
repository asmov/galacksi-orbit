Galacksi Orbit
========================================================================================================================
*A multiplayer space shooter set in the Galacksi universe.*

Standalone Installation
------------------------------------------------------------------------------------------------------------------------
*None of this is necessary if you've purchsed using Steam, Windows Store, or Apple Store.*
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

License (AGPL3 & CC-BY-SA-4)
------------------------------------------------------------------------------------------------------------------------
Galacksi Orbit: Multiplayer space shooter set in the Galacksi universe  
Copyright (C) 2025 Asmov LLC  
Asmov is a trademark (TM) of Asmov LLC  
Galacksi is a trademark (TM) of Asmov LLC  

This distribution is comprised of Software and Multimedia assets (images, videos, sounds, music, etc.).

Software is licensed under the terms of the GNU Affero Publice License 3.0 (AGPL-3) license.

Multimedia assets that do not portray elements of brand or trademark are licensed under the terms of the
Community Commons Attribution ShareAlike 4.0 International (CC-BY-SA-4) license.

All rights are reserved for Multimedia assets that portray elements of brand or trademark.

You should have received copying and license files for each license with this software:
  - COPYING-AGPL-3.txt and LICENSE-AGPL-3.txt
  - COPYING-CC-BY-SA-4.txt and LICENSE-CC-BY-SA-4.txt

Each subcomponent of this distribution may contain specific copying instructions and licensing terms.
