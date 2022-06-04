#[macro_use] extern crate rocket;

use std::arch::asm;
use rocket::form::Form;

#[derive(FromForm)]
struct Data{
    val:i64
}

#[get("/")]
fn index() -> &'static str {
    "Hoş geldiniz! Çay, kahve ikram edelim :)"
}

fn square(mut val: i64) -> i64{
    unsafe {
        asm!(
            "mov rax, {1}",
            "mov rbx, {1}",
            "mul rax",
            "mov {0}, rax",
            out(reg) val,
            in(reg) val,
        );
    }

    return val;
}

#[post("/square", data = "<data>")]
fn calc(data: Form<Data>) -> String {
    format!("{}", square(data.val));
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index, calc])
}