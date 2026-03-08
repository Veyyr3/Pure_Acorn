// (c) 2026 Lord of the Pure Acorn: Veyyr3.
// This file is part of Pure Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/main.rs
mod acorn_kernel;
use acorn_kernel::{
    acorn_piston::acorn_loop, // import acorn_loop
    acorn_heart::AcornECS, // import AcornECS
    acorn_init::{
        acorn_setup,
        acorn_example_spawn_entity
    }
};

/*
Hi!

This main.rs file is the example which you may try and search.

======================
Right now you are using tempelate ACORN-way 
(When you put function in strict order. Functions order is not mutable in runtime).
======================

See other templates of projects in "TEMPLATES" folder.

======================
Memorise: Zone is when, Location is where, Function is atom.
======================
*/


fn main() {
    // Global variable ECS. Hand over to acorn_loop.
    let mut acorn_ecs = AcornECS::default();

    // Global variable of Zones. Hand over to acorn_loop.
    let (before, after) = acorn_setup();

    // Create entities here (or in runtime by your logic)
    acorn_example_spawn_entity(&mut acorn_ecs.world);

    // main loop
    acorn_loop(before, after, acorn_ecs);
}
