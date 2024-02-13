use std::fs;
use crate::settings::{AppSettings, CONFIG_FILE};

#[test]
fn create_config() {
    let cfg = AppSettings::new().unwrap();
    assert_eq!("test@oxbox.ru", cfg.user_email);
    assert_eq!("12345", cfg.user_password);
    assert_eq!(false, cfg.run_on_logon);
    assert_eq!(true, fs::metadata(CONFIG_FILE).is_ok())
}

#[test]
fn read_config() {
    let config_file_name = "config_test.json";
    let mut cfg = AppSettings::new().unwrap();
    cfg.user_email = "neko_chan@test.ru".to_string();
    cfg.user_password = "qwerty".to_string();
    cfg.run_on_logon = true;

    cfg.to_file(config_file_name).unwrap();
    assert_eq!(true, fs::metadata(config_file_name).is_ok());

    let test = AppSettings::from_file(config_file_name).unwrap();
    assert_eq!("neko_chan@test.ru", test.user_email);
    assert_eq!("qwerty", test.user_password);
    assert_eq!(true, test.run_on_logon);

    fs::remove_file(config_file_name).unwrap();
}