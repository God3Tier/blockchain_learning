pub trait StateMachine {
    type State;
    type Transition;

    fn switch_state_next(starting_state: &Self::State, t: &Self::Transition) -> Self::State;

    fn human_readable() -> String {
        "unknown state machine".to_string()
    }
}