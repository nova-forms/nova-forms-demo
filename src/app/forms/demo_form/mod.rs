use leptos::*;
use leptos_i18n::*;
use leptos_meta::*;
use nova_forms::*;
use crate::app::use_i18n;

mod data;
pub use data::*;

// Defines how to render the form.
#[component]
pub fn DemoForm() -> impl IntoView {
    // Get the locale context.
    let i18n = use_i18n();
    // Define the submit server action.
    let submit_action = create_server_action::<OnSubmit>();

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
                    <p>{t!(i18n, welcome_message)}</p>
                </Page>

                <Page id="page-datatypes" label=t!(i18n, datatypes)>
                    <p>{t!(i18n, field_message)}</p>
                    <p>{t!(i18n, datatypes_message)}</p>
                    <Cols>
                        <Group bind="datatypes">
                            <Input<String> bind="string" label=t!(i18n, string) />
                            <Input<NonEmptyString> bind="non_empty_string" label=t!(i18n, non_empty_string) />
                            <Input<Email> bind="email" label=t!(i18n, email) />
                            <Input<Phone> bind="phone" label=t!(i18n, telephone) />
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
                    <p>{t!(i18n, dynamic_dialog_message)}</p>
                    <Checkbox<bool> bind="show_dialog" label=t!(i18n, boolean) />
                    <Dialog
                        kind=DialogKind::Info
                        msg={t!(i18n, lorem_ipsum)}
                        title={t!(i18n, dynamic_dialog)}
                        open={Signal::derive(move || value!(..[show_dialog] as bool).get().unwrap_or(false) ) } />
                    <Cols>
                        <Input<Email> bind="email" label=t!(i18n, email) />
                        <ScreenOnly><Input<Email> bind="repeat_email" label=t!(i18n, email) error={Signal::derive(move || {
                            (value!(..[email] as Email).get() != value!(..[repeat_email] as Email).get())
                                .then(|| t!(i18n, error_emails_not_equal).into())
                        })} /></ScreenOnly>
                    </Cols>
                </Page>

                <Page id="page-server-fn" label=t!(i18n, server_fn)>
                    <p>{t!(i18n, server_fn_message)}</p>
                    <Cols>
                        <Group bind="address">
                            <Colspan><Input<String> bind="street" label=t!(i18n, street) /></Colspan>
                            <Input<PostalCodeCH> bind="zip" label=t!(i18n, zip_code) />
                            <Input<NonEmptyString> bind="city" label=t!(i18n, city) value=postal_code_service(value!(..[zip] as PostalCodeCH)) />
                        </Group>
                    </Cols>
                </Page>

                <Page id="page-repeatable" label=t!(i18n, repeatables)>
                    <p>{t!(i18n, repeatable_information_message)}</p>
                    <Repeatable bind="children" item = move |idx| view! {
                        <h3>{t!(i18n, child, count = move || idx + 1)}</h3>
                        <Input<NonEmptyString> label=t!(i18n, first_name) bind="name" />
                    }>
                    </Repeatable>
                </Page>

                <Page id="page-file-upload" label=t!(i18n, file_upload)>
                    <p>{t!(i18n, uploading_files_message)}</p>
                    <FileUpload bind="files" />
                </Page>


                <Page id="page-finale" label=t!(i18n, the_grand_finale)>
                    <ScreenOnly>
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
            <ToolbarLocaleSelect i18n=i18n />
            <ToolbarPreviewButton />
            <ToolbarSubmitButton />
        </Toolbar>
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
