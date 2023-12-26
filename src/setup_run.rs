use leptos::*;
use leptos_meta::*;
use leptos_use::storage::{use_local_storage, JsonCodec};

use crate::app::{ Difficulty, RunSettings};
use crate::lexicanum;

#[derive(Clone, Debug, PartialEq)]
struct ToggleableKey {
    id: String,
    label: String,
    value: String,
    is_active: Signal<bool>,
    set_active: WriteSignal<bool>,
}

#[component]
pub fn setup_run(settings: RunSettings, #[prop(into)] onready: Callback<i32>) -> impl IntoView {
    let get_server_words = create_action(
        move |options: &(Option<String>, usize, Difficulty)| {
            let cloned_options = options.clone();
            logging::log!("calling server for words");
            async move { lexicanum::get_word_pool(cloned_options.0, cloned_options.1, cloned_options.2).await }
        }
    );

    let start_new_run = move |_| { 
        let filter = match settings.all_words.get() {
            true => None,
            false=> Some(settings.allowed_chars.get()),
        };
        get_server_words.dispatch((filter, settings.num_words.get(), settings.difficulty.get()));
    };
    
    create_effect(move |_| {
        if let Some(Ok(word_pool)) = get_server_words.value().get() {
            logging::log!("words file was loaded. {} words retrieved", word_pool.len());
            settings.word_pool.set(word_pool);
            onready.call(1);
        }
    });

    view! {
        <h1 class="settings-title"> "Vamos Ler!"</h1>
        <div class="settings-section">
            <div>
                <span> usar todas as letras: </span>
                <input type="checkbox" prop:checked=settings.all_words on:input = move |e| { settings.set_all_words.set(event_target_checked(&e))} />
            </div>
            <div>
                <span>letras permitidas: </span>  
                <ToggleKeyboard set_all_values = settings.set_allowed_chars > </ToggleKeyboard>
                <input type="text" on:input= move |e| { settings.set_allowed_chars.set(event_target_value(&e))} prop:value=settings.allowed_chars prop:disabled=settings.all_words />
            </div>
            <div class="settings-difficulty">
                <div class="difficulty-title"> "Dificuldade"</div>
                <div>
                    <input type="radio" prop:checked=move || settings.difficulty.with( |diff| *diff == Difficulty::Easiest) on:input = move |_e| {settings.set_difficulty.set(Difficulty::Easiest)} />
                    <span>"🌶  "</span>
                </div>
                <div>
                    <input type="radio" prop:checked=move || settings.difficulty.with( |diff| *diff == Difficulty::Easy) on:input = move |_e| {settings.set_difficulty.set(Difficulty::Easy)}/>
                    <span>"🌶🌶  "</span>
                </div>
                <div>
                    <input type="radio" prop:checked=move || settings.difficulty.with( |diff| *diff == Difficulty::Medium) on:input = move |_e| {settings.set_difficulty.set(Difficulty::Medium)} />
                    <span>"🌶🌶🌶  "</span>
                </div>
                <div>
                    <input type="radio" prop:checked=move || settings.difficulty.with( |diff| *diff == Difficulty::Hard) on:input = move |_e| {settings.set_difficulty.set(Difficulty::Hard)}/>
                    <span>"🌶🌶🌶🌶  "</span>
                </div>
                <div>
                    <input type="radio" prop:checked=move || settings.difficulty.with( |diff| *diff == Difficulty::Hardest) on:input = move |_e| {settings.set_difficulty.set(Difficulty::Hardest)} />
                    <span>"🌶🌶🌶🌶🌶  "</span>
                </div>
            </div>
        </div>
        <div class="start-button" on:click=start_new_run>"Começar!"</div>
    }
}

#[component]
pub fn toggle_keyboard( set_all_values: WriteSignal<String> ) -> impl IntoView {
    let ids_labels_vals = vec![
        ("tk_A", "A", "aAàÀáÁâÂãÃ"),
        ("tk_E", "E", "eEèÈéÉêÊ"),
        ("tk_I", "I", "iIíÍ"),
        ("tk_O", "O", "oOôÔòÒóÓõÕ"),
        ("tk_U", "U", "uUùÙúÚ"),
        ("tk_B", "B", "bB"),
        ("tk_C", "C", "cC"),
        ("tk_D", "D", "dD"),
        ("tk_F", "F", "fF"),
        ("tk_G", "G", "gG"),
        ("tk_H", "H", "hH"),
        ("tk_J", "J", "jJ"),
        ("tk_K", "K", "kK"),
        ("tk_L", "L", "lL"),
        ("tk_M", "M", "mM"),
        ("tk_N", "N", "nN"),
        ("tk_P", "P", "pP"),
        ("tk_Q", "Q", "qQ"),
        ("tk_R", "R", "rR"),
        ("tk_S", "S", "sS"),
        ("tk_T", "T", "tT"),
        ("tk_V", "V", "vV"),
        ("tk_W", "W", "wW"),
        ("tk_X", "X", "xX"),
        ("tk_Y", "Y", "yY"),
        ("tk_Z", "Z", "zZ"),
    ];

    let (keys, set_keys) = create_signal(
        ids_labels_vals
        .iter()
        .map(|(id,l,v)| { 
            let (rs, ws, _) = use_local_storage::<bool, JsonCodec>(id); 
            ToggleableKey { id: id.to_string(), label: l.to_string(), value: v.to_string(), is_active: rs, set_active: ws }
        })
        .collect::<Vec<_>>());

    let derived_all_values = create_effect( move |_| {
        keys.with( |vals| 
            set_all_values.set(
                vals.iter()
                    .filter(|k| k.is_active.get())
                    .map(|k| k.value.clone() )
                    .collect::<Vec<String>>()
                    .join("")
            )
        )
    });

    view! {
        <div class="keyboard">
            <For 
                each=keys
                key=|key| key.id.clone()
                children= move |key| {
                    view! {
                        <ToggleKey key=key></ToggleKey>
                    }
                }
            />
        </div>
    }
}

#[component]
fn toggle_key(key: ToggleableKey) -> impl IntoView {
    view!{
        <div class="key" class:active=key.is_active on:click= move |_| { key.set_active.set(!key.is_active.get()) }  >
            {key.label}
        </div>
    }
}