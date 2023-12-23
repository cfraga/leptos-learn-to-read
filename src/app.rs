use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::{use_local_storage, JsonCodec};
use serde::{Serialize, Deserialize};

use crate::setup_run::SetupRun;
use crate::lexicanum;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Difficulty {
    Easiest,
    #[default]
    Easy,
    Medium,
    Hard,
    Hardest,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RunSettings {
    pub num_words: RwSignal<usize>,
    pub word_pool: RwSignal<Vec::<String>>, 
    pub allowed_chars: Signal<String>,
    pub set_allowed_chars: WriteSignal<String>,
    pub all_words: Signal<bool>,
    pub set_all_words: WriteSignal<bool>,
    pub difficulty: Signal<Difficulty>,
    pub set_difficulty: WriteSignal<Difficulty>,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Script async_ = "true" src="https://www.googletagmanager.com/gtag/js?id=G-X6E71G2155"></Script>
        <Script>{r#"
            window.dataLayer = window.dataLayer || [];
            function gtag(){dataLayer.push(arguments);}
            gtag('js', new Date());
          
            gtag('config', 'G-X6E71G2155);
            "#}
        </Script>
        <meta charset="UTF-8" />

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="assets/main.css"/>

        // sets the document title
        <Title text="Vamos Ler!"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

fn select_word(existing_words: RwSignal<Vec<String>>) -> Option<String> {
        existing_words.try_update( |words| {
            logging::log!("attempting a word");
            match words.len() {
                0 => { logging::log!("no more words"); None },
                _ => {
                    logging::log!("got a word");
                    Some(words.swap_remove(0))
                },
            }
        }).unwrap()
    }



/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (allowed_chars, set_allowed_chars, _) = use_local_storage::<String, JsonCodec>("allowed_chars");
    let (all_words, set_all_words, _) = use_local_storage::<bool, JsonCodec>("all_words");
    let (difficulty, set_difficulty, _) = use_local_storage::<Difficulty, JsonCodec>("difficulty");

    // Creates a reactive value to update the button
    let settings = RunSettings {
        num_words: create_rw_signal(10),
        word_pool: create_rw_signal(Vec::<String>::new()),
        allowed_chars: allowed_chars,
        set_allowed_chars: set_allowed_chars,
        all_words: all_words,
        set_all_words: set_all_words,
        difficulty: difficulty,
        set_difficulty: set_difficulty,
    };
    
    let is_reading = create_rw_signal(false);
    let (word, set_word) = create_signal("".to_string());
    let remaining_words = move || settings.word_pool.with(|words| words.len());

    let get_new_word = move || {
        
        match select_word(settings.word_pool) {
            Some(w) => {
                set_word(w)
            },
            None => is_reading.set(false)
        }
    };

    let click_new_word = move |_| {
        get_new_word();
    };


    let start_reading= move |_| {
        is_reading.set(true);
        get_new_word();
    };
        

    view! {
        { move || 
            match is_reading() {
                true => view! { 
                    <div class="active-word"><a target="window" href={move || format!("https://dicionario.priberam.org/{}",word())}>{word}</a></div>
                    <div class="remaining-words"><span>"Faltam "</span><span style="font-weight: bold;">{remaining_words}</span><span>" palavras!"</span></div>
                    <div class="next-word-button" on:click=click_new_word>"Outra Palavra!"</div>
                }.into_view(),
                false => view! {
                    <SetupRun settings=settings.clone() onready=start_reading />
                }.into_view(),
            }
        }
    }
}



/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
