use std::rc::Rc;

use yew::prelude::*;

use crate::molecules::report_table::Vulnarable;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ProjectDetail {
    pub data: Vulnarable,
    pub namespaces: String,
}

impl ProjectDetail {
    pub fn new(data: Vulnarable,namespaces: String) -> Self {
        Self{
            data,
            namespaces
        }
    }
}

impl Reducible for ProjectDetail {
    type Action = ProjectDetail;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }
}
pub type ProjectDetailContext = UseReducerHandle<ProjectDetail>;

#[derive(Properties, Debug, PartialEq)]
pub struct ProjectProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn ProjectDetailProvider(props: &ProjectProviderProps) -> Html {
    let msg = use_reducer(|| ProjectDetail::default());

    html! {
        <ContextProvider<ProjectDetailContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<ProjectDetailContext>>
    }
}
