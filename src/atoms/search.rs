use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct SearchProps {
    pub onfocus: Callback<()>, // Callback that takes no arguments
    pub onblur: Callback<()>, // Callback that takes no arguments
    pub oninput: Callback<InputEvent>, // Callback that takes no arguments
    pub value: String,
    pub class: String,
}


#[function_component]
pub fn Search(props: &SearchProps) -> Html {
    let onfocus = props.onfocus.clone();
    let onblur = props.onblur.clone();
    let oninput = props.oninput.clone();
    let value = props.value.clone();
    let class = props.class.clone();

    html! {
        <div class={format!("flex items-center bg-neutral-800 rounded-lg pl-2 text-sm w-full {}",class)}>
            <Icon
                class="hover:scale-105 transition-transform duration-100"
                icon_id={IconId::LucideSearch}
                onclick={Callback::from(|_: MouseEvent| {})}
                width={"1.5em".to_owned()}
            />
            <input
                onfocus={move |_| onfocus.emit(())}
                onblur={move |_| onblur.emit(())}
                oninput={oninput}
                value={value}
                class="!bg-neutral-800 !shadow-none outline-transparent w-full"
                placeholder="What do you want to search."
            />
        </div>
    }
}
