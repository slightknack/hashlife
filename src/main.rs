mod cell;
mod world;
mod macro_;

fn main() {
    let glider = cell::Cell::Base([
        false, true,  false, false,
        false, false, true,  false,
        true,  true,  true,  false,
        false, false, false, false
    ]);
}
