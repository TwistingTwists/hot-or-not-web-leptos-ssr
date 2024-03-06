mod history;

use candid::Principal;
use leptos::*;
use leptos_icons::*;
use leptos_router::create_query_signal;
use leptos_use::use_window;

use crate::{
    component::connect::ConnectLogin,
    state::{auth::account_connected_reader, canisters::authenticated_canisters},
    try_or_redirect_opt,
};
use history::HistoryView;

#[component]
fn WorkButton(#[prop(into)] text: String, #[prop(into)] icon: icondata::Icon) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-3">
            <div class="grid h-12 w-12 place-items-center rounded-sm bg-white/10">
                <Icon class="text-orange-600 text-xl" icon=icon/>
            </div>
            <span class="text-xs md:text-sm">{text}</span>
        </div>
    }
}

#[component]
fn ReferLoaded(user_canister: Principal) -> impl IntoView {
    let refer_code = user_canister.to_text();
    let window = use_window();
    let refer_link = window
        .as_ref()
        .and_then(|w| {
            let origin = w.location().origin().ok()?;
            Some(format!(
                "{}/?user_refer={}",
                origin,
                user_canister.to_text()
            ))
        })
        .unwrap_or_default();
    let copy_link = move || {
        let navigator = window.navigator()?;
        _ = navigator.clipboard()?.write_text(&refer_link);
        Some(())
    };

    view! {
        <div class="flex items-center w-fit rounded-full border-dashed border-2 p-3 gap-2 border-orange-500">
            <span class="text-md lg:text-lg text-ellipsis line-clamp-1">{refer_code}</span>
            <button on:click=move |_| _ = copy_link()>
                <Icon class="text-xl" icon=icondata::FaCopyRegular/>
            </button>
        </div>
    }
}

#[component]
fn ReferLoading() -> impl IntoView {
    view! {
        <div class="flex border-dashed w-full md:w-2/12 p-1 h-10 md:h-12 border-2 border-orange-500 rounded-full">
            <span class="bg-white/30 w-full h-full animate-pulse rounded-full "></span>
        </div>
    }
}

#[component]
fn ReferCode() -> impl IntoView {
    let canisters = authenticated_canisters();

    view! {
        <Suspense fallback=ReferLoading>
            {move || {
                canisters()
                    .and_then(|canisters| {
                        let canisters = try_or_redirect_opt!(canisters)?;
                        Some(view! { <ReferLoaded user_canister=canisters.user_canister()/> })
                    })
                    .unwrap_or_else(|| {
                        view! { <ReferLoading/> }
                    })
            }}

        </Suspense>
    }
}

#[component]
fn ReferView() -> impl IntoView {
    let (logged_in, _) = account_connected_reader();

    view! {
        <div class="flex flex-col w-full h-full items-center text-white gap-10">
            <img class="shrink-0 h-40 select-none" src="/img/coins-stash.webp"/>
            <div class="flex flex-col w-full items-center gap-4 text-center">
                <span class="font-bold text-2xl">Invite & Win 500 Tokens</span>
                <span class="text-white/50 text-xs">
                    Send a referral link to your friends via link and win tokens!
                </span>
            </div>
            <div class="flex flex-col w-full gap-4 px-4 text-white items-center">
                <span class="uppercase text-sm md:text-md">Referral Code</span>
                <Show when=logged_in fallback=ConnectLogin>
                    <ReferCode/>
                </Show>
            </div>
            <div class="flex flex-col w-full items-center gap-8 mt-4">
                <span class="font-xl">How does it work?</span>
                <div class="flex flex-row gap-8 text-center">
                    <WorkButton text="Share your link with a friend" icon=icondata::TbShare3/>
                    <WorkButton
                        text="Your friends download and log into the app"
                        icon=icondata::TbCloudDownload
                    />
                    <WorkButton
                        text="You both win 500 tokens each"
                        icon=icondata::AiDollarCircleOutlined
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn TabSelector(
    tab_idx: i32,
    text: String,
    tab_str: String,
    current_tab: Memo<i32>,
    set_cur_tab: SignalSetter<Option<String>>,
) -> impl IntoView {
    let button_class = move || {
        if tab_idx == current_tab() {
            "text-white font-bold"
        } else {
            "text-white/50 font-bold"
        }
    };
    let selector_class = move || {
        if tab_idx == current_tab() {
            "bg-orange-500 w-2 h-2 rounded-full"
        } else {
            "bg-transparent w-2 h-2 rounded-full"
        }
    };

    view! {
        <div class="flex w-full flex-col items-center gap-y-2">
            <button class=button_class on:click=move |_| set_cur_tab(Some(tab_str.clone()))>
                {text}
            </button>
            <div class=selector_class></div>
        </div>
    }
}

#[component]
fn ListSwitcher() -> impl IntoView {
    let (cur_tab, set_cur_tab) = create_query_signal::<String>("tab");
    let current_tab = create_memo(move |_| {
        with!(|cur_tab| match cur_tab.as_deref() {
            Some("how-to") => 0,
            Some("history") => 1,
            _ => 0,
        })
    });

    view! {
        <div class="flex flex-row w-full text-md md:text-lg lg:text-xl text-center">
            <TabSelector
                text="How to earn".into()
                tab_idx=0
                tab_str="how-to".to_string()
                current_tab
                set_cur_tab=set_cur_tab
            />
            <TabSelector
                text="History".into()
                tab_idx=1
                tab_str="history".to_string()
                current_tab
                set_cur_tab=set_cur_tab
            />
        </div>
        <Show when=move || current_tab() == 0 fallback=HistoryView>
            <ReferView/>
        </Show>
    }
}

#[component]
pub fn ReferEarn() -> impl IntoView {
    let (logged_in, _) = account_connected_reader();

    view! {
        <div class="flex flex-col items-center min-w-dvw min-h-dvh bg-black pt-2 pb-4 gap-6 px-8">
            <span class="text-lg font-bold text-white">Refer & Earn</span>
            <Show when=logged_in fallback=ReferView>
                <ListSwitcher/>
            </Show>
        </div>
    }
}
