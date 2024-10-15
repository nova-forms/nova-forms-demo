use leptos::*;
use leptos_meta::*;
use nova_forms::*;
use leptos_i18n::*;
use serde::{Deserialize, Serialize};

// This generates the `NovaFormContextProvider` component at compile-time to initialize all the necessary context.
init_nova_forms!();

// Define the app that contains the form.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <NovaFormContextProvider>
            <DemoForm/>
        </NovaFormContextProvider>
    }
}

// Define the form data structure.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DemoForm {
    me: PersonData,
    address: Address,
    #[serde(default)]
    children: Vec<PersonData>,
    #[serde(default)]
    files: Vec<FileId>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Address {
    street: String,
    house_number: String,
    zip: String,
    city: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PersonData {
    first_name: NonEmptyString,
    last_name: NonEmptyString,
}

// Defines how to render the form.
#[component]
pub fn DemoForm(#[prop(optional)] form_data: DemoForm) -> impl IntoView {
    // Get the locale context.
    let i18n = use_i18n();
    // Define the submit server action.
    let submit_action = create_server_action::<OnSubmit>();
    // Use the zip service to resolve city names.
    let (set_zip, city) = use_zip_service();

    // Define custom error message translations.
    provide_translation_context(move |err| match err {
        NonEmptyStringError::EmptyString => t!(i18n, error_empty_string),
    });

    view! {
        // Injects a stylesheet into the document <head>.
        // id=leptos means cargo-leptos will hot-reload this stylesheet.
        <Stylesheet id="leptos" href="/pkg/nova-forms-demo.css"/>

        // Sets the document title.
        <Title text=t!(i18n, demo_form)/>

        // Defines how to render the form itself.
        <NovaForm form_data=form_data on_submit=submit_action bind="form_data" bind_meta_data="meta_data" i18n=i18n>

            <Page id="first-page" label=t!(i18n, about_yourself)>
                <h1>{t!(i18n, demo_form)}</h1>
                <p>{t!(i18n, welcome_message)}</p>
                <h2>{t!(i18n, about_yourself)}</h2>
                <p>{t!(i18n, about_yourself_message)}</p>
                <fieldset class="cols-2">
                    <legend>{t!(i18n, about_yourself)}</legend>
                    <Group bind="me">
                        <Input<NonEmptyString> label=t!(i18n, first_name) bind="first_name" />
                        <Input<NonEmptyString> label=t!(i18n, last_name) bind="last_name" />
                    </Group>
                    <Group bind="address">
                        <Input<String> bind="street" label=t!(i18n, street) />
                        <Input<String> bind="house_number" label=t!(i18n, house_number) />
                        <Input<String> bind="zip" label=t!(i18n, zip_code) on:input=set_zip />
                        <Input<String> bind="city" label=t!(i18n, city) value=city />
                    </Group>
                </fieldset>
            </Page>

            <Page id="second-page" label={t!(i18n, your_children)}>
                <h2>{t!(i18n, your_children)}</h2>
                <p>{t!(i18n, children_information_message)}</p>
                <p>{t!(i18n, repeatable_information_message)}</p>
                <Repeatable bind="children" item = move |idx| view! {
                    <fieldset class="cols-2">
                        <legend>{t!(i18n, child, num = move || idx + 1)}</legend>
                        <Input<NonEmptyString> label=t!(i18n, first_name) bind="first_name" />
                        <Input<NonEmptyString> label=t!(i18n, last_name) bind="last_name" />
                    </fieldset>
                }>
                </Repeatable>

                <div class="no-print">
                    <h2>{t!(i18n, uploading_files)}</h2>
                    <p>{t!(i18n, uploading_files_message)}</p>
                    <p>{t!(i18n, pdf_rendering_note)}</p>
                    <FileUpload bind="files"/>
                    <h2>{t!(i18n, the_grand_finale)}</h2>
                    <p>{t!(i18n, check_form_preview_message)}</p>
                    <p>{t!(i18n, submit_message)}</p>
                </div>

                <h2 class="only-print">{t!(i18n, signatures)}</h2>
            </Page>

        </NovaForm>
    }
}

// Defines the server action for form submission.
#[server]
async fn on_submit(form_data: DemoForm, meta_data: MetaData) -> Result<(), ServerFnError> {
    use crate::app::NovaFormContextProvider;

    println!("form data received: {:#?}", form_data);

    let pdf_gen = expect_context::<PdfGen>();
    let output_path = pdf_gen
        .render_form(move || view! {
            <NovaFormContextProvider meta_data=meta_data>
                <DemoForm form_data=form_data/>
            </NovaFormContextProvider>
        })
        .await?;

    println!("form successfully rendered: {:?}", output_path);

    Ok(())
}
