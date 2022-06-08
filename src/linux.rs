use std::path::PathBuf;

pub(crate) fn check() -> bool {
    let lid = PathBuf::from("/proc/acpi/button/lid");
    lid.exists()
}
