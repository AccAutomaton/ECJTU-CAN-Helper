use super::file_io::Settings;

pub fn refresh(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    logout()?;
    login(&settings)?;

    Ok(())
}

#[tokio::main]
async fn login(settings: &Settings) -> Result<(), reqwest::Error> {
    let params = [
        ("DDDDD", format!(",1,{}@{}", settings.username, match settings.operator_mode {
            1 => "mobile",
            2 => "unicom",
            3 => "telecom",
            _ => ""
        })),
        ("upass", settings.password.to_string()),
        ("R1", "0".to_string()),
        ("R2", "0".to_string()),
        ("R3", "0".to_string()),
        ("R6", "0".to_string()),
        ("para", "00".to_string()),
        ("0MKKey", "123456".to_string()),
        ("buttonClicked", "".to_string()),
        ("redirect_url", "".to_string()),
        ("err_flag", "".to_string()),
        ("username", "".to_string()),
        ("password", "".to_string()),
        ("user", "".to_string()),
        ("cmd", "".to_string()),
        ("Login", "".to_string()),
    ];

    let body = reqwest::get("http://172.16.2.100/")
        .await?
        .text()
        .await?;
    let v46ip_location = body.find("v46ip").unwrap();
    let rest_body = &body[v46ip_location + 6..];
    let ip_end = rest_body.find("'").unwrap();
    let ip = &rest_body[..ip_end];

    let client = reqwest::Client::new();
    let client_url = format!("http://172.16.2.100:801/eportal/?c=ACSetting&a=Login&protocol=http:&hostname=172.16.2.100&iTermType=1&wlanuserip={}&wlanacip=null&wlanacname=null&mac=00-00-00-00-00-00&ip={}&enAdvert=0&queryACIP=0&loginMethod=1", ip.to_string(), ip.to_string());

    client.post(client_url)
        .form(&params)
        .send()
        .await?;
    Ok(())
}

#[tokio::main]
async fn logout() -> Result<(), reqwest::Error> {
    reqwest::get("http://172.16.2.100:801/eportal/?c=ACSetting&a=Logout&wlanuserip=null&wlanacip=null&wlanacname=null&port=&hostname=172.16.2.100&iTermType=1&session=null&queryACIP=0&mac=00-00-00-00-00-00")
        .await?
        .text()
        .await?;
    Ok(())
}
