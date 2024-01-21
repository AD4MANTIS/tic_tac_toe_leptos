#[allow(clippy::wildcard_imports)]
use leptos::*;

use self::field::Fields;
use self::symbol::PlayerSymbol;

mod field;
mod symbol;
mod win;

#[component]
#[must_use]
pub fn TicTacToe() -> impl IntoView {
    let current_player = create_rw_signal(PlayerSymbol::X);
    let won_player = create_rw_signal(None::<Option<PlayerSymbol>>);

    let fields = Fields::default();

    let after_click = move || {
        if win::has_won(current_player(), fields) {
            won_player.set(Some(Some(current_player())));
        } else if fields.0.iter().all(|x| x().is_some()) {
            won_player.set(Some(None));
        } else {
            current_player.set(match current_player() {
                PlayerSymbol::X => PlayerSymbol::O,
                PlayerSymbol::O => PlayerSymbol::X,
            })
        }
    };

    let reset = move |_| {
        won_player.set(None);

        for field in fields.0 {
            field.set(None)
        }
    };

    view! {
        <div class="inline-flex flex-col p-4 gap-3">
            <button class="btn btn-warning" on:click=reset>
                Reset
            </button>

            <div class="inline-grid grid-cols-3 gap-2">

                <For
                    // a function that returns the items we're iterating over; a signal is fine
                    each=move || fields.0.into_iter().enumerate()
                    // a unique key for each item
                    key=|counter| counter.0
                    // renders each item to a view
                    children=move |field| {
                        view! {
                            <Field
                                symbol=field.1.into()
                                on_click=move |_| {
                                    if field.1().is_some() {
                                        return;
                                    }
                                    field.1.set(Some(current_player()));
                                    after_click();
                                }
                            />
                        }
                    }
                />

            </div>

            <Show when=move || { won_player().is_some() }>
                <span>
                    {move || {
                        won_player()
                            .expect("Someone should have won")
                            .map_or_else(
                                || "Draw!".to_owned(),
                                |winner| format!("{} has Won!", winner.to_string()),
                            )
                    }}

                </span>
            </Show>
        </div>
    }
}

#[component]
#[must_use]
fn Field(
    symbol: Signal<Option<PlayerSymbol>>,
    #[prop(into)] on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            class="btn btn-outline btn-lg btn-square"
            class=("no-animation", move || symbol().is_some())
            on:click=move |_| on_click(())
        >
            {move || symbol().map_or_else(|| " ".to_owned(), |player| player.to_string())}
        </button>
    }
}
