# The Pure Acorn Framework

<img title="" src="/DOCS/logo_pure_acorn.png" alt="" width="168" data-align="center">

**Pure Acorn** is **Full Open Source Project** under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.

Hi! This was done on an old 2013 X550CC laptop and antiX.

> **Pure Acorn is clean Acorn architecture.**

**Pure has his brother Light:** Everything is like in Pure but with Macroquad to create games.

[Light Acorn. Click here.](https://github.com/Veyyr3/Light_Acorn)

# Features

- **Pure Acorn:** A lightweight Zone & Location-based architecture, powered by Bevy ECS. 
- **Pure Acorn is the Foundation** for your projects or even for building other frameworks.

- **Pure Acorn is Functional Data-Oriented Framework by design.**

- **This framework is not crate. It is template for your projects.**

- **Minimum entry threshold:** you don't need to lifetime fighting 'a and 'b, to know complex macros and smart pointers for begin creating your projects.

- **Acorn Architecture:** The entire framework skeleton is **built exclusively on vectors and loops**. Not a single unsafe, not a single smart pointer, not a single complex macro in the core. (Only vec! macros).

- **Bevy ECS include but optional.**

- **Zone & Location is unique concept:** The grouping of functions and their order is the basis of the engine. Developer control code's order, grouping functions by Zone (group of Locations) and Location (group of functions). Developer can create own Zones or Locations in Kernel: custom Zones with custom execution order.

- **Developer is owner his code:** You can **change functions order** in Locations. You can change Location's order in Zones. And **YOU CAN do it all in runtime** **WITHOUT**: unsafe blocks, smart pointers, macros and you shouldn't linking your code with Python, Lua (REACORN-way). PS: Only vectors and syntactic sugar macros.

- **Developer has choice of architecture Pure Acorn:** predicatable monolith **(ACORN)** OR flexible change of the order of execution of functions **(REACORN)**

# Stack

- Rust.

- Bevy ECS.

# Quick Start

1. Pure Acorn is a **template**. To start, clone the repository:

```bash
# For Linux & Windows (Git Bash / PowerShell)
git clone https://github.com/Veyyr3/Pure_Acorn.git
cd Pure_Acorn
```

2. And run the demo:

```bash
cargo run
```

# Code's examples

Create Simple function:

```rust
fn acorn_example_greeting(_world: &mut World, _context: &mut AcornContext) {
    print!("Hello, Pure Acorn!")
}
```

Add function to Zone and Location:

```rust
let zone = Zone::default()
.with_locations(vec![
    Location::from_fn_vec(vec![
        // simple function
        acorn_example_greeting,
        // add own functions through comma 
    ]),
    // add own locations through comma 
]);
```

**All code examples are already in the template!**

# Acknowledgments

**I express my gratitude to the authors:**

1. **Rust:** For the difficulty of learning.

2. **Bevy ECS:** For ease of learning (15 times easier than OOP) and Data-Oriented.

3. **antiX:** For giving the opportunity to continue working on old laptop.

4. **AI:** For the opportunity to do the impossible.

# License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.
