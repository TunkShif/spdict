use std::env;

mod cli;
mod models;
mod render;
mod service;

use render::Render;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = service::query(&args[1]);
    match query {
        Ok(data) => {
            print!("{}", data.render());
        }
        Err(err) => {
            dbg!(err);
        }
    }
}
