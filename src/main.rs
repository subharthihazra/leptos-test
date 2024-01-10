use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <button
                on:click=move |_| {
                    set_count.update(|x| *x+=3);
                }
                class:pika=move || {count.get() % 2 == 1}
            >
                "Click me: "
                {count}
            </button>
            <SimpleCounter />
        </div>
    }
}

#[component]
pub fn SimpleCounter() -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(0);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_ev| set_value.set(0);
    let decrement = move |_ev| set_value.update(|value| *value -= 1);
    let increment = move |_ev| set_value.update(|value| *value += 1);

    view! {
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement class="pudu">"-1"</button>
            <span>"Value: " {move || value.get().to_string()} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}