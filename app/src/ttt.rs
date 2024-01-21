use std::iter::repeat;

#[allow(clippy::wildcard_imports)]
use leptos::*;

use crate::counter_test::SimpleCounter;

#[derive(Debug, Clone, Copy)]
enum PlayerSymbol {
    X,
    O,
}

impl ToString for PlayerSymbol {
    fn to_string(&self) -> String {
        match self {
            Self::X => "X",
            Self::O => "O",
        }
        .to_owned()
    }
}

#[derive(Clone)]
struct CurrentPlayer(RwSignal<PlayerSymbol>);

#[component]
#[must_use]
pub fn TicTacToe() -> impl IntoView {
    let current_player = CurrentPlayer(create_rw_signal(PlayerSymbol::X));

    provide_context(current_player);

    let fields = repeat(None::<PlayerSymbol>)
        .take(9)
        .map(create_rw_signal)
        .collect::<Vec<_>>();

    view! {
        <SimpleCounter />

        <div class="inline-grid grid-cols-3 gap-2">
            <For
                // a function that returns the items we're iterating over; a signal is fine
                each=move || fields.clone().into_iter().enumerate()
                // a unique key for each item
                key=|counter| counter.0
                // renders each item to a view
                children=move |field| {
                    view! { <Field symbol=field.1/> }
                }
            />
        </div>
    }
}

#[component]
#[must_use]
fn Field(symbol: RwSignal<Option<PlayerSymbol>>) -> impl IntoView {
    let current_player = expect_context::<CurrentPlayer>();
    let click = move |_| {
        symbol.set(Some(current_player.0.get()));
        current_player.0.set(match current_player.0() {
            PlayerSymbol::X => PlayerSymbol::O,
            PlayerSymbol::O => PlayerSymbol::X,
        })
    };

    view! {
        <button class="btn btn-outline btn-lg btn-square" on:click=click >
          {move || symbol().map_or_else(|| " ".to_owned(), |player| player.to_string())}
        </button>
    }
}
