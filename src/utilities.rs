use ants_ai_challenge_api::*;
use std::collections::HashSet;

/// Create an hash set
#[macro_export]
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
    fn offset_from(a: char, b: char) -> u8 {
        b as u8 - a as u8
    }

    chars_with_pos(multi_line_map).into_iter().fold(
        WorldState::default(),
        |world, (ch, pos)| match ch {
            '*' => world.food(pos),
            '%' => world.water(pos),
            c @ '0'..='9' => world.hill(pos, offset_from('0', c)),
            c @ 'a'..='j' => world.live_ant(pos, offset_from('a', c)),
            c @ 'A'..='J' => {
                let player = offset_from('A', c);
                world.live_ant(pos.clone(), player).hill(pos, player)
            }
            _ => world,
        },
    )
}

/// Retireves all positions of character 'x' in a 2D coordinate system.
pub fn positions_of_x(multi_lines: &str) -> HashSet<Position> {
    positions_of('x', multi_lines)
}

/// Retireves all positions of given character in a 2D coordinate system.
pub fn positions_of(
    char_to_find: char,
    multi_lines: &str,
) -> HashSet<Position> {
    chars_with_pos(multi_lines)
        .into_iter()
        .filter(|(ch, _)| *ch == char_to_find)
        .map(|(_, pos)| pos)
        .collect()
}

/// Retrieves all orders from character 2D coordinate system.
/// - 'W', 'w' and '<' indicates an order directed to the west.
/// - 'E', 'e' and '>' indicates an order directed to the east.
/// - 'N', 'n' and '^' indicates an order directed to the north.
/// - 'S', 's', 'V' and 'v' indicates an order directed to the south.
///
/// # #[macro_use] extern crate crate_name;
///
/// # Example
/// ```
/// use ants_ai_challenge_api::pos;
/// use jockbot_ants_bot::set;
/// use jockbot_ants_bot::utilities::orders;
///
/// assert_eq![
///     orders("
///         --*<
///         -*N-
///         "),
///     set![
///         pos(0,3).west(),
///         pos(1,2).north()]]
/// ```
pub fn orders(multi_line_map: &'static str) -> HashSet<Order> {
    let mut result = set!();

    chars_with_pos(multi_line_map).into_iter().for_each(
        |(ch, pos)| {
            match ch {
                '^' => result.insert(pos.north()),
                'V' => result.insert(pos.south()),
                'v' => result.insert(pos.south()),
                '<' => result.insert(pos.west()),
                '>' => result.insert(pos.east()),
                // ----
                'N' => result.insert(pos.north()),
                'S' => result.insert(pos.south()),
                'W' => result.insert(pos.west()),
                'E' => result.insert(pos.east()),
                // ----
                'n' => result.insert(pos.north()),
                's' => result.insert(pos.south()),
                'w' => result.insert(pos.west()),
                'e' => result.insert(pos.east()),
                _ => false,
            };
        },
    );

    result
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
/// use ants_ai_challenge_api::pos;
/// use jockbot_ants_bot::utilities::size_of_world;
///
/// let x = size_of_world("
///      aa
///      bb
///      cc
///      ");
///  assert_eq![x, pos(3,2)];
///  ```
///
pub fn size_of_world(multi_line_map: &'static str) -> Position {
    let trimmed_map = trim_lines(multi_line_map);
    let rows = trimmed_map.lines().count() as u16;
    let cols = trimmed_map
        .lines()
        .map(|line| line.len() as u16)
        .fold(0 as u16, u16::max);

    pos(rows, cols)
}

/// Splits a multi line string up into individual characters
/// and corresponding position.
fn chars_with_pos(map: &str) -> Vec<(char, Position)> {
    trim_lines(map)
        .lines()
        .zip(Indexer::new())
        .flat_map(|(line, row)| {
            line.chars().zip(Indexer::new()).map(
                move |(ch, column)| {
                    (ch, pos(row as u16, column as u16))
                },
            )
        })
        .collect()
}

/// Trim away left and right padding of multi line string
/// and removes empty lines.
pub fn trim_lines(multi_lines: &str) -> String {
    multi_lines
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
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

/// Assert that cut.get_orders equals expected orders.
#[cfg(test)]
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

/// Assert that cut.available_directions is empty.
#[cfg(test)]
macro_rules! assert_no_dirs {
        ($cut:expr, $position:expr) => {
            let expected_dirs = Vec::new();
            assert_eq!($cut.available_directions($position), expected_dirs);
        };
    }

/// Assert that cut.available_directions equals
/// expected directions.
#[cfg(test)]
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

    #[test]
    fn positions_of_success() {
        assert_eq!(
            set!(
                pos(0, 1),
                pos(0, 2),
                pos(1, 0),
                pos(1, 3),
                pos(6, 0)
            ),
            positions_of_x(
                "-xx-
                 x--x
                 -
                 X
                 a
                 -
                 x"
            )
        )
    }
}
