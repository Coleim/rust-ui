use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddTaskBarProps {
    pub on_add_task: Callback<String>,
}

#[function_component]
pub fn AddTaskBar(props: &AddTaskBarProps) -> Html {
    let input_value = use_state(|| String::new());
    let input_ref = use_node_ref();

    {
        let input_ref = input_ref.clone();
        let input_value = input_value.clone();
        use_effect_with(
            input_value.clone(),
            move |val| {
                if val.is_empty() {
                    if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                        let _ = input.focus();
                    }
                }
                || ()
            },
        );
    }

    let oninput = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    let onclick = {
        let input_value = input_value.clone();
        let add_task_prop = props.on_add_task.clone();
        Callback::from(move |_| {
            let task = (*input_value).clone();
            if !task.trim().is_empty() {
                add_task_prop.emit(task);
                input_value.set(String::new());
            }
        })
    };

    html! {
        <div>
            <input
                ref={input_ref.clone()}
                class={classes!("add-input")}
                placeholder="Add a new task"
                type="text"
                oninput={oninput}
                value={(*input_value).clone()}
            />
            <button class={classes!("add-btn")} onclick={onclick}>{ "Add Task" }</button>
        </div>
    }
}
