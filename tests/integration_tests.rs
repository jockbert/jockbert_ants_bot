extern crate ants_ai_challenge_api;
extern crate jockbot_ants_bot;

use ants_ai_challenge_api::*;
use jockbot_ants_bot::*;

#[test]
fn test_forced_to_walk_east() {
    let mut agent = FooAgent::default();

    let params = GameParameters {
        cols: 6,
        rows: 3,
        attackradius2: 1,
        loadtime_ms: 1000,
        turntime_ms: 1000,
        turns: 10,
        spawnradius2: 1,
        viewradius2: 3,
        player_seed: 1_244_972_669_850_148_400,
    };

    agent.prepare(params);

    // Ant sorounded by water except on east side.
    let world = WorldState::default()
        .food(pos(1, 5))
        .live_ant(pos(1, 1), 0)
        .water(pos(0, 1))
        .water(pos(1, 0))
        .water(pos(2, 1));

    let orders = agent.make_turn(world, 1);

    assert_eq![orders, vec![pos(1, 1).east()]];
}
