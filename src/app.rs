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
pub struct PersonData {
    first_name: NonEmptyString,
    last_name: NonEmptyString,
    age: u32,
}

// Defines how to render the form.
#[component]
pub fn DemoForm(#[prop(optional)] form_data: DemoForm) -> impl IntoView {
    // Get the locale context.
    let i18n = use_i18n();
    // Define the submit server action.
    let submit_action = Action::<OnSubmit, _>::server();

    view! {
        // Injects a stylesheet into the document <head>.
        // id=leptos means cargo-leptos will hot-reload this stylesheet.
        <Stylesheet id="leptos" href="/pkg/nova-forms-demo.css"/>

        // Sets the document title.
        <Title text=t!(i18n, demo_form)/>

        // Defines how to render the form itself.
        <NovaForm form_data=form_data on_submit=submit_action bind="form_data" i18n=i18n>

            <Page id="first-page" label={t!(i18n, about_yourself)}>
                <h1>{t!(i18n, demo_form)}</h1>
                <p>{t!(i18n, welcome_message)}</p>

                // Section about the client.
                <h2>{t!(i18n, about_yourself)}</h2>
                <p>{t!(i18n, about_yourself_message)}</p>
                <Group bind="me">
                    <fieldset class="cols-2">
                        <legend>{t!(i18n, about_yourself)}</legend>
                        <Input<NonEmptyString> label=t!(i18n, first_name) bind="first_name" placeholder="Max" />
                        <Input<NonEmptyString> label=t!(i18n, last_name) bind="last_name" placeholder="Muster" />
                        <Input<u32> label=t!(i18n, age)bind="age"/>
                    </fieldset>
                </Group>

                <Address bind="address"/>
            </Page>

            <Page id="second-page" label={t!(i18n, your_children)}>

                // Repeatable section for children of the client.
                <h2>{t!(i18n, your_children)}</h2>
                <p>"Add the personal information of your children here. You can easily add and remove children with the respective buttons."</p>
                <p>"This demonstrates the ability to dynamically add and remove components, or have repeatable components"</p>
                <Repeatable bind="children" item = |idx| {
                    let i18n = use_i18n();

                    view! {
                        <fieldset class="cols-2">
                            <legend>{format!("Child {}", idx + 1)}</legend>
                            <Input<NonEmptyString> label=t!(i18n, first_name) bind="first_name" placeholder="Max" />
                            <Input<NonEmptyString> label=t!(i18n, last_name) bind="last_name" placeholder="Muster" />
                            <Input<u32> label=t!(i18n, age)bind="age"/>
                        </fieldset>
                    }
                }>
                </Repeatable>

                <div class="no-print">
                    <h2 class="no-print">"Uploading Files"</h2>
                    <p>"Support for file upload can be easily added by inserting the respective component. The server side handling is generated automatically."</p>
                    <p>"Also note that this part of the form won't be rendered in the output PDF. On the other hand, the output PDF can contain sections that are not shown here."</p>
                    <FileUpload bind="files"/>
                </div>

                <div class="no-print">
                    <h2 class="no-print">"The Grand Finale"</h2>
                    <p class="no-print">"Please check the preview of your form by clicking the preview button on the bottom right."</p>
                    <p class="no-print">"After you have confirmed that everything looks alright, you can click submit button send the data to the server and generate the final PDF."</p>
                </div>


                <h2 class="only-print">"Signatures"</h2>
            </Page>

        </NovaForm>
    }
}

// Defines the server action for form submission.
#[server]
async fn on_submit(form_data: DemoForm, meta_data: MetaData) -> Result<(), ServerFnError> {
    use crate::app::NovaFormContextProvider;

    println!("form data received: {:#?}", form_data);
    println!("meta data received: {:#?}", meta_data);

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
