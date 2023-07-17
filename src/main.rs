use leptos::{ev::InputEvent, *};
fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

enum InputType {
    Rem,
    Px,
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (px_value, set_px_value) = create_signal(cx, 16.0);
    let rem_value = px_value() / 16.0;
    fn verify_value(value: &str) -> f32 {
        value.parse::<f32>().expect("Invalid value")
    }

    let update_rem_value = move |e: InputEvent| {
        let res = verify_value(&event_target_value(&e)) * 16.0;
        set_px_value(res);
    };

    let handle_px_input = move |e: InputEvent| {
        let res = verify_value(&event_target_value(&e));
        set_px_value(res);
    };

    view! { cx,
        <CalculatedInput on:input=update_rem_value calculated_value=rem_value  set_value=Signal::derive(cx, set_rem_value(px_value)) label="Rem" input_type=InputType::Rem />
        <CalculatedInput calculated_value=px_value set_value=set_px_value label="Px" input_type=InputType::Px />
    }
}

#[component]
fn CalculatedInput<F>(
    cx: Scope,
    #[prop(into)] calculated_value: Signal<f32>,
    label: &'static str,
    on_value_changed: F,
) -> impl IntoView
where
    F: Fn(InputEvent) + 'static,
{
    view! {cx,
        <label>{label}</label>

        <input type="number"
        on:input=on_value_changed
        placeholder={label} value={calculated_value} props:value={calculated_value} />
    }
}
