//! Windows registry read helpers for Addictive Keys.

pub const APP_KEY: &str = "Software\\Addictive\\Addictive Keys";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Addictive Keys";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Addictive Keys") },
        RegValue { name: "Version",     value: String::from("v4.5.5") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Addictive Keys") },
        RegValue { name: "Publisher",       value: String::from("Addictive") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Addictive Keys\\uninstall.exe\"") },
    ]
}
