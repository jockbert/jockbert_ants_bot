use ants_ai_challenge_api::*;

/// Game world simulation step state.
pub trait WorldStep {
    // Add an ant movement order.
    fn add_order(&mut self, order: Order) -> &mut WorldStep;

    // Get all effective orders accumulated in this step state.
    fn get_orders(&mut self) -> Orders;

    // World size
    fn size(&self) -> Position;

    fn all_my_ants(&self) -> Vec<Position>;

    fn available_directions(&self, pos: Position) -> Vec<Direction>;
}

pub struct BasicWorldStep {
    given_orders: Vec<Order>,
    world: WorldState,
    size: Position,
}

impl BasicWorldStep {
    pub fn new(world: WorldState, size: Position) -> BasicWorldStep {
        BasicWorldStep {
            given_orders: vec![],
            world: world,
            size: size,
        }
    }
}

impl WorldStep for BasicWorldStep {
    fn add_order(&mut self, order: Order) -> &mut WorldStep {
        self.given_orders.push(order);
        self
    }

    fn get_orders(&mut self) -> Orders {
        self.given_orders.clone()
    }

    fn size(&self) -> Position {
        self.size.clone()
    }

    fn all_my_ants(&self) -> Vec<Position> {
        self.world.live_ants_for_player(0)
    }

    fn available_directions(&self, _pos: Position) -> Vec<Direction> {
        vec![North, South, East, West]
    }
}
