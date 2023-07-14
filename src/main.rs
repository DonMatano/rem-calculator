use leptos::*;
fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let doubleCount = move || count() * 2;
    view! { cx,
        <button
            class:red=move || count() % 2 == 1
            on:click=move |_| {
            set_count.update(|n| *n += 1)
        }
        >"Click me: " {count}</button>
        <ProgressBar progress=count max=10 />
        <ProgressBar progress=Signal::derive(cx, doubleCount) max=20 />
    }
}

/// Show progress to a goal
#[component]
fn ProgressBar(
    cx: Scope,
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be shown.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { cx,
        <progress
            max=max
            value=progress
        />

    }
}
