use std::rc::Rc;

use yew::prelude::*;

use super::project_context::Project;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ProjectList {
    pub data: Vec<Project>
}

impl ProjectList {
    pub fn new() -> Self {

        let data: Vec<Project> = vec![];

        // let data= vec![
        //     Project {
        //         id: String::from("gid://gitlab/Project/48959789"),
        //         name: String::from("hsa-identity"),
        //         full_path: String::from("innovate-tech/hsa/hsa-identity"),
        //         avatar_url: None,
        //     },
        //     Project {
        //         id: String::from("gid://gitlab/Project/48959789"),
        //         name: String::from("TTS Backend"),
        //         full_path: String::from("innovate-tech/e-library/tts-backend"),
        //         avatar_url: None,
        //     },
        //     Project {
        //         id: String::from("gid://gitlab/Project/57899016"),
        //         name: String::from("hsa-chat"),
        //         full_path: String::from("innovate-tech/hsa/hsa-chat"),
        //         avatar_url: Some(String::from("https://picsum.photos/200/300")),
        //     },
        //     Project {
        //         id: String::from("gid://gitlab/Project/16537165"),
        //         name: String::from("apollo-api"),
        //         full_path: String::from("innovate-tech/apollo/apollo-backend/apollo-api"),
        //         avatar_url: None,
        //     },
        // ];

        return ProjectList { data };
    }
}

impl Reducible for ProjectList {
    type Action = Vec<Project>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<ProjectList> {
        ProjectList { data: action }.into()
    }
}
pub type ProjectListContext = UseReducerHandle<ProjectList>;

#[derive(Properties, Debug, PartialEq)]
pub struct ProjectListProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn ProjectListProvider(props: &ProjectListProps) -> Html {
    let msg = use_reducer(|| ProjectList::new());

    html! {
        <ContextProvider<ProjectListContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<ProjectListContext>>
    }
}
