use ants_ai_challenge_api::*;

/// Game world simulation step state.
pub trait WorldStep {
    // Add an ant movement order.
    fn add_order(self, order: Order) -> Self;

    // Get all effective orders accumulated in this step state.
    fn get_orders(self) -> Orders;
}
