use yew::prelude::*;
use web_sys::window;
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::callback::Timeout;

use crate::{atoms::{search::Search, Home, ReloadIcon}, components::projects::Projects, container::home::Route};

#[function_component]
pub fn MidNavbar() -> Html {
    let search: UseStateHandle<String> = use_state(|| String::new());

    let is_search = use_state(|| false);

    let is_search_clone = is_search.clone();
    let onfocus: Callback<()> = Callback::from(move |_| {
        is_search_clone.set(true);
    });

    let is_search_clone = is_search.clone();
    let onblur: Callback<()> = Callback::from(move |_| {
        let is_search_clone = is_search_clone.clone();
        spawn_local(async move {
            let timeout = Timeout::new(100, move || {
                is_search_clone.set(false);
            });
            timeout.forget();
        });
    });

    let oninput = {
        let input_value = search.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                input_value.set(input.value());
            }
        })
    };

    let navigator = use_navigator().unwrap();

    let onclick = Callback::from({
        let input_value = search.clone();

        move |_| {
            input_value.set(String::new());
            navigator.push(&Route::Home);
        }
    });

    let reload_page = Callback::from({
        move |_| {
            if let Some(win) = window() {
                win.location().reload().expect("Failed to reload");
            }
        }
    });


    let route = use_route::<Route>().expect("Failed to get route");

    let path = match route {
        Route::Home => String::from("Home"),
        _ => String::from("No ID found"),
    };

    html! {
        <div class="w-[32%] relative">
            <div class="flex items-center w-full gap-2">
                <Home onclick={onclick.clone()}  path={path.clone()}/>
                <div class={
                    if *is_search {
                        "rounded-full overflow-hidden outline-2 w-full outline-gray-300"
                    } else {
                        "rounded-full overflow-hidden w-full"
                    }
                }>
                    <Search onfocus={onfocus} onblur={onblur} oninput={oninput} value={(*search).clone()} class="!text-base" />
                </div>
                <ReloadIcon onclick={reload_page}/>
            </div>
            {
                if *is_search {
                    html!{
                        <div class="overflow-y-auto w-full h-100 mt-1 absolute bg-neutral-800 rounded-lg shadow-lg shadow-stone-950 z-9 custom-scrollbar">
                            <p class="text-gray-300 mt-4 text-base pl-3 font-extrabold">{"Searches"}</p>
                            <Projects filter={(*search).clone()} icon_class="!min-w-12 !min-h-12" update_search={search} />
                        </div>
                    }
                } else {
                    html!{<></>}
                }
            }
            
         </div>
    }
}
