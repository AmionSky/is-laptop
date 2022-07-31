use regex::Regex;
use std::error::Error;
use std::ptr::null;
use windows::Win32::Foundation::BSTR;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitialize, CoInitializeSecurity, CoUninitialize, CLSCTX_INPROC_SERVER,
    CLSCTX_LOCAL_SERVER, EOAC_NONE, RPC_C_AUTHN_LEVEL_DEFAULT, RPC_C_IMP_LEVEL_IMPERSONATE,
};
use windows::Win32::System::Wmi::{
    IWbemLocator, WbemLocator, WBEM_FLAG_FORWARD_ONLY, WBEM_FLAG_NO_FLAVORS, WBEM_NO_WAIT,
};

const LAPTOP: u16 = 9;
const NOTEBOOK: u16 = 10;

pub(crate) fn check() -> bool {
    if let Ok(chassis) = unsafe { query() } {
        return chassis == LAPTOP || chassis == NOTEBOOK;
    }
    false
}

unsafe fn query() -> Result<u16, Box<dyn Error>> {
    CoInitialize(null())?;
    let result = run_query();
    CoUninitialize();
    result
}

unsafe fn run_query() -> Result<u16, Box<dyn Error>> {
    CoInitializeSecurity(
        None,
        -1,
        null(),
        null(),
        RPC_C_AUTHN_LEVEL_DEFAULT,
        RPC_C_IMP_LEVEL_IMPERSONATE,
        null(),
        EOAC_NONE,
        null(),
    )?;

    let wbem_locator: IWbemLocator = CoCreateInstance(
        &WbemLocator,
        None,
        CLSCTX_INPROC_SERVER | CLSCTX_LOCAL_SERVER,
    )?;

    let wbem_service =
        wbem_locator.ConnectServer(&BSTR::from("root\\CIMV2"), None, None, None, 0, None, None)?;

    let query = wbem_service.ExecQuery(
        &BSTR::from("WQL"),
        &BSTR::from("SELECT ChassisTypes FROM Win32_SystemEnclosure"),
        WBEM_FLAG_FORWARD_ONLY.0,
        None,
    )?;

    let mut rnum = 0;
    let mut objs = vec![None];
    let result = query.Next(WBEM_NO_WAIT.0, &mut objs, &mut rnum);

    if result.is_ok() && rnum > 0 {
        if let Some(qres) = &objs[0] {
            let text = qres.GetObjectText(WBEM_FLAG_NO_FLAVORS.0)?.to_string();
            let re = Regex::new(r"ChassisTypes.*(\d+)")?;
            if let Some(caps) = re.captures(&text) {
                if let Some(val) = caps.get(1) {
                    return Ok(val.as_str().parse()?);
                }
            }
        }
    }

    Err("Failed to get ChassisTypes".into())
}
