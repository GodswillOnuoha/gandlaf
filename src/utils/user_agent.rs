use crate::adapters::dtos::DeviceInfo;
use axum::http::HeaderMap;

pub fn get_device_info(headers: HeaderMap) -> DeviceInfo {
    let mut ua = "".to_lowercase();

    let user_agent_str = headers
        .get("user-agent")
        .and_then(|ua| ua.to_str().ok())
        .map(|s| s.to_string());

    if let Some(user_agent) = &user_agent_str {
        ua = user_agent.to_lowercase();
    }

    let os = if ua.contains("windows") {
        "Windows"
    } else if ua.contains("mac os x") || ua.contains("macos") {
        "macOS"
    } else if ua.contains("linux") {
        "Linux"
    } else if ua.contains("android") {
        "Android"
    } else if ua.contains("ios") || ua.contains("iphone os") {
        "iOS"
    } else {
        "Unknown OS"
    };

    let browser = if ua.contains("chrome") && !ua.contains("chromium") {
        "Chrome"
    } else if ua.contains("firefox") {
        "Firefox"
    } else if ua.contains("safari") && !ua.contains("chrome") {
        "Safari"
    } else if ua.contains("edge") {
        "Edge"
    } else if ua.contains("opera") {
        "Opera"
    } else {
        "Unknown Browser"
    };

    let (device_type, device_name) = if ua.contains("iphone") {
        ("Mobile", "iPhone")
    } else if ua.contains("ipad") {
        ("Tablet", "iPad")
    } else if ua.contains("android") {
        if ua.contains("mobile") {
            ("Mobile", "Android Phone")
        } else {
            ("Tablet", "Android Tablet")
        }
    } else if ua.contains("mobile") {
        ("Mobile", "Mobile Device")
    } else if ua.contains("tablet") {
        ("Tablet", "Tablet")
    } else if ua.contains("desktop") {
        ("Desktop", "Desktop Computer")
    } else {
        ("Unknown", "Unknown Device")
    };

    DeviceInfo {
        device_type: device_type.to_string(),
        device_name: device_name.to_string(),
        browser: browser.to_string(),
        os: os.to_string(),
    }
}
