// This Source Code Form is subject to the terms of the Mozilla Public 
// License, v. 2.0. If a copy of the MPL was not distributed with this 
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/* Copyright © 2026 Veyyr3
  Pure Acorn Framework: Kernel
  Lord of the Framework: Veyyr3
*/

// src/acorn_kernel/acorn_piston.rs
use crate::acorn_kernel::{
    acorn_heart::AcornECS, 
    acorn_settings::AcornContext
};

/// Main loop of Pure Acorn.
/// 
/// Warning: If you want to add new Zones you should touch this (read in docs about this).
pub fn acorn_loop(mut acorn_context: AcornContext, mut acorn_ecs: AcornECS) {
    loop {
        // Run Schedule
        acorn_ecs.schedule.run(&mut acorn_ecs.world);

        // acorn_zone
        let len_acorn_zone = acorn_context.acorn_zone.locations.len();

        // locations go by order
        for location_index in 0..len_acorn_zone {
            let fn_count = acorn_context
                .acorn_zone
                .locations[location_index]
                .functions.len();

            // Reverse cycle for protect from panic (101 errors) in runtime
            // Functions go by reverse order
            // Warning: You should add new functions from down to top in acorn_init.rs
            for fn_index in (0..fn_count).rev() {
                let function = 
                acorn_context.acorn_zone
                    .locations[location_index]
                    .functions[fn_index];
                    
                // Call function in strict order
                function(&mut acorn_ecs.world, &mut acorn_context);
            }
        }
    }
}