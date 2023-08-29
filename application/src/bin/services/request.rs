use super::file_io::Settings;

pub fn refresh(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "请求参数: username: {}, password: {}, operator_mode: {}",
        settings.username, settings.password, settings.operator_mode
    );

    logout();
    login(settings);

    println!("请求成功!");
    Ok(())
}

fn login(settings: Settings) {
    // do something ...
}

fn logout() {
    // do something ...
}
