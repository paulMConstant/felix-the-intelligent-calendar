use serde::{Deserialize, Serialize};

use std::hash::{Hash, Hasher};

/// Represents a color. Each field should be kept in [0.0; 1.0].
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rgba {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl PartialEq for Rgba {
    // Simply round up the floats
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red
            && self.green == other.green
            && self.blue == other.blue
            && self.alpha == other.alpha
    }
}

impl Hash for Rgba {
    // Simply round up the floats
    fn hash<H: Hasher>(&self, state: &mut H) {
        for value in &[self.red, self.green, self.blue, self.alpha] {
            const HASH_PRECISION: f64 = 1_000_000.0;
            ((value * HASH_PRECISION) as usize).hash(state);
        }
    }
}
