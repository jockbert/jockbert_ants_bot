use crate::strategy::*;

//enum CombatMode {
//    StandOff,
//    Retract
//}

pub struct Combat {
    attack_radius_2: u16,
}

impl Strategy for Combat {
    fn apply(
        &self,
        _world_step: &dyn WorldStep,
        _ants_available: &HashSet<Position>,
    ) -> Orders {
        vec![]
    }
}

#[cfg(test)]

mod tests {

    use super::*;
    use crate::utilities::*;
    use crate::world_step::*;

    #[test]
    #[ignore = "not implemented"]
    fn standoff_one_on_one() {
        given()
            .attack_radius(2)
            .world_state("---b---a---")
            .assert_dist("32101234567")
            .assert_move("-------P---")
            //.assert_defe("---0---0---")
            //.assert_area("xxxxxxxxxxx")
            //.assert_dead("-----------")
            ;
    }

    #[test]
    #[ignore = "not implemented"]
    fn retract_one_on_one() {
        given()
            .attack_radius(2)
            .world_state("---b--a----")
            .assert_dist("32101234567")
            .assert_move("------>----")
            //.assert_dead("xxxxxxx----")
            //.assert_take("---xxxxxxx-")
            ;
    }

    #[test]
    #[ignore = "not implemented"]
    fn advance_one_on_one() {
        given()
            .attack_radius(2)
            .world_state("---b----a---")
            .assert_move("--------<---");
    }

    #[test]
    #[ignore = "not implemented"]
    fn ignored_when_no_opponent() {
        given()
            .attack_radius(2)
            .world_state("-------a---")
            .assert_move("-----------");
    }

    #[test]
    #[ignore = "not implemented"]
    fn regroup_far_away_ant() {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn ignore_when_alone_far_away() {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn regroup_when_two_far_away() {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn attack_on_safe_gain() {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn standoff_on_unsafe_gain() {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn standoff_five_on_five() {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn allow_symetric_decimation_at_massive_limit() {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn block_asymetric_decimation_at_massive_limit() {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn standoff_below_advantage_limit() {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn reckless_attack_at_advantage_limit() {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn massive_battle_does_not_programatically_fail() {
        unimplemented!()
    }

    // multiple enemies

    // multiple attack clusters

    // including water

    struct TestFixture {
        world_state: Option<&'static str>,
        attack_radius_2: Option<u16>,
    }

    fn given() -> TestFixture {
        TestFixture {
            world_state: None,
            attack_radius_2: None,
        }
    }

    impl TestFixture {
        fn world_state(mut self, world: &'static str) -> Self {
            self.world_state = Option::Some(world);
            self
        }

        fn attack_radius(mut self, ar: u8) -> Self {
            self.attack_radius_2 = Some(ar as u16 * ar as u16);
            self
        }

        fn assert_dist(self, _distance: &'static str) -> Self {
            // let expected_distances: Vec<Vec<u8>> =
            //    crate::utilities::value_matrix(distance);

            unimplemented!();
        }

        fn assert_move(&self, ord: &'static str) -> &Self {
            let ws = self
                .world_state
                .expect("World State should be specified");

            let world_step = AvoidWaterFilter::new_from_line_map(ws);

            let expected_orders = crate::utilities::orders(ord);

            let combat = self.create_combat();

            let ants_available: HashSet<Position> =
                crate::utilities::positions_of('a', ws);

            let actual_orders = combat
                .apply(&world_step, &ants_available)
                .iter()
                .cloned()
                .collect::<HashSet<_>>();

            assert_eq!(expected_orders, actual_orders);
            self
        }

        fn create_combat(&self) -> Combat {
            Combat {
                attack_radius_2: self
                    .attack_radius_2
                    .expect("Attack radius should be specified"),
            }
        }
    }
}
