use std::path::Path;

use libappindicator::{AppIndicator, AppIndicatorStatus};

pub fn create_indicator() -> AppIndicator {
    // Create indicator
    let mut indicator = AppIndicator::new("Dell CCTK applet", "");
    indicator.set_status(AppIndicatorStatus::Active);

    // Set the indicator's icon
    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("img");
    indicator.set_icon_theme_path(icon_path.to_str().unwrap());
    indicator.set_icon_full("dell-t", "icon");

    indicator
}
