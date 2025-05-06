use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub tasks: Vec<String>,
}


pub struct TaskProp {
    pub task: String
}


#[function_component]
pub fn TaskList(props: &TaskListProps) -> Html {

    html! {
        <ul>
            { for (*props.tasks).iter().map(|task| html! { <li> { task }</li> }) }
        </ul>
   }
}

