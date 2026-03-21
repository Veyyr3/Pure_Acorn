// (c) 2026 Lord of the Pure Acorn: Veyyr3.
// This file is part of Pure Acorn and is distributed under the AGPL-3.0 License.
// See the LICENSE folder in the project root for the full license text.

// src/acorn_gsetup.rs
use crate::acorn_kernel::acorn_settings::{
    AcornGlobalContext,
};

/// Create here your Global States.
pub fn acorn_global_setup() -> AcornGlobalContext {
    AcornGlobalContext { 
        // money: 32,
    }
}