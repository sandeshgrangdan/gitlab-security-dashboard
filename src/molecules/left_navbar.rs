use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::window;

use crate::atoms::{LeftArrow, RightArrow};

#[function_component]
pub fn LeftNavbar() -> Html {

    let can_go_forward = use_state(|| 0);
    let navigator = use_navigator().unwrap();

    let go_back = {
        let can_go_forward = can_go_forward.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            can_go_forward.set(*can_go_forward + 1);
            navigator.back();
        })
    };

    let go_forward = {
        let can_go_forward = can_go_forward.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            if *can_go_forward > 0 {
                can_go_forward.set(*can_go_forward - 1);
                navigator.forward();
            }
        })
    };

    let can_go_back = {
        window()
            .and_then(|w| w.history().ok())
            .map(|h| h.length().unwrap() > 1)
            .unwrap_or(false)
    };

    html! {
         <div class="flex w-[10%] gap-1">
            <LeftArrow onclick={go_back} class={
                if can_go_back {
                    "hover:scale-107 cursor-pointer hover:text-gray-200 !text-gray-300 font-bold"
                } else {
                    "cursor-not-allowed"
                }
            }
            
            />
            <RightArrow onclick={go_forward} class={
                if *can_go_forward > 0 {
                    "hover:scale-107 cursor-pointer hover:text-gray-200 !text-gray-300 font-bold"
                } else {
                    "cursor-not-allowed"
                }
            }/>
         </div>
    }
}
