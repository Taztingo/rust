#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
mod handlers;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn mount_routes() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/blog", routes![handlers::blog::get, 
                                handlers::blog::post,
                                handlers::blog::patch, 
                                handlers::blog::delete])
}

fn main() {
    mount_routes().launch();
}