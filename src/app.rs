use leptos::{*, ev::{click, MouseEvent}};
use leptos_meta::*;
use leptos_router::*;
use regex::Regex;
use rand::seq::SliceRandom;


// pub enum Difficulty {
//     Easy,
//     Medium,
//     Hard,
// }

#[derive(Clone, Debug, PartialEq)]
pub struct RunSettings {
    num_words: RwSignal<usize>,
    allowed_chars: RwSignal<String>,
    all_words: RwSignal<bool>,
    word_pool: RwSignal<Vec::<&'static str>>, 
    // difficulty: RwSignal<Difficulty>,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

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


fn get_word_pool(allowed_chars: Option<String>, num_words: usize) -> Vec<&'static str> {
    let existing_words = vec! [ "pata", "batata", "pena", "Pedro", "Papá", "Tia", "touro", "tempo"];

    match allowed_chars {
        None => if existing_words.len() > num_words { existing_words.into_iter().take(num_words).collect() } else { existing_words },
        Some(chars) => {
            let allowed_regex = Regex::new(format!("^[{}]+$", sanitize_filter(chars)).as_str()).unwrap();
            existing_words
                .into_iter()
                .filter(|word| allowed_regex.is_match(word))
                .collect()
        },
    }
}

fn sanitize_filter(chars: String) -> String {
    chars.chars().filter( |c| c.is_alphabetic()).collect()
}

fn select_word() -> &'static str {
    let existing_words = vec! [ "pata", "batata", "pena", "Pedro", "Papá", "Tia", "touro", "tempo"];

    existing_words.choose(&mut rand::thread_rng()).expect("could not pick string from vec")
}



/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let settings = RunSettings {
        num_words: create_rw_signal(10),
        word_pool: create_rw_signal(Vec::<&str>::new()),
        allowed_chars: create_rw_signal("ap".to_string()),
        all_words: create_rw_signal(false),
    };
    
    let is_reading = create_rw_signal(false);
    let (word, set_word) = create_signal("");
    let (count, set_count) = create_signal(0);


    let get_new_word = move || {
        set_count.update(|count| *count += 1);
        set_word(select_word());
    };

    let click_new_word = move |_| {
        get_new_word();
    };


    let start_reading= move |i:i32| {
        is_reading.set(true);
        get_new_word();
    };
        

    view! {
        { move || 
            match is_reading() {
                true => view! { 
                    <div style="font-size: 20vw;">{word}</div>
                    <div style="width: 100vw; height: 5vh; background-color:aquamarine;" on:click=click_new_word>"Outra Palavra!"</div>
                    <div><span>"Já li "</span><span class="font-weight: bold;">{count}</span><span>" palavras!"</span></div>
                }.into_view(),
                false => view! {
                    <SetupRun settings=settings.clone() onready=start_reading />
                }.into_view(),
            }
        }
    }
}

#[component]
fn setup_run(settings: RunSettings, #[prop(into)] onready: Callback<i32>) -> impl IntoView {

    let start_new_run = move |_| { 
        let filter = match settings.all_words.get() {
            true => None,
            false=> Some(settings.allowed_chars.get()),
        };

        settings.word_pool.set(get_word_pool(filter, settings.num_words.get()));
        onready.call(1);
    };

    view! {
        <h1>"Vamos Ler!"</h1>
        <div>
            <span>letras permitidas: </span>  
            <input type="text" on:input= move |e| { settings.allowed_chars.set(event_target_value(&e))} prop:value=settings.allowed_chars prop:disabled=settings.all_words />
        </div>
        <div>
            <span> usar todas as letras: </span>
            <input type="checkbox" prop:checked=settings.all_words on:input = move |e| { settings.all_words.set(event_target_checked(&e))} />
        </div>
        <div style="width: 100vw; height: 5vh; background-color:aquamarine;" on:click=start_new_run>"Começar!"</div>
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
