use crate::tuiapp::TuiApp;

pub struct TuiBuilder<State: TuiApp + Default> {
    pub a: std::marker::PhantomData<State>,
}
impl<State: TuiApp + Default> TuiBuilder<State> {
    pub fn with_state(state: State) -> Self {
        Self {
            a: std::marker::PhantomData {},
        }
    }

    pub fn tick_rate(&mut self, t: usize) -> &mut Self {
        self
    }

    fn event_handler(&mut self) -> &mut Self {
        self
    }

    fn renderer(&mut self, a: impl FnOnce(State)) -> &mut Self {
        self
    }
    pub fn launch(&mut self) {}
}

impl<State: TuiApp + Default> Default for TuiBuilder<State> {
    fn default() -> Self {
        Self::with_state(State::default())
    }
}
