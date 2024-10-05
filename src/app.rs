mod demo_form;

pub use nova_forms::*;

use demo_form::*;
use leptos::*;
use leptos_meta::*;
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
            <DemoForm />
        </FormContainer>
    }
}
