mod services;

use std::error::Error;

use crate::services::file_io;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    println!("Programming Starting ...");

    let main = Main::new().unwrap();

    let weak_main = main.as_weak();
    match before_start(&main) {
        Ok(()) => {}
        Err(e) => {
            let main = weak_main.upgrade().unwrap();
            main.invoke_setTextString(format!("{} {}", "配置文件请求失败:", e.to_string()).into())
        }
    }

    let weak_main = main.as_weak();
    main.on_do_and_save(move |username, password, operator_mode, flush_mode| {
        let settings = file_io::Settings {
            username: username.to_string(),
            password: password.to_string(),
            operator_mode,
            flush_mode,
        };
        let main = weak_main.upgrade().unwrap();
        match file_io::write_or_new(settings) {
            Ok(()) => {
                main.invoke_setTextString("请求成功!".into());
            }
            Err(e) => {
                main.invoke_setTextString(format!("{} {}", "请求失败:", e.to_string()).into())
            }
        }
    });

    main.on_cancel_service(|| {
        println!("cancel_service");
        // doing...
    });

    main.run().unwrap();
    Ok(())
}

fn before_start(main: &Main) -> Result<(), Box<dyn std::error::Error>> {
    let main_ = main.as_weak().upgrade().unwrap();
    let settings = file_io::read_or_new()?;
    main_.invoke_setInit(
        settings.username.into(),
        settings.password.into(),
        settings.operator_mode.into(),
        settings.flush_mode.into(),
    );

    Ok(())
}
