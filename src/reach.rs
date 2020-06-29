use ants_ai_challenge_api::*;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

#[cfg(test)]
fn blank_mask_from_radius(radius2: u64) -> Vec<Coord> {
    let reach = (radius2 as f64).sqrt().round() as i32;
    let mut mask: Vec<Coord> = vec![];

    for y in -reach..=reach {
        for x in -reach..=reach {
            let abs = (y * y + x * x) as u64;
            if abs <= radius2 {
                mask.push(Coord { x, y });
            }
        }
    }

    mask
}

fn world_mask_from_blank(
    blank_mask: Vec<Coord>,
    mask_offset: Position,
    world_size: Position,
) -> HashSet<Position> {
    blank_mask
        .iter()
        .map(|c| {
            let x = c.x as i64 + mask_offset.col as i64;
            let y = c.y as i64 + mask_offset.row as i64;
            world_size.as_size_for_pos(y, x)
        })
        .collect::<HashSet<Position>>()
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! assert_blank_mask {
        ($radius2:expr, $( ($x:expr, $y:expr)),* ) => {

            let mut expected_mask: Vec<Coord> = Vec::new();
            $(
                expected_mask.push(Coord{x:$x, y:$y});
            )*
            let actual_mask = super::blank_mask_from_radius($radius2);
            assert_eq!(actual_mask, expected_mask);
        }
    }

    #[test]
    fn blank_mask_from_radius() {
        assert_blank_mask!(0, (0, 0));
        assert_blank_mask!(
            1,
            (0, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (0, 1)
        );
        assert_blank_mask!(
            2,
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1)
        );
        assert_blank_mask!(
            4,
            (0, -2),
            (-1, -1),
            (0, -1),
            (1, -1),
            (-2, 0),
            (-1, 0),
            (0, 0),
            (1, 0),
            (2, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (0, 2)
        );
    }

    fn assert_world_mask(
        radius2: u64,
        single_ant_world: &'static str,
        expected_mask: &'static str,
    ) {
        let expected =
            crate::utilities::positions_of_x(expected_mask);

        let mask_offset =
            crate::utilities::positions_of('a', single_ant_world)
                .iter()
                .last()
                .expect("world should contain at least one ant 'a'")
                .clone();

        let world_size =
            crate::utilities::size_of_world(single_ant_world);

        let blank_mask = super::blank_mask_from_radius(radius2);
        let actual = super::world_mask_from_blank(
            blank_mask,
            mask_offset,
            world_size,
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn mask_from_radius() {
        assert_world_mask(
            0,
            "
            ---
            -a-
            ---",
            "
            ---
            -x-
            ---",
        );

        assert_world_mask(
            4,
            "
            -------
            -------
            ---a---
            -------
            -------",
            "
            ---x---
            --xxx--
            -xxxxx-
            --xxx--
            ---x---",
        );

        // wrap and overlap
        assert_world_mask(
            1,
            "
            ---a
            ----",
            "
            x-xx
            ---x",
        );
    }
}
