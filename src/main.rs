// (c) 2026 Lord of the Pure Acorn: Veyyr3.
// This file is part of Pure Acorn and is distributed under the AGPL-3.0 License.
// See the LICENSE folder in the project root for the full license text.

// src/main.rs
mod acorn_kernel;
use acorn_kernel::{
    acorn_piston::acorn_loop, // import acorn_loop
    acorn_heart::AcornECS, // AcornECS
    acorn_zinit::{
        acorn_setup,
        // example funtion to spawn entities
        acorn_example_spawn_entity,
    },
};
use crate::acorn_kernel::acorn_ginit::acorn_global_setup; // to global setup

/*
Hi!

This main.rs file is the example which you may try and search.

======================
Right now you are using tempelate REACORN-way (when you can reoder functions in runtime).
BUT IF YOU DON'T WANT MUTABLE CODE IN RUNTIME: use ACORN WAY template in "TEMPLATES" folder.
======================

See other templates of projects in "TEMPLATES" folder.

======================
Memorise: Zone is when, Location is where, Function is time-marker.
======================
*/

fn main() {
    // Global variable ECS. Hand over to acorn_loop.
    let mut acorn_ecs = AcornECS::default();

    // Global variable of Zones. Hand over to acorn_loop.
    let mut acorn_zone_context = acorn_setup();
    // Global states. Hand over to acorn_loop.
    let mut acorn_global_context = acorn_global_setup();

    // Create entities here before loop (or in runtime by your logic)
    acorn_example_spawn_entity(
        &mut acorn_ecs.world, 
        &mut acorn_zone_context, 
        &mut acorn_global_context
    );

    // main loop
    acorn_loop(acorn_ecs, acorn_zone_context, acorn_global_context);
}
