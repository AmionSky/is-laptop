use serde::Deserialize;
use std::error::Error;
use wmi::{COMLibrary, WMIConnection};

const LAPTOP: u16 = 9;
const NOTEBOOK: u16 = 10;

pub(crate) fn check() -> bool {
    if let Ok(chassis) = get_case_types() {
        return chassis.iter().any(|c| *c == LAPTOP || *c == NOTEBOOK);
    }
    false
}

#[derive(Deserialize)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct Win32_SystemEnclosure {
    pub ChassisTypes: Vec<u16>,
}

fn get_case_types() -> Result<Vec<u16>, Box<dyn Error>> {
    // Init wmi
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    // Query result
    let mut results: Vec<Win32_SystemEnclosure> = wmi_con.query()?;
    if results.is_empty() {
        return Err("Query has no result".into());
    }

    // Extract the chassis vector
    Ok(std::mem::take(&mut results[0].ChassisTypes))
}
