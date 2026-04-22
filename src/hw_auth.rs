use hbb_common::{log, tokio};

pub async fn check_hardware_authorization() -> bool {
    let mut serial = String::new();

    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("powershell")
            .args(&["-Command", "Get-CimInstance Win32_ComputerSystemProduct | Select-Object -ExpandProperty UUID"])
            .output()
        {
            let out = String::from_utf8_lossy(&output.stdout);
            serial = out.trim().to_string();
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Ok(s) = std::fs::read_to_string("/sys/class/dmi/id/product_uuid") {
            serial = s.trim().to_string();
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        // On macOS we can use system_profiler
        if let Ok(output) = std::process::Command::new("system_profiler")
            .args(&["SPHardwareDataType"])
            .output()
        {
             let out = String::from_utf8_lossy(&output.stdout);
             for line in out.lines() {
                 if line.contains("Hardware UUID") {
                      if let Some(s) = line.split(':').nth(1) {
                          serial = s.trim().to_string();
                          break;
                      }
                 }
             }
        }
    }

    if serial.is_empty() || serial.eq_ignore_ascii_case("UNKNOWN") {
        serial = "UNKNOWN_SERIAL".to_string();
    }

    let url = format!("https://kpit.pl/api/verify-serial/?serial={}", serial);
    
    // We use a blocking task to do the http request safely in the background
    let is_authorized = tokio::task::spawn_blocking(move || {
        match reqwest::blocking::get(&url) {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false
        }
    }).await.unwrap_or(false);

    is_authorized
}
