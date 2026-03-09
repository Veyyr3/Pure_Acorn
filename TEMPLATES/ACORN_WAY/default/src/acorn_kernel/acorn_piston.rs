// (c) 2026 Lord of the Pure Acorn: Veyyr3.
// This file is part of Pure Acorn and is distributed under the AGPL-3.0 License.
// See the LICENSE folder in the project root for the full license text.

/* Copyright © 2026 Veyyr3
  Pure Acorn Framework: Kernel
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_piston.rs
use crate::acorn_kernel::acorn_heart::{Zone, AcornECS};

/// Main loop of Pure Acorn.
/// You shouldn't touch this. 
/// Warning: If you want to add new Zones you should touch this (read in docs about this).
pub fn acorn_loop(acorn_zone: Zone, mut ecs: AcornECS) {
    loop {
        // Run Schedule
        ecs.schedule.run(&mut ecs.world);

        // acorn_zone
        for location in &acorn_zone.locations {
            for function in &location.functions {
                function(&mut ecs.world); // Call function in strict order
            }
        }
    }
}