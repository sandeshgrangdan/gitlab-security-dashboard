use yew::prelude::*;

#[function_component]
pub fn RightNavbar() -> Html {
    html! {
         <div class="cursor-pointer flex bg-stone-800 p-1 rounded-full">
            <img class="size-8 rounded-full m-1 shadow-sm shadow-black" src="/public/innovatetech.png" />
           // <span class="inset-shadow-indigo-500/50 flex items-center justify-center size-8 bg-orange-800 rounded-full font-bold text-xl text-stone-950 m-1 shadow-sm shadow-black">{"S"}</span>
         </div>
    }
}
