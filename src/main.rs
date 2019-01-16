extern crate ants_ai_challenge_api;
extern crate rand;

use ants_ai_challenge_api::run_game;
use jockbot_ants_bot::FooAgent;

fn main() {
    let mut agent = FooAgent {};
    run_game(&mut agent);
}
