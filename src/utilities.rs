use ants_ai_challenge_api::*;

/// Create a world representation from multi
/// line string representing the world as a 2d
/// map with coordinate (0,0) as the first
/// character.
///
/// .   = land
/// %   = water
/// *   = food
/// !   = dead ant or ants
/// ?   = unseen territory
/// a-j = ant
/// A-J = ant on its own hill
/// 0-9 = hill
///
pub fn world(multi_line_map: &'static str) -> WorldState {
    let map = trim_lines(multi_line_map);

    let x =
        map.lines().zip(Indexer::new()).flat_map(|(line, row)| {
            line.chars().zip(Indexer::new()).map(
                move |(ch, column)| {
                    (ch, pos(row as u16, column as u16))
                },
            )
        });

    x.fold(WorldState::default(), |world, (ch, pos)| match ch {
        'a' => world.live_ant(pos, 0),
        'b' => world.live_ant(pos, 1),
        'c' => world.live_ant(pos, 2),
        'd' => world.live_ant(pos, 3),
        'e' => world.live_ant(pos, 4),
        'f' => world.live_ant(pos, 5),
        'g' => world.live_ant(pos, 6),
        'h' => world.live_ant(pos, 7),
        'i' => world.live_ant(pos, 8),
        'j' => world.live_ant(pos, 9),
        'A' => world.live_ant(pos.clone(), 0).hill(pos, 0),
        'B' => world.live_ant(pos.clone(), 1).hill(pos, 1),
        'C' => world.live_ant(pos.clone(), 2).hill(pos, 2),
        'D' => world.live_ant(pos.clone(), 3).hill(pos, 3),
        'E' => world.live_ant(pos.clone(), 4).hill(pos, 4),
        'F' => world.live_ant(pos.clone(), 5).hill(pos, 5),
        'G' => world.live_ant(pos.clone(), 6).hill(pos, 6),
        'H' => world.live_ant(pos.clone(), 7).hill(pos, 7),
        'I' => world.live_ant(pos.clone(), 8).hill(pos, 8),
        'J' => world.live_ant(pos.clone(), 9).hill(pos, 9),
        '0' => world.hill(pos, 0),
        '1' => world.hill(pos, 1),
        '2' => world.hill(pos, 2),
        '3' => world.hill(pos, 3),
        '4' => world.hill(pos, 4),
        '5' => world.hill(pos, 5),
        '6' => world.hill(pos, 6),
        '7' => world.hill(pos, 7),
        '8' => world.hill(pos, 8),
        '9' => world.hill(pos, 9),
        '*' => world.food(pos),
        '%' => world.water(pos),
        _ => world,
    })
}

/// Trim away left and right padding of multi line string
/// and removes empty lines.
fn trim_lines(multi_lines: &'static str) -> String {
    let result: Vec<_> = multi_lines
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .collect();

    result.join("\n")
}

struct Indexer {
    index: usize,
}
impl Indexer {
    fn new() -> Indexer {
        Indexer { index: 0 }
    }
}
impl Iterator for Indexer {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let result = Some(self.index);
        self.index += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_success() {
        assert_eq!(
            WorldState::default()
                .live_ant(pos(0, 1), 0)
                .live_ant(pos(0, 2), 1)
                .live_ant(pos(0, 3), 2)
                .food(pos(2, 2))
                .water(pos(3, 0))
                .hill(pos(1, 5), 4)
                .hill(pos(2, 5), 0)
                .live_ant(pos(2, 5), 0),
            world(
                "-abc---
                 -----4-
                 --*--A-
                 %----§--"
            )
        )
    }

    #[test]
    fn trim_lines_success() {
        assert_eq!(
            "a\nb\nc",
            trim_lines(
                "a

                 b
                 c  "
            )
        )
    }

}
