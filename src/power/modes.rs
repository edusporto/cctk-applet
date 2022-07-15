use crate::shell_utils::CctkCommand;

pub const POWER_MODES: [&PowerMode; 4] = [OPTIMIZED, COOL, QUIET, ULTRA_PERFORMANCE];

pub const OPTIMIZED: &PowerMode = &PowerMode {
    name: "Optimized",
    code: "Optimzied",
};

pub const COOL: &PowerMode = &PowerMode {
    name: "Cool",
    code: "Cool",
};

pub const QUIET: &PowerMode = &PowerMode {
    name: "Quiet",
    code: "Quiet",
};

pub const ULTRA_PERFORMANCE: &PowerMode = &PowerMode {
    name: "Ultra Performance",
    code: "UltraPerformance",
};

#[derive(Debug, Clone, Copy)]
pub struct PowerMode {
    pub name: &'static str,
    pub code: &'static str,
}

impl CctkCommand for PowerMode {
    fn cctk_arg(&self) -> String {
        format!("--ThermalManagement={}", self.code)
    }
}
