use std::path::Path;

mod cell;
mod world;
mod macro_;

pub fn main() {
    let (mut w, c) = world::World::from_file(Path::new("./glider.txt"));
    if let cell::Cell::Macro(macro_::Macro { result, .. }) = c.as_ref() {
        println!("{}", world::World::to_string(&c));
        println!("{}", world::World::to_string(&result));
    }
}
