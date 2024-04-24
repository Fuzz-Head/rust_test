pub fn test_options() -> Option<u8> {
    let mut opt1: Option<u8> = None;
    opt1 = Some(10);
    return opt1;
}

pub fn test_option_string() -> Option<String> {
    let mut opt1: Option<String> = None;
    opt1 = Some("Some Name".to_string());
    return opt1;
}
