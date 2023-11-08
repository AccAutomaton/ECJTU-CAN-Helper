#![windows_subsystem = "windows"]

mod services;

use std::error::Error;

use crate::services::{file_io, request};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
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
    main.on_do_and_save(move |username, password, operator_mode| {
        let settings = file_io::Settings {
            username: username.to_string(),
            password: password.to_string(),
            operator_mode,
        };

        let main = weak_main.upgrade().unwrap();
        match check_parameters(
            &settings.username,
            &settings.password,
            settings.operator_mode,
        ) {
            Err(e) => {
                main.invoke_setTextString(e.to_string().into());
            }
            _ => {}
        }

        let main = weak_main.upgrade().unwrap();
        match file_io::write_or_new(&settings) {
            Ok(()) => {
                main.invoke_setTextString("配置写入成功!".into());
            }
            Err(e) => {
                main.invoke_setTextString(format!("{} {}", "配置写入失败:", e.to_string()).into())
            }
        }

        let main = weak_main.upgrade().unwrap();
        match request::refresh(settings) {
            Ok(()) => {
                main.invoke_setTextString("请求成功!".into());
            }
            Err(e) => {
                main.invoke_setTextString(format!("{} {}", "请求失败:", e.to_string()).into());
                eprintln!("{} {}", "请求失败:", e.to_string())
            }
        }
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
    );

    Ok(())
}

fn check_parameters(
    username: &String,
    password: &String,
    operator_mode: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    if username.eq("") {
        return Err("用户名不能为空".into());
    }
    if password.eq("") {
        return Err("密码不能为空".into());
    }
    if operator_mode < 1 && operator_mode > 3 {
        return Err("请选择运营商".into());
    }
    Ok(())
}
