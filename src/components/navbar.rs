use yew::prelude::*;

use crate::molecules::{left_navbar::LeftNavbar, mid_navbar::MidNavbar, right_navbar::RightNavbar};

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <div class="flex items-center justify-between my-2 w-full">
            <LeftNavbar/>
            <MidNavbar/>
            <RightNavbar/>
        </div>
    }
}
