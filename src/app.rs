use leptos::*;
use leptos_i18n::*;
use leptos_meta::*;
use nova_forms::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

// This generates the `NovaFormContextProvider` component at compile-time to initialize all the necessary context.
init_nova_forms!();

// Define the app that contains the form.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <NovaFormsContextProvider 
            base_url=if cfg!(feature = "csr") { "/nova-forms-demo" } else { "/" }
        >
        {
            let i18n = use_i18n();
            
            view! {
                <NovaFormWrapper title=t!(i18n, nova_forms) subtitle=t!(i18n, demo_form) logo="logo.svg">
                    <DemoForm />
                </NovaFormWrapper>  
            }
        }
        </NovaFormsContextProvider>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display, IntoStaticStr, Default)]
pub enum RadioValue {
    #[default]
    A,
    B,
    C,
}

// Define the form data structure.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DemoForm {
    datatypes: Datatypes,
    address: Address,
    #[serde(default)]
    children: Vec<ChildData>,
    #[serde(default)]
    files: Vec<FileId>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Datatypes {
    string: String,
    non_empty_string: NonEmptyString,
    email: Email,
    phone: Telephone,
    number: u64,
    date: Date,
    time: Time,
    date_time: DateTime,
    #[serde(default)]
    bool: bool,
    accept: Accept,
    radio: RadioValue,
    select: RadioValue,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Address {
    street: String,
    house_number: String,
    zip: String,
    city: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ChildData {
    first_name: NonEmptyString,
    last_name: NonEmptyString,
}

// Defines how to render the form.
#[component]
pub fn DemoForm(#[prop(optional)] form_data: DemoForm, #[prop(optional)] render: bool) -> impl IntoView {
    // Get the locale context.
    let i18n = use_i18n();
    // Define the submit server action.
    let submit_action = create_server_action::<OnSubmit>();
    // Use the zip service to resolve city names.
    let (set_zip, city) = use_zip_service();

    // Define custom error message translations.
    provide_translation(move |err| match err {
        NonEmptyStringError::EmptyString => t!(i18n, error_empty_string).into(),
    });

    provide_translation(move |err| match err {
        RadioValue::A => t!(i18n, radio_a).into(),
        RadioValue::B => t!(i18n, radio_b).into(),
        RadioValue::C => t!(i18n, radio_c).into(),
    });

    provide_translation(move |err| match err {
        SubmitState::Initial => "".into(),
        SubmitState::Error(_) => t!(i18n, submit_error).into(),
        SubmitState::Pending => t!(i18n, submit_pending).into(),
        SubmitState::Success => t!(i18n, submit_success).into(),
    });

    provide_translation(move |toolbar| match toolbar {
        Translation::Edit => t!(i18n, edit).into(),
        Translation::Submit => t!(i18n, submit).into(),
        Translation::Preview => t!(i18n, preview).into(),
        Translation::Language => t!(i18n, language).into(),
        Translation::Menu => t!(i18n, menu).into(),
    });

    view! {
        // Sets the document title.
        <Title text=t!(i18n, demo_form) />

        // Defines how to render the form itself.
        <NovaForm
            form_data=form_data
            on_submit=submit_action
            bind="form_data"
            bind_meta_data="meta_data"
            i18n=i18n
            render=render
        >

            <Pages>
                <Page id="page-welcome" label=t!(i18n, welcome)>
                    <h2>{t!(i18n, welcome)}</h2>
                    <p>{t!(i18n, welcome_message)}</p>
                    {
                        if cfg!(feature = "csr") {
                            view! {
                                <p>{t!(i18n, csr_only_messge)}</p>
                            }.into_view()
                        } else {
                            View::default()
                        }
                    }
                </Page>

                <Page id="page-datatypes" label=t!(i18n, datatypes)>
                    <h2>{t!(i18n, datatypes)}</h2>
                    <p>{t!(i18n, datatypes_message)}</p>
                    <div class="cols-2">
                        <Group bind="datatypes">
                            <Input<String> bind="string" label=t!(i18n, string) />
                            <Input<NonEmptyString> bind="non_empty_string" label=t!(i18n, non_empty_string) />
                            <Input<Email> bind="email" label=t!(i18n, email) />
                            <Input<Telephone> bind="phone" label=t!(i18n, telephone) />
                            <Input<u64> bind="number" label=t!(i18n, number) />
                            <Input<Time> bind="time" label=t!(i18n, time) />
                            <Input<Date> bind="date" label=t!(i18n, date) />
                            <Input<DateTime> bind="date_time" label=t!(i18n, date_time) />
                            <Checkbox<bool> bind="bool" label=t!(i18n, boolean) />
                            <Checkbox<Accept> bind="accept" label=t!(i18n, accept) />
                            <Radio<RadioValue> bind="radio" label=t!(i18n, radio) />
                            <Select<RadioValue> bind="select" label=t!(i18n, select) />
                        </Group>
                    </div>
                </Page>

                <Page id="page-server-fn" label=t!(i18n, server_fn)>
                    <h2>{t!(i18n, server_fn)}</h2>
                    <p>{t!(i18n, server_fn_message)}</p>
                    <div class="cols-2">
                        <Group bind="address">
                            <Input<String> bind="street" label=t!(i18n, street) />
                            <Input<String> bind="house_number" label=t!(i18n, house_number) />
                            <Input<String> bind="zip" label=t!(i18n, zip_code) on:input=set_zip />
                            <Input<String> bind="city" label=t!(i18n, city) value=city />
                        </Group>
                    </div>
                </Page>

                <Page id="page-repeatable" label=t!(i18n, repeatables)>
                    <h2>{t!(i18n, repeatables)}</h2>
                    <p>{t!(i18n, repeatable_information_message)}</p>
                    <Repeatable bind="children" item = move |idx| view! {
                        <fieldset class="cols-2">
                            <legend>{t!(i18n, child, num = move || idx + 1)}</legend>
                            <Input<NonEmptyString> label=t!(i18n, first_name) bind="first_name" />
                            <Input<NonEmptyString> label=t!(i18n, last_name) bind="last_name" />
                        </fieldset>
                    }>
                    </Repeatable>
                </Page>

                <Page id="page-file-upload" label=t!(i18n, file_upload)>
                    <h2>{t!(i18n, file_upload)}</h2>
                    <p>{t!(i18n, uploading_files_message)}</p>
                    <FileUpload bind="files" />
                </Page>


                <Page id="page-finale" label=t!(i18n, the_grand_finale)>
                    <div class="no-print">
                        <h2>{t!(i18n, the_grand_finale)}</h2>
                        <p>{t!(i18n, check_form_preview_message)}</p>
                        <p>{t!(i18n, submit_message)}</p>
                        <p>{t!(i18n, pdf_rendering_note)}</p>
                    </div>
                    <h2 class="only-print">{t!(i18n, signatures)}</h2>
                </Page>
            </Pages>

            <PageStepper />

        </NovaForm>
    }
}

// Defines the server action for form submission.
#[server]
async fn on_submit(form_data: DemoForm, meta_data: MetaData) -> Result<(), ServerFnError> {
    use crate::app::NovaFormsContextProvider;

    println!("form data received: {:#?}", form_data);
    println!("meta data received: {:#?}", meta_data);

    let pdf_gen = expect_context::<PdfGen>();
    let output_path = pdf_gen
        .render_form(move || {
            view! {
                <NovaFormsContextProvider meta_data=meta_data>
                    <DemoForm form_data=form_data render=true />
                </NovaFormsContextProvider>
            }
        })
        .await?;

    println!("form successfully rendered: {:?}", output_path);

    Ok(())
}
