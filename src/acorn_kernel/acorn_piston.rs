// (c) 2026 Lord of the Pure Acorn: Veyyr3.
// This file is part of Pure Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/acorn_kernel/acorn_render.rs
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

        // before_2d_zone (Ex: UI input, ECS Queries, 3D Mesh drawing and other Locations)
        let len_before_2d_zone = acorn_context.before_2d_zone.locations.len();

        // locations go by order
        for location_index in 0..len_before_2d_zone {
            let fn_count = acorn_context
                .before_2d_zone
                .locations[location_index]
                .functions.len();

            // Reverse cycle for protect from panic (101 errors) in runtime
            // Functions go by reverse order
            // Warning: You should add new functions from down to top
            for fn_index in (0..fn_count).rev() {
                let function = 
                acorn_context.before_2d_zone
                    .locations[location_index]
                    .functions[fn_index];
                    
                // Call function in strict order
                function(&mut acorn_ecs.world, &mut acorn_context);
            }
        }
    }
}