#[macro_use]
extern crate bencher;
#[macro_use(set)]
extern crate jockbot_ants_bot;
extern crate ants_ai_challenge_api;

use ants_ai_challenge_api::*;
use bencher::Bencher;
use jockbot_ants_bot::strategy::search::*;
use jockbot_ants_bot::world_step::*;

use jockbot_ants_bot::utilities::*;

use std::collections::HashSet;

fn search(search: Box<dyn Search>) {
    let world = world("b-------------------a--");
    let size = pos(32_000, 32_000);
    let world_step = BasicWorldStep::new(world, size);

    let mut ants: HashSet<Position> = set![];

    for col in 200..400 {
        for row in 0..20 {
            ants.insert(pos(row, col));
        }
    }

    // The nearest ants is last in the set, with a little randomness
    ants.insert(pos(0, 103));
    ants.insert(pos(0, 104));
    ants.insert(pos(0, 105));
    ants.insert(pos(0, 106));
    ants.insert(pos(0, 107));
    ants.insert(pos(0, 100));
    ants.insert(pos(0, 101));
    ants.insert(pos(0, 102));
    ants.insert(pos(0, 108));
    ants.insert(pos(0, 109));

    let actual =
        search.search(&world_step, pos(0, 0), &ants, 10, 500);
    assert_eq![
        actual,
        vec![
            pos(0, 100).west(),
            pos(0, 101).west(),
            pos(0, 102).west(),
            pos(0, 103).west(),
            pos(0, 104).west(),
            pos(0, 105).west(),
            pos(0, 106).west(),
            pos(0, 107).west(),
            pos(0, 108).west(),
            pos(0, 109).west()
        ]
    ];
}

fn search_bfs(b: &mut Bencher) {
    b.iter(|| search(Box::new(BFS {})));
}

fn search_a_star(b: &mut Bencher) {
    b.iter(|| search(Box::new(RepeatedAStar {})));
}
benchmark_group!(benches, search_bfs, search_a_star);
benchmark_main!(benches);
