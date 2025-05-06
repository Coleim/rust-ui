pub mod addtaskbar;
pub mod tasklist;

use yew::prelude::*;
use addtaskbar::AddTaskBar;
use tasklist::TaskList;

#[function_component]
fn App() -> Html {
    let tasks = use_state(|| vec![]);
    let on_task_add = {
        let tasks = tasks.clone();
        Callback::from(move|task: String| {
            log::info!("Task added: {}", task);
            tasks.set({
                let mut new_tasks = (*tasks).clone();
                new_tasks.push(task);
                new_tasks
            });
        })
    };
    html! {
        <div>
            <h1 class={classes!("main-title")}>{ "Todo Yew" }</h1>
            <div class={classes!("main-container")}>
                <AddTaskBar on_add_task={on_task_add}/>
                <TaskList tasks={(*tasks).clone()}/>
           </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
