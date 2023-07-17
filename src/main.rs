use leptos::{ev::Event, html::Input, *};
fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (px_value, set_px_value) = create_signal(cx, 16.0);
    let rem_value = move || px_value() / 16.0;
    let px_input_ref = create_node_ref::<html::Input>(cx);
    let rem_input_ref = create_node_ref::<html::Input>(cx);

    fn verify_value(value: &str) -> f32 {
        value.parse::<f32>().unwrap_or(1.0)
    }

    let update_rem_value = move |e: Event| {
        let input_value = event_target_value(&e);
        let res = verify_value(&input_value) * 16.0;
        set_px_value(res);
        if let Some(input) = px_input_ref.get() {
            input.set_value(&px_value().to_string());
        }
    };

    let handle_px_input = move |e: Event| {
        let input_value = event_target_value(&e);
        let res = verify_value(&input_value);
        set_px_value(res);
        if let Some(input) = rem_input_ref.get() {
            input.set_value(&rem_value().to_string());
        }
    };

    view! { cx,
        <CalculatedInput
            on:input=update_rem_value
            input_ref=rem_input_ref
            calculated_value=Signal::derive(cx, rem_value)
            label="Rem"  />
        <CalculatedInput
            on:input=handle_px_input
            input_ref=px_input_ref
            calculated_value=px_value
            label="Px"
        />
    }
}

#[component]
fn CalculatedInput(
    cx: Scope,
    #[prop(into)] calculated_value: Signal<f32>,
    label: &'static str,
    input_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {cx,
        <label>{label}</label>
        <input type="number"
        node_ref=input_ref
        placeholder={label} value={calculated_value} props:value={calculated_value} />
    }
}
