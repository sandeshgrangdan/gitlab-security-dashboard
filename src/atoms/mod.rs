pub mod search;

use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Clone, PartialEq, Properties, Default)]
pub struct LinkProps {
    pub url: String,
    pub class: String,
    pub name: String,
}

#[function_component]
pub fn Link(props: &LinkProps) -> Html {
    let LinkProps { url, class, name} = props;

    html! {
            <a class={format!("group {}",class)} href={url.clone()} target="_blank">
                {
                    if name.is_empty() {
                        url
                    } else {
                        name
                    }
                }
            </a>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct HelpIconProps {
    pub help: String,
    pub head: String,
    pub class: String,
}

#[function_component]
pub fn HelpIcon(props: &HelpIconProps) -> Html {
    let HelpIconProps { head, help, class} = props;
    let default_class = "cursor-pointer hover:scale-105 transition-transform duration-100";

    html! {
            <a class={format!("relative group inline-block {}",class)}>
                <Icon
                    class={default_class}
                    icon_id={IconId::HeroiconsOutlineQuestionMarkCircle}
                    onclick={Callback::from(|_: MouseEvent| {})}
                    height={"1.3em".to_owned()}
                    width={"1.3em".to_owned()}
                />
                <div class="absolute left-0 right-0 w-90 !z-40 text-base px-2 py-2 flex flex-row mt-2 hidden group-hover:block bg-orange-900 text-white text-sm px-2 py-1 rounded shadow-lg">
                    <p class="font-bold text-gray-300 mb-2">{format!("What is {}?",head)}</p>
                    <span class="text-gray-400">
                        {
                            help
                        }
                    </span>
                </div>

            </a>
    }
}

#[function_component]
pub fn Logo() -> Html {
    html! {
        <div class="flex gap-3 items-center ml-3">
            <img src="/public/gitlab.svg" />
            <h1 class="text-lg text-neutral-400 font-extrabold">{ "Security!" }</h1>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub onclick: Callback<()>, 
    pub class: String
}

#[function_component]
pub fn GitBranch(props: &IconProps) -> Html {

    let onclick = props.onclick.clone();
    let class = props.class.clone();
    html! {
        <div class={format!("text-gray-300 p-1 hover:!bg-neutral-800 rounded-lg hover:scale-105 transition-transform duration-100 {}",class)}>
            <Icon
                icon_id={IconId::OcticonsGitBranch16}
                onclick={move |_| onclick.emit(())}
                height={"1em".to_owned()}
                width={"1em".to_owned()}
            />
        </div>
    }
}

#[function_component]
pub fn CopyIcon(props: &IconProps) -> Html {

    let onclick = props.onclick.clone();
    let class = props.class.clone();
    html! {
        <div class={format!("text-gray-300 p-1 hover:!bg-neutral-800 rounded-lg hover:scale-105 transition-transform duration-100 {}",class)}>
            <Icon
                icon_id={IconId::LucideClipboardCopy}
                onclick={move |_| onclick.emit(())}
                height={"1em".to_owned()}
                width={"1em".to_owned()}
            />
        </div>
    }
}


#[function_component]
pub fn LeftArrow(props: &IconProps) -> Html {

    let onclick = props.onclick.clone();
    let class = props.class.clone();
    html! {
        <div class={format!("text-gray-500 {}",class)}>
            <Icon
                icon_id={IconId::HeroiconsOutlineChevronLeft}
                onclick={move |_| onclick.emit(())}
                height={"1.8em".to_owned()}
                width={"1.8em".to_owned()}
            />
        </div>
    }
}

#[function_component]
pub fn RightArrow(props: &IconProps) -> Html {

    let onclick = props.onclick.clone();
    let class = props.class.clone();

    html! {
        <div class={format!("text-gray-500 {}",class)}>
            <Icon
                icon_id={IconId::HeroiconsOutlineChevronRight}
                onclick={move |_| onclick.emit(())}
                height={"1.8em".to_owned()}
                width={"1.8em".to_owned()}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ReloadIconProps {
    pub onclick: Callback<()>,
}

#[function_component]
pub fn ReloadIcon(props: &ReloadIconProps) -> Html {

    let onclick = props.onclick.clone();

    html! {
        <div class="cursor-pointer hover:scale-105 transition-transform duration-100 bg-neutral-800 rounded-full p-2">
            <Icon
                icon_id={
                    IconId::BootstrapArrowClockwise
                }
                onclick={move |_| onclick.emit(())}
                height={"1.8em".to_owned()}
                width={"1.8em".to_owned()}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub onclick: Callback<()>,
    pub path: String, // Callback that takes no arguments
}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {

    let onclick = props.onclick.clone();
    let path = props.path.clone();
    html! {
        <div class="cursor-pointer hover:scale-105 transition-transform duration-100 bg-neutral-800 rounded-full p-2">
            <Icon
                icon_id={
                    if path == "Home" {
                        IconId::OcticonsHomeFill24
                    } else {
                        IconId::OcticonsHome24
                    }
                }
                onclick={move |_| onclick.emit(())}
                height={"1.8em".to_owned()}
                width={"1.8em".to_owned()}
            />
        </div>
    }
}

// BugSeverityIcon
// Props
#[derive(Clone, PartialEq, Properties)]
pub struct BugSeverityIconProps {
    icon: IconId,
    class: String,
}

#[function_component]
pub fn BugSeverityIcon(props: &BugSeverityIconProps) -> Html {
    let BugSeverityIconProps { icon, class } = props;
    let default_class = "cursor-pointer hover:scale-105 transition-transform duration-100";

    html! {
            <div class={class}>
                <Icon
                    class={default_class}
                    icon_id={icon.clone()}
                    onclick={Callback::from(|_: MouseEvent| {})}
                    height={"0.8em".to_owned()}
                    width={"0.8em".to_owned()}
                />
            </div>
    }
}

// BugSeverityIcon function_component
#[derive(Clone, PartialEq, Properties)]
pub struct BugSeverityLevelProps {
    pub level: String,
}

#[function_component]
pub fn BugSeverityLevel(props: &BugSeverityLevelProps) -> Html {
    let BugSeverityLevelProps { level } = props;

    let default_class = "flex items-center py-1 justify-center";

    if level == "Critical" {
        return html! {
            <div class={default_class}>
                <BugSeverityIcon class={"text-orange-600".to_owned()} icon={IconId::BootstrapHexagonFill}/>
                <span class="ml-2 text-md font-extrabold">{ level }</span>
            </div>
        };
    } else if level == "High" {
        return html! {
            <div class={default_class}>
                <BugSeverityIcon class={"text-orange-800".to_owned()} icon={IconId::BootstrapDiamondFill}/>
                <span class="ml-2 text-md font-extrabold">{ level }</span>
            </div>
        };
    } else if level == "Medium" {
        return html! {
            <div class={default_class}>
                <BugSeverityIcon class={"text-yellow-600".to_owned()} icon={IconId::BootstrapCaretDownFill}/>
                <span class="ml-2 text-md font-extrabold">{ level }</span>
            </div>
        };
    } else if level == "Low" {
        return html! {
            <div class={default_class}>
                <BugSeverityIcon class={"itext-yellow-600".to_owned()} icon={IconId::FontAwesomeSolidCircle}/>
                <span class="ml-2 text-md font-extrabold">{ level }</span>
            </div>
        };
    } else if level == "Info" {
        return html! {
            <div class={default_class}>
                <BugSeverityIcon class={"text-blue-400".to_owned()} icon={IconId::FontAwesomeSolidCircleExclamation}/>
                <span class="ml-2 text-md font-extrabold">{ level }</span>
            </div>
        };
    } else {
        return html! {
            <div class={default_class}>
                <BugSeverityIcon class={"text-neutral-400".to_owned()} icon={IconId::BootstrapQuestionCircleFill}/>
                <span class="ml-2 text-md font-extrabold">{ level }</span>
            </div>
        };
    }
}

#[function_component]
pub fn CheckBox() -> Html {
    let is_checked = use_state(|| false);

    let onclick = {
        let check_me = is_checked.clone();
        move |_| {
            check_me.set(!*check_me);
        }
    };

    let def_div_class = "size-4 border-1 border-neutral-600 rounded-sm flex items-center justify-center transition-all duration-200 ease-in-out";
    let def_svg_class = "w-3 h-3 text-neutral-950 font-bold";

    let (div_class, svg_class) =if  *is_checked {
        ("bg-sky-900 border-sky-900","")
    } else {
        ("opacity-0","")
    };

    html! {
        <div onclick={onclick} class={format!("{} {}",def_div_class,div_class )}>
            <svg class={format!("{} {}",def_svg_class, svg_class)} fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
            </svg>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct LoaderProps {
    pub class: String,
}
#[function_component]
pub fn Loader(props: &LoaderProps) -> Html {

    let LoaderProps { class } = props;

    html! {
        <div class="flex gap-2">
            <div class={format!("{} bg-orange-600 rounded-full animate-bounce [animation-delay:-0.3s]",class)}></div>
            <div class={format!("{} bg-orange-600 rounded-full animate-bounce [animation-delay:-0.15s]",class)}></div>
            <div class={format!("{} bg-orange-600 rounded-full animate-bounce",class)}></div>
        
            // <span class={format!("relative flex {}", class)}> 
            //     <span class="absolute inline-flex h-full w-full animate-bounce  animate-ping rounded-full bg-orange-500 opacity-75 [animation-delay:-0.3s]">
            //     </span>  
            //     <span class={format!("relative inline-flex rounded-full bg-orange-600 {}", class)}>
            //     </span>
            // </span>
            // <span class={format!("relative flex {}", class)}> 
            //     <span class="absolute inline-flex h-full w-full delay-150 animate-ping rounded-full bg-orange-500 opacity-75 [animation-delay:-0.15s]">
            //     </span>  
            //     <span class={format!("relative inline-flex rounded-full bg-orange-600 {}", class)}>
            //     </span>
            // </span>
            // <span class={format!("relative flex {}", class)}> 
            //     <span class="absolute inline-flex h-full w-full delay-300 animate-ping rounded-full bg-orange-500 opacity-75">
            //     </span>  
            //     <span class={format!("relative inline-flex rounded-full bg-orange-600 {}", class)}>
            //     </span>
            // </span>

        </div>
    }
}


