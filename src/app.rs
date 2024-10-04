mod demo_form;

use std::str::FromStr;

pub use nova_forms::*;

use leptos::*;
use leptos_meta::*;
use demo_form::*;
use leptos_router::*;

leptos_i18n::load_locales!();
use i18n::*;

// Define the app that contains the form.
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <I18nContextProvider>
            <Router>
                <Forms/>
            </Router>
        </I18nContextProvider>
    }
}

#[component]
pub fn Forms() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <FormContainer title=t!(i18n, nova_forms) subtitle=t!(i18n, demo_form) logo="/logo.png">
            //<Route path="demo-form" view=move || view! { <DemoForm/> } />
            <DemoForm />
        </FormContainer>
        <Select
            values={Locale::get_all().iter().map(|locale| {
                let display_name = match locale {
                    Locale::en => "English",
                    Locale::de => "Deutsch",
                };
                (*locale, display_name.to_string())
            }).collect::<Vec<(Locale, String)>>()}
            value=move || i18n.get_locale()
            on_change=move |locale| i18n.set_locale(locale) />
    }
}