use actix_web_starter_client::{
    apis::{
        configuration::Configuration,
        notes_api::{CreateNoteParams, CreateNoteSuccess},
    },
    models::CreateNoteReqPayload,
};

use super::configure::ActixTemplateConfiguration;

pub async fn create_note(config: ActixTemplateConfiguration, title: Option<String>) {
    let title = if title.is_none() {
        inquire::Text::new("Enter a title for the note:")
            .prompt()
            .expect("Prompt configured correctly")
    } else {
        title.unwrap()
    };

    let payload = CreateNoteReqPayload { title };

    let created_note = actix_web_starter_client::apis::notes_api::create_note(
        &config.clone().into(),
        CreateNoteParams {
            organization: config.clone().org_id.clone(),
            create_note_req_payload: payload,
        },
    )
    .await
    .map_err(|e| {
        eprintln!("Error creating note: {:?}", e);
        std::process::exit(1);
    })
    .unwrap()
    .entity
    .unwrap();

    let CreateNoteSuccess::Status201(note) = created_note else {
        eprintln!("Error creating note.");
        std::process::exit(1);
    };

    println!("\nNote created successfully!\n");
}
