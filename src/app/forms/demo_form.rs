use leptos::*;
use leptos_i18n::*;
use leptos_meta::*;
use nova_forms::*;
use crate::app::use_i18n;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoStaticStr};

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
    zip: String,
    city: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ChildData {
    name: NonEmptyString,
}

// Define an enum for the radio button and the select field.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display, IntoStaticStr, Default)]
pub enum RadioValue {
    #[default]
    A,
    B,
    C,
}

// Defines how to render the form.
#[component]
pub fn DemoForm() -> impl IntoView {
    // Get the locale context.
    let i18n = use_i18n();
    // Define the submit server action.
    let submit_action = create_server_action::<OnSubmit>();

    // Use the zip service to resolve city names.
    let (city, zip) = use_zip_service();
    // Define a signal to show or hide a dialog.
    let show_msg = create_rw_signal(false);
    // Define a signal with two write signals to check the email addresses.
    // This can be easily achieved with the `wire!` macro.
    let (
        error_emails_not_equal,
        email1,
        email2
    ) = wire!(move |email1, email2| {
        (email1 != email2).then(|| t!(i18n, error_emails_not_equal).into())
    });

    view! {
        // Sets the document title.
        <Title text=t!(i18n, demo_form) />

        // Defines how to render the form itself.
        <NovaForm
            on_submit=submit_action
            bind="form_data"
            bind_meta_data="meta_data"
            i18n=i18n
        >

            <Pages>
                <Page id="page-welcome" label=t!(i18n, welcome)>
                    <h2>{t!(i18n, welcome)}</h2>
                    <p>{t!(i18n, welcome_message)}</p>
                    <CsrOnlyWarning />
                </Page>

                <Page id="page-datatypes" label=t!(i18n, datatypes)>
                    <h2>{t!(i18n, datatypes)}</h2>
                    <p>{t!(i18n, field_message)}</p>
                    <p>{t!(i18n, datatypes_message)}</p>
                    <Cols>
                        <Group bind="datatypes">
                            <Input<String> bind="string" label=t!(i18n, string) />
                            <Input<NonEmptyString> bind="non_empty_string" label=t!(i18n, non_empty_string) />
                            <Input<Email> bind="email" label=t!(i18n, email) />
                            <Input<Telephone> bind="phone" label=t!(i18n, telephone) />
                            <Input<u32> bind="number" label=t!(i18n, number) />
                            <Input<Time> bind="time" label=t!(i18n, time) />
                            <Input<Date> bind="date" label=t!(i18n, date) />
                            <Input<DateTime> bind="date_time" label=t!(i18n, date_time) />
                            <Checkbox<bool> bind="bool" label=t!(i18n, boolean) />
                            <Checkbox<Accept> bind="accept" label=t!(i18n, accept) />
                            <Radio<RadioValue> bind="radio" label=t!(i18n, radio) />
                            <Select<RadioValue> bind="select" label=t!(i18n, select) />
                        </Group>
                    </Cols>
                </Page>

                <Page id="page-dynamic-dialog" label=t!(i18n, dynamic_dialog)>
                    <h2>{t!(i18n, dynamic_dialog)}</h2>
                    <p>{t!(i18n, dynamic_dialog_message)}</p>
                    <Checkbox<bool> bind="show_dialog" label=t!(i18n, boolean) change=on_ok(set(show_msg)) />
                    <Dialog
                        kind=DialogKind::Info
                        msg={t!(i18n, lorem_ipsum)}
                        title={t!(i18n, dynamic_dialog)}
                        open=show_msg />
                    <Cols>
                        <Input<Email> bind="email" label=t!(i18n, email) change=on_ok(set(email1)) />
                        <ScreenOnly><Input<Email> bind="repeat_email" label=t!(i18n, email) change=on_ok(set(email2)) error=error_emails_not_equal /></ScreenOnly>
                    </Cols>
                </Page>

                <Page id="page-server-fn" label=t!(i18n, server_fn)>
                    <h2>{t!(i18n, server_fn)}</h2>
                    <p>{t!(i18n, server_fn_message)}</p>
                    <CsrOnlyWarning />
                    <Cols>
                        <Group bind="address">
                            <Colspan><Input<String> bind="street" label=t!(i18n, street) /></Colspan>
                            <Input<String> bind="zip" label=t!(i18n, zip_code) change=on_ok(set(zip)) />
                            <Input<String> bind="city" label=t!(i18n, city) value=city />
                        </Group>
                    </Cols>
                </Page>

                <Page id="page-repeatable" label=t!(i18n, repeatables)>
                    <h2>{t!(i18n, repeatables)}</h2>
                    <p>{t!(i18n, repeatable_information_message)}</p>
                    <Repeatable bind="children" item = move |idx| view! {
                        <h3>{t!(i18n, child, count = move || idx + 1)}</h3>
                        <Input<NonEmptyString> label=t!(i18n, first_name) bind="name" />
                    }>
                    </Repeatable>
                </Page>

                <Page id="page-file-upload" label=t!(i18n, file_upload)>
                    <h2>{t!(i18n, file_upload)}</h2>
                    <p>{t!(i18n, uploading_files_message)}</p>
                    <CsrOnlyWarning />
                    <FileUpload bind="files" />
                </Page>


                <Page id="page-finale" label=t!(i18n, the_grand_finale)>
                    <ScreenOnly>
                        <h2>{t!(i18n, the_grand_finale)}</h2>
                        <p>{t!(i18n, check_form_preview_message)}</p>
                        <p>{t!(i18n, submit_message)}</p>
                        <p>{t!(i18n, pdf_rendering_note)}</p>
                    </ScreenOnly>
                    <PrintOnly>
                        <h2>{t!(i18n, signatures)}</h2>
                    </PrintOnly>
                </Page>
            </Pages>

            <PageStepper />

        </NovaForm>

        <Preview/>

        <Toolbar>
            <ToolbarPageSelect />
            <ToolbarLocaleSelect i18n=i18n />
            <ToolbarPreviewButton />
            <ToolbarSubmitButton />
        </Toolbar>
    }
}

// Show an info dialog if the form is in CSR only mode.
#[component]
pub fn CsrOnlyWarning() -> impl IntoView {
    let i18n = use_i18n();

    if cfg!(feature = "csr") {
        view! {
            <Dialog
                kind=DialogKind::Warn
                msg={t!(i18n, csr_only_messge)}
                title={t!(i18n, csr_only)} />
        }.into_view()
    } else {
        View::default()
    }
}


// Defines the server action for form submission.
#[server]
async fn on_submit(form_data: DemoForm, meta_data: MetaData) -> Result<(), ServerFnError> {
    use crate::app::RenderContextProvider;

    println!("form data received: {:#?}", form_data);
    println!("meta data received: {:#?}", meta_data);

    let pdf_gen = expect_context::<PdfGen>();
    let output_path = pdf_gen
        .render_form(move || view! {
            <RenderContextProvider form_data=form_data meta_data=meta_data>
                <DemoForm />
            </RenderContextProvider>
        })
        .await?;

    println!("form successfully rendered: {:?}", output_path);

    Ok(())
}
