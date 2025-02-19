use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Project {
    pub full_path: String,
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

impl Project {
    pub fn new(full_path: String,id: String, name: String) -> Self {
        Self{
            full_path,
            id,
            name,
            avatar_url: None
        }
    }
}

impl Reducible for Project {
    type Action = Project;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }
}
pub type ProjectContext = UseReducerHandle<Project>;

#[derive(Properties, Debug, PartialEq)]
pub struct ProjectProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn ProjectProvider(props: &ProjectProviderProps) -> Html {
    let msg = use_reducer(|| Project::default());

    html! {
        <ContextProvider<ProjectContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<ProjectContext>>
    }
}
