// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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