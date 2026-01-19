use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CounterState {
    pub count: i32,
}

#[lithe::rpc]
pub async fn increment(state: CounterState) -> CounterState {
    CounterState {
        count: state.count + 1,
    }
}

#[lithe::rpc]
pub async fn decrement(state: CounterState) -> CounterState {
    CounterState {
        count: state.count - 1,
    }
}
