mod examples;

use neon::register_module;

use examples::say_hello::{say_hello};
use examples::send_message::{send_message};
use examples::add_numbers::{add_numbers};
use examples::get_user::{get_user};
use examples::fibonacci_async::{fibonacci_async};

// --- Register and export all functions --- //
register_module!(mut m, {
    m.export_function("sayHello", say_hello)?;
    m.export_function("sendMessage", send_message)?;
    m.export_function("addNumbers", add_numbers)?;
    m.export_function("getUser", get_user)?;
    m.export_function("fibonacci", fibonacci_async)?;
    Ok(())
});
