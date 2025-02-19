use yew::prelude::*;

use crate::container::home::Home;
use crate::context::api_context::ApiProvider;
use crate::context::detail_context::ProjectDetailProvider;
use crate::context::pin_projects::PinProjectListProvider;
use crate::context::project_context::ProjectProvider;
use crate::context::project_list_context::ProjectListProvider;

#[function_component]
pub fn App() -> Html {
    html! {
        // `ctx` is type `Rc<Use,StateHandle<Theme>>` while we need `Theme`
        // so we deref it.
        // It derefs to `&Theme`, hence the clone
        <ApiProvider>
            <ProjectListProvider>
                <PinProjectListProvider>
                    <ProjectProvider>
                        <ProjectDetailProvider>
                        // Every child here and their children will have access to this context.
                            <Home />
                        </ProjectDetailProvider>
                    </ProjectProvider>
                </PinProjectListProvider>
            </ProjectListProvider>
        </ApiProvider>
    }
}
