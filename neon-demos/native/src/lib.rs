mod examples;

use neon::register_module;

// --- Register and export all functions --- //
register_module!(mut m, {
    m.export_function("sayHello", examples::say_hello::run)?;
    m.export_function("sendMessage", examples::send_message::run)?;
    m.export_function("addNumbers", examples::add_numbers::run)?;
    m.export_function("getUser", examples::get_user::run)?;
    m.export_function("fibonacci", examples::fibonacci_async::run)?;
    Ok(())
});
