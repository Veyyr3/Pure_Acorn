// (c) 2026 Lord of the Pure Acorn: Veyyr3.
// This file is part of Pure Acorn and is distributed under the MIT License.
// See the LICENSES folder in the project root for the full license text.

// src/acorn_kernel/acorn_settings.rs
use crate::acorn_kernel::{
    acorn_heart::Zone
};

/// Contain here your Zones and global statements 
/// 
/// Advise: better keep global statements in other struct.
pub struct AcornContext {
    pub acorn_zone: Zone,
}