pub use nova_forms::*;

use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};

// Define the form data structure.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct DemoForm {
    me: PersonData,
    #[serde(default)]
    children: Vec<PersonData>,
    #[serde(default)]
    files: Vec<FileId>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct PersonData {
    first_name: NonEmptyString,
    last_name: NonEmptyString,
    age: u32,
}

// Define the app that contains the form.
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <FormContainer title="Nova Forms" subtitle="Demo Form" logo="/logo.png">
            <DemoForm/>
        </FormContainer>
    }
}

// Defines how to render the form.
#[component]
fn DemoForm(#[prop(optional)] form_data: DemoForm) -> impl IntoView {
    // Define the submit server action.
    let submit: Action<OnSubmit, Result<(), ServerFnError>> = Action::<OnSubmit, _>::server();

    view! {
        // Injects a stylesheet into the document <head>.
        // id=leptos means cargo-leptos will hot-reload this stylesheet.
        <Stylesheet id="leptos" href="/pkg/nova-forms-demo.css"/>

        // Sets the document title.
        <Title text="Demo Form"/>

        // Defines how to render the form itself.
        <NovaForm form_data=form_data on_submit=submit bind="form_data">

            <h1>"Demo Form"</h1>
            <p>"Welcome to the Nova Forms demo. In this demo, we show you how easy it is to use Nova Forms compared to other alternatives. Sit back and enjoy!"</p>

            // Section about the client.
            <h2>"Tell Us About Yourself"</h2>
            <p>"Please fill out the form below with your personal information."</p>
            <Group bind="me">
                <fieldset class="cols-2">
                    <legend>{format!("About Yourself")}</legend>
                    <Input label="First Name" bind="first_name" placeholder=NonEmptyString::from("Max") />
                    <Input label="Last Name" bind="last_name" placeholder=NonEmptyString::from("Muster") />
                    <Input<u32> label="Age" bind="age"/>
                </fieldset>
            </Group>

            // Repeatable section for children of the client.
            <h2>"Your Children"</h2>
            <p>"Add the personal information of your children here. You can easily add and remove children with the respective buttons."</p>
            <p>"This demonstrates the ability to dynamically add and remove components, or have repeatable components"</p>
            <Repeatable bind="children" item = |idx| {
                view! {
                    <fieldset class="cols-2">
                        <legend>{format!("Child {}", idx + 1)}</legend>
                        <Input label="First Name" bind="first_name" placeholder=NonEmptyString::from("Max") />
                        <Input label="Last Name" bind="last_name" placeholder=NonEmptyString::from("Muster") />
                        <Input<u32> label="Age" bind="age"/>
                    </fieldset>
                }
            }>
            </Repeatable>

            <div class="no-print">
                <h2 class="no-print">"Uploading Files"</h2>
                <p>"Support for file upload can be easily added by inserting the respective component. The server side handling is generated automatically."</p>
                <p>"Also note that this part of the worm won't be rendered in the output PDF. On the other hand, the output PDF can contain sections that are not shown here."</p>
                <FileUpload bind="files" label="Select File"/>
            </div>

            <div class="no-print">
                <h2 class="no-print">"The Grand Finale"</h2>
                <p class="no-print">"Please click the submit button below to generate a PDF of the form you just filled out."</p>
                <input type="submit" value="Submit"/>
            </div>

            /*
            <Modal kind=ModalKind::Success title="Form Submission Successful">
                "The form submission was sucessful"
            </Modal>
            */


            <h2 class="only-print">"Signatures"</h2>
        </NovaForm>
    }
}

// Defines the server action for form submission.
#[server]
async fn on_submit(form_data: DemoForm) -> Result<(), ServerFnError> {
    println!("form data received: {:?}", form_data);

    let pdf_gen = expect_context::<PdfGen>();
    let output_path = pdf_gen
        .render_form(move || view! { <DemoForm form_data=form_data/> })
        .await?;

    println!("form successfully rendered: {:?}", output_path);

    Ok(())
}