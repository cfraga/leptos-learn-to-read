use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use regex::Regex;
use rand::seq::SliceRandom;

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


// fn words(allowed_chars: &str) -> Vec<&str> {
//     let existing_words = vec! [ "pata", "batata", "pena", "Pedro", "Papá", "Tia", "touro", "tempo"];
//     let allowed_regex = Regex::new(format!("^[{}]+$", allowed_chars).as_str()).unwrap();

//     existing_words
//         .iter()
//         .filter( |word| allowed_regex.is_match(word))
//         .collect()
// }

fn select_word() -> &'static str {
    let existing_words = vec! [ "pata", "batata", "pena", "Pedro", "Papá", "Tia", "touro", "tempo"];

    existing_words.choose(&mut rand::thread_rng()).expect("could not pick string from vec")
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let (word, set_word) = create_signal("");

    let on_click = move |_| {
        set_count.update(|count| *count += 1);
        set_word(select_word());
    };
        

    view! {
        <h1>"Vamos Ler!"</h1>
        <br/>
        <div style="font-size: 20vw;">{word}</div>
        <button on:click=on_click>"Outra Palavra!"</button>
        <div><span>"Já li "</span><span class="font-weight: bold;">{count}</span><span>" palavras!"</span></div>
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
