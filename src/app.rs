mod translations;
mod forms;

use translations::*;
use forms::*;

use leptos::*;
use nova_forms::*;

// This generates the `BaseContextProvider` as well as the `RenderContextProvider` component at compile-time to initialize all the necessary context.
// Note that the csr feature is only necessary for the client-side demo, in a real application you would only use the normal mode.
#[cfg(feature = "csr")]
init_nova_forms!("/nova-forms-demo");
#[cfg(not(feature = "csr"))]
init_nova_forms!();

// Define the app that contains the form.
#[component]
pub fn App() -> impl IntoView {
    view! {
        // We differentiate the base URL between CSR only mode and normal mode for this form.
        // This is only necessary for the demo, in a real application you would only use one mode.
        <AppContextProvider>
            <FormWrapper />
        </AppContextProvider>
    }
}

// Define the app that contains the form.
#[component]
pub fn FormWrapper() -> impl IntoView {
    let i18n = use_i18n();

    provide_translations();
    
    view! {
        <NovaFormWrapper title=t!(i18n, nova_forms) subtitle=t!(i18n, demo_form) logo="logo.svg">
            <DemoForm />
        </NovaFormWrapper>  
    }
}