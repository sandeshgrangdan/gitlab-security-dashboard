use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::callback::Timeout;

use crate::{
    api::get_route_full_path, 
    atoms::{search::Search, Logo}, 
    container::home::Route, 
    context::{
        project_context::Project, 
        project_list_context::ProjectListContext
    }
};

#[derive(Properties, PartialEq)]
pub struct TopSidebarProps {
    pub oninput: Callback<String>,
    pub update_search: UseStateHandle<String>,
}

#[function_component]
pub fn TopSidebar(props : &TopSidebarProps) -> Html {
    let is_search = use_state(|| false);

    let route = use_route::<Route>().expect("Failed to get route");

    let route_full_path = get_route_full_path(route);

    let update_search = props.update_search.clone();

    let project_list_ctx = use_context::<ProjectListContext>().unwrap();

    let get_project = project_list_ctx.data
        .iter()
        .find(|p| p.full_path == route_full_path)
        .unwrap_or(&Project::default())
        .clone();

    let oninput = {
        let oninput = props.oninput.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                oninput.emit(input.value());
            }
        })
    };
    
    let onfocus: Callback<()> = Callback::from(move |_| {
        let is_search_clone = is_search.clone();
        spawn_local(async move {
            let timeout = Timeout::new(100, move || {
                is_search_clone.set(!*is_search_clone);
            });
            timeout.forget();
        });
    });

    let onblur: Callback<()> = Callback::from(move |_| {});

    html! {
        <div class="shadow shadow-stone-950 gap-2 flex flex-col p-2 rounded-md bg-neutral-900">
            <Logo/>
            <div class="flex gap-2 flex-wrap">
                {
                    if !get_project.full_path.is_empty() {
                        get_project.full_path.split("/").into_iter().map(|name| {
                            html!{
                                <span class="px-3 py-1 py bg-stone-800 rounded-full w-fit text-sm text-gray-300">{ name }</span>
                            }
    
                        }).collect::<Html>()
                    } else {
                        html!{<></>}
                    }
                }
            </div>
            <div class="mt-1">
                <Search oninput={oninput} onblur={onblur} onfocus={onfocus} value={(*update_search).clone()} class="" />
            </div>
         </div>
    }
}
