/// Don't require restart
pub struct SoftSettings {
    /// Stiffness
    pub k: f64,
    /// Moment of inertia
    pub i: f64,
    /// Wave speed
    pub v: f64,
    pub anchor_top: bool,
    pub anchor_bottom: bool,
}

impl Default for SoftSettings {
    fn default() -> Self {
        Self {
            k: 0.1,
            i: 1.,
            v: 0.1,
            anchor_bottom: true,
            anchor_top: false,
        }
    }
}

/// Require restart
pub struct HardSettings {
    /// Amount of the rods
    pub n: u32,
    /// Length of the rods
    pub l: f32,
}

impl Default for HardSettings {
    fn default() -> Self {
        Self { n: 32, l: 5.0 }
    }
}
