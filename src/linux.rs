use std::error::Error;
use std::path::PathBuf;

const LAPTOP: u16 = 9;
const NOTEBOOK: u16 = 10;

pub(crate) fn check() -> bool {
    // Try to check the chassis type directly
    if let Ok(laptop) = check_chassis() {
        return laptop;
    }

    // If that fails check if the lid exists
    PathBuf::from("/proc/acpi/button/lid").exists()
}

fn check_chassis() -> Result<bool, Box<dyn Error>> {
    let chassis_txt = std::fs::read_to_string("/sys/class/dmi/id/chassis_type")?;
    let chassis: u16 = chassis_txt.parse()?;
    Ok(chassis == LAPTOP || chassis == NOTEBOOK)
}
