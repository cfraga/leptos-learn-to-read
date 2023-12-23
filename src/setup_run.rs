use leptos::*;
use leptos_meta::*;
use crate::app::{ Difficulty, RunSettings};
use crate::lexicanum;

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