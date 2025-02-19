use std::rc::Rc;

use crate::api::ApiClient;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Api {
    pub client: ApiClient,
}

impl Reducible for Api {
    type Action = ApiClient;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Api { client: action }.into()
    }
}
pub type ApiContext = UseReducerHandle<Api>;

#[derive(Properties, Debug, PartialEq)]
pub struct ApiProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn ApiProvider(props: &ApiProviderProps) -> Html {
    let msg = use_reducer(|| Api {
        client: ApiClient::default(),
    });

    html! {
        <ContextProvider<ApiContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<ApiContext>>
    }
}
