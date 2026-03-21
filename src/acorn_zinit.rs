// (c) 2026 Lord of the Pure Acorn: Veyyr3.
// This file is part of Pure Acorn and is distributed under the AGPL-3.0 License.
// See the LICENSE folder in the project root for the full license text.

// src/acorn_kernel/acorn_zinit.rs
use crate::acorn_kernel::{
    acorn_heart::{Zone, Location}, // import Zone, Location, AcornECS
};
use crate::acorn_settings::{AcornZoneContext, AcornGlobalContext};
use bevy_ecs::prelude::*;

/*
Create here your Zones, Locations. 
It's your interface. (sorry, code doesn't let me use GUI here)

Below, there are examples of Acorn functions.

======================
Warning: If you want to add new Zone then you should add new loop "for" in acorn_piston (read in docs about this).
======================
*/

/// Create here your Zones and Locations. 
/// It's your interface. (sorry, code doesn't let me use GUI here)
/// Add function to Location, Location to Zone.
/// Warning: If you want to add new Zone then you should add new cycle "for" in acorn_piston (read in docs about this).
pub fn acorn_setup() -> AcornZoneContext {
    /* 
    Here is an example. 
    

    Locations don't need variables! But you can use variables if you want.
    Warning: Variables of Locations should exists before variables of Zones in acorn_setup.

    ======================
    Memorise: read code from top to down. Locations, Zones will run by chain.
    ======================
    Warning Memorise: Functions will run from down to top (see reason in acorn_piston.rs)
    ======================
    */

    /*
    ======================
    Also I offer to you Lord-Minor achitecture to full control life of functions.
    ======================

    Lord-Location: here are Lord-Functions which can change other functions order in Minor-Locations.
    For each Lord-Functions in Lord-Location you should create own Minor-Location.

    Minor-Location: here are functions which obey to Lord-Function. They listen him and die, move or born by his orders.

    ======================
    Example: 
    if there are 3 Lord-Functions in Lord-Location then 3 Minor-Locations for each Lord-Functions.
    ======================
    OR Just Memorise: One Lord-Function = One his Minor-Location.
    ======================
    But YOU are not required to use this architecture. You are Lord of your ideas.
    ======================
    */

    // acorn_zone
    let acorn_zone = Zone::default()
    .with_locations(vec![
        // Lord-Location.
        Location::from_fn_vec(vec![
            /*
            Put here your Lords.
            Lords should changing his Minor-Location.
            In the code's bottom there are examples from Light Acorn, and so...
            PLEASE, come up with your own events that Lords will change theirs Minors.
            Because now is_key_pressed from Macroquad which is not here.
             */
            // acorn_example_delete_function
        ]),
        // Minor-Location.
        Location::from_fn_vec(vec![
            acorn_example_greeting,
            acorn_example_query_ecs,
            acorn_example_update_oaks,
            // add own functions through comma 
        ]),
        // add own locations through comma 
    ]);

    // Return AcornZoneContext for Main function
    AcornZoneContext { 
        acorn_zone, 
    }
}

/* ======================
Here are examples of functions.

Create functions by this template:
fn name(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // your_logic
}

Arguments:
* world - for ECS Queries. This is necessary in order to process thousands of objects.
* zones - for Lord-Functions. This is necessary for control Minor-Locations.
* context - for Global States.

Advise: Create functions in other files and import here.
====================== */

// ---------------------------- Example simple functions ----------------------------
// All simple functions should have World argument but shouldn't use it.
fn acorn_example_greeting(
    _world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    println!("Hello, Pure Acorn!")
}

// ---------------------------- Example ECS functions ----------------------------
// All ECS functions should have World argument.

// example component
#[derive(Component)]
struct Oaks {x: u64}

// Use spawn entities in fn main
pub fn acorn_example_spawn_entity(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    world.spawn((
        Oaks { x: 100 },
    ));
    println!("Entity spawned!");
}

// Add this function into location
fn acorn_example_query_ecs(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // create query
    let mut query = world.query::<&Oaks>();
    
    // cycle for all entities
    for oaks in query.iter(world) {
        println!("Entity has: {} oaks", oaks.x);
    }
}

// Add this function into location
fn acorn_example_update_oaks(
    world: &mut World, 
    _zones: &mut AcornZoneContext, 
    _context: &mut AcornGlobalContext
) {
    // create query
    let mut query = world.query::<&mut Oaks>();

    // loop for all entities. 
    // Spoiler: loop will be over when oaks reach 18 446 744 073 709 551 615 :)
    for mut oaks in query.iter_mut(world) {
        oaks.x += 1; 
    }
}

// ---------------------------- Example Lord-Functions ----------------------------
// Add this function into Lord-Location
// fn acorn_example_delete_function(
//     _world: &mut World, 
//     zones: &mut AcornZoneContext, 
//     _context: &mut AcornGlobalContext
// ) {
//     // KILL ANY FUNCTION IN FIRST ZONE, SECOND LOCATION!
//     // PRESS TAB!
//     // of course you have right to write if/else checking to get rid of 101 error in runtime:
//     // if !zones.acorn_zone.locations[1].functions.is_empty()
//     // but I leave this to understand REACORN-way for you
//     if is_key_pressed(KeyCode::Tab) { 
//         zones.acorn_zone.locations[1].functions.remove(0);
//         println!("I've killed function! Message from: acorn_example_delete_function");
//     }
// }

// // Add this function into Lord-Location
// fn acorn_example_add_circle_function(
//     _world: &mut World, 
//     zones: &mut AcornZoneContext, 
//     _context: &mut AcornGlobalContext
// ) {
//     // press left mouse button to draw your circle!
//     if is_mouse_button_pressed(MouseButton::Left) { 
//         zones.after_2d_zone.locations[1].functions.push(acorn_example_draw_circle);
//         println!("I've gave birth function! Message from: acorn_example_add_circle_function");
//     }
// }