// (c) 2026 Lord of the Pure Acorn: Veyyr3.
// This file is part of Pure Acorn and is distributed under the AGPL-3.0 License.
// See the LICENSE folder in the project root for the full license text.

// src/acorn_kernel/acorn_piston.rs
use crate::acorn_kernel::{
    acorn_heart::AcornECS, 
};
use crate::acorn_settings::{AcornGlobalContext, AcornZoneContext};

/// Main loop of Pure Acorn.
/// 
/// Warning: If you want to add new Zones you should touch this (read in docs about this).
pub fn acorn_loop(
    mut acorn_ecs: AcornECS,
    mut acorn_zone_context: AcornZoneContext, 
    mut acorn_global_context:AcornGlobalContext
) {
    loop {
        // Run Schedule
        // acorn_ecs.schedule.run(&mut acorn_ecs.world);

        // acorn_zone
        let len_acorn_zone = acorn_zone_context.acorn_zone.locations.len();

        // locations go by order
        for location_index in 0..len_acorn_zone {
            let fn_count = acorn_zone_context
                .acorn_zone
                .locations[location_index]
                .functions.len();

            // Reverse cycle for protect from panic (101 errors) in runtime
            // Functions go by reverse order
            // Warning: You should add new functions from down to top in acorn_init.rs
            for fn_index in (0..fn_count).rev() {
                let function = 
                acorn_zone_context.acorn_zone
                    .locations[location_index]
                    .functions[fn_index];
                    
                // Call function in strict order
                function(&mut acorn_ecs.world, &mut acorn_zone_context, &mut acorn_global_context);
            }
        }
    }
}