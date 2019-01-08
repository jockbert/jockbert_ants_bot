extern crate ants_ai_challenge_api;
extern crate rand;

// TODO Simplify "imports"
use ants_ai_challenge_api::game_parameters::GameParameters;
use ants_ai_challenge_api::position::*;
use ants_ai_challenge_api::world_state::WorldState;
use ants_ai_challenge_api::Direction::*;
use ants_ai_challenge_api::*;
use rand::Rng;

struct FooAgent {}

fn random_direction() -> Direction {
    let mut rng = rand::thread_rng();
    let directions = [North, South, East, West];
    let index = rng.gen_range(0 as usize, directions.len());
    *directions.get(index).expect("no out of bounds")
}

impl Agent for FooAgent {
    fn prepare(&mut self, _params: &GameParameters) {
        // do nothing in prep
    }

    fn make_turn(&mut self, _params: &GameParameters, world: &WorldState) -> Orders {
        world
            .live_ants_for_player(0)
            .iter()
            // TODO: fix strange copy of position
            .map(|p| (pos(p.row, p.col), random_direction()))
            .collect()
    }

    fn at_end(&mut self, _params: &GameParameters, _world: &WorldState, _score: Score) {
        // do nothing at end
    }
}

fn main() {
    let mut agent = FooAgent {};
    run_game(&mut agent);
}
