#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;

/// Check if the local device is a Laptop or not.
///
/// True if the device is a Laptop or similar (like Notebook).
pub fn check() -> bool {
    #[cfg(target_os = "windows")]
    return windows::check();

    #[cfg(target_os = "linux")]
    return linux::check();
}
