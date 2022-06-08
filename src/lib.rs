#[cfg(target_os = "windows")]
mod windows;

/// Check if the local device is a Laptop or not.
///
/// True if the device is a Laptop or similar (like Notebook).
pub fn check() -> bool {
    #[cfg(target_os = "windows")]
    windows::check()
}
