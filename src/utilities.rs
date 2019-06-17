use ants_ai_challenge_api::*;

/// Create a world representation from multi
/// line string representing the world as a 2d
/// map with coordinate (0,0) as the first
/// character.
///
/// ```
/// .   = land
/// %   = water
/// *   = food
/// !   = dead ant or ants
/// ?   = unseen territory
/// a-j = ant
/// A-J = ant on its own hill
/// 0-9 = hill
/// ```
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

    fn offset_from(a: char, b: char) -> u8 {
        b as u8 - a as u8
    }

    x.fold(WorldState::default(), |world, (ch, pos)| match ch {
        '*' => world.food(pos),
        '%' => world.water(pos),
        c @ '0'...'9' => world.hill(pos, offset_from('0', c)),
        c @ 'a'...'j' => world.live_ant(pos, offset_from('a', c)),
        c @ 'A'...'J' => {
            let player = offset_from('A', c);
            world.live_ant(pos.clone(), player).hill(pos, player)
        }
        _ => world,
    })
}

pub fn serialize_world(
    world: &WorldState,
    size: &Position,
) -> String {
    fn hill_owner(w: &WorldState, p: &Position) -> Option<u8> {
        for player in 0..w.hills.len() {
            if w.hills.get(player).expect("").contains(p) {
                return Some(player as u8);
            }
        }
        None
    }

    fn to_char(w: &WorldState, p: &Position) -> char {
        if w.foods.contains(p) {
            '*'
        } else if w.waters.contains(p) {
            '%'
        } else {
            for player in 0..w.live_ants.len() {
                if w.live_ants
                    .get(player)
                    .expect("Live ants")
                    .contains(p)
                {
                    return ((if hill_owner(w, p).is_some() {
                        b'A'
                    } else {
                        b'a'
                    }) + (player as u8))
                        as char;
                }
            }
            match hill_owner(w, p) {
                Some(owner) => (b'0' + owner) as char,
                None => '-',
            }
        }
    }

    let mut result = String::from("");

    for row in 0..size.row {
        for col in 0..size.col {
            result.push(to_char(&world, &pos(row, col)));
        }
        if row != size.row - 1 {
            result.push('\n');
        }
    }
    result
}

/// Calculates the size of a given textual multi line map. The
/// following example map has size (3,2):
///
/// ```
/// aa
/// bb
/// cc
/// ```
pub fn size_of_world(multi_line_map: &'static str) -> Position {
    let trimmed_map = trim_lines(multi_line_map);
    let rows = trimmed_map.lines().count() as u16;
    let cols = trimmed_map
        .lines()
        .map(|line| line.len() as u16)
        .fold(0 as u16, u16::max);

    pos(rows, cols)
}

/// Trim away left and right padding of multi line string
/// and removes empty lines.
fn trim_lines(multi_lines: &'static str) -> String {
    let result: Vec<_> = multi_lines
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
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

macro_rules! set(
    () => { ::std::collections::HashSet::new(); };
    ($($value:expr),+ ) => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($value);
            )+
            m
        }};
    );

/// Assert that dut.get_orders equals expected orders.
macro_rules! assert_orders {
        ($cut:expr) => {
            assert_eq![$cut.get_orders(),vec![]];
        };

        ($cut:expr, $( $order:expr ),*) => {
            let mut expected_orders = Vec::new();
            $(
                expected_orders.push($order);
            )*
            assert_eq!($cut.get_orders(),expected_orders);
        };
    }

/// Assert that dut.available_directions equals
/// expected directions.
macro_rules! assert_dirs {
        ($cut:expr, $position:expr, $( $dir:expr ),*) => {
            let mut expected_dirs = Vec::new();
            $(
                expected_dirs.push($dir);
            )*
            assert_eq!($cut.available_directions($position), expected_dirs);
        };
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
    fn world_serialization_roundtrip() {
        let data = "-abcdefghij-
                    -ABCDEFGHIJ-
                    -0123456789-
                    -%**--%%*%%-";

        assert_eq![
            trim_lines(&data),
            serialize_world(&world(data), &pos(4, 12))
        ];
    }

    #[test]
    fn size_of_world_success() {
        assert_eq![
            pos(3, 2),
            size_of_world(
                "a
                 bb

                 c
                 "
            )
        ]
    }

    #[test]
    fn trim_lines_success() {
        // second line contains whitespace
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
