use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    println!("Programming Starting ...");

    let main = Main::new().unwrap();

    main.on_do_and_save(|username, password, operator_mode, flush_mode| {
        println!("do_and_save: username: {username}, password: {password}, operator_mode: {operator_mode}, flush_mode: {flush_mode}");
        // doing...
    });

    main.on_cancel_service(|| {
        println!("cancel_service");
        // doing...
    });

    main.run().unwrap();
    Ok(())
}
