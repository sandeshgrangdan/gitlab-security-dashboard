pub mod left_navbar;
pub mod mid_navbar;
pub mod report_table;
pub mod right_navbar;
pub mod top_sidebar;

use yew::prelude::*;

use crate::atoms::BugSeverityLevel;

// for BugSeverityIndicator
#[derive(Clone, PartialEq, Properties)]
pub struct BugSeverityProps {
    pub level: String,
    pub number: usize,
}

#[function_component]
pub fn BugSeverityIndicator(props: &BugSeverityProps) -> Html {
    let BugSeverityProps { level, number } = props;

    html! {
        <div class="flex flex-col min-w-40 rounded-md border border-zinc-600">
            <BugSeverityLevel
                level={level.to_owned()}
            />

            <div class="py-2 flex border-t border-zinc-600 items-center justify-center">
                <span class="ml-3 text-xl font-black">{ number }</span>
            </div>
        </div>
    }
}
