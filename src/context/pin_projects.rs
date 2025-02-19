use std::rc::Rc;

use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default, Deserialize)]
pub struct PinProject {
    pub full_path: String,
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct PinProjectList {
    pub data: Vec<PinProject>
}

impl PinProjectList {
    pub fn new() -> Self {

        let data: Vec<PinProject> = vec![];

        return PinProjectList { data };
    }
}

impl Reducible for PinProjectList {
    type Action = Vec<PinProject>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<PinProjectList> {
        PinProjectList { data: action }.into()
    }
}
pub type PinProjectListContext = UseReducerHandle<PinProjectList>;

#[derive(Properties, Debug, PartialEq)]
pub struct PinProjectListProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn PinProjectListProvider(props: &PinProjectListProps) -> Html {
    let msg = use_reducer(|| PinProjectList::new());
    

    html! {
        <ContextProvider<PinProjectListContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<PinProjectListContext>>
    }
}
