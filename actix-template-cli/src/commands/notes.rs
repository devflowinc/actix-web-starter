use actix_web_starter_client::{
    apis::{
        self,
        notes_api::{
            CreateNoteParams, CreateNoteSuccess, GetNotesForOrgError, GetNotesForOrgParams,
            GetNotesForOrgSuccess,
        },
    },
    models::{CreateNoteReqPayload, Note},
};

use super::configure::ActixTemplateConfiguration;

async fn get_org_notes(
    config: ActixTemplateConfiguration,
) -> Result<Vec<Note>, actix_web_starter_client::apis::Error<GetNotesForOrgError>> {
    println!("Fetching notes for organization... {:?}", config);
    let notes = actix_web_starter_client::apis::notes_api::get_notes_for_org(
        &config.clone().into(),
        GetNotesForOrgParams {
            organization: config.org_id,
            limit: Some(40),
            offset: Some(0),
        },
    )
    .await?
    .entity
    .unwrap();

    match notes {
        GetNotesForOrgSuccess::Status200(notes) => Ok(notes),
        GetNotesForOrgSuccess::UnknownValue(val) => Err(apis::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Unknown value: {:?}", val),
        ))),
    }
}

pub async fn list_notes(config: ActixTemplateConfiguration) {
    let notes = get_org_notes(config.clone())
        .await
        .map_err(|e| {
            eprintln!("Error fetching notes: {:?}", e);
            std::process::exit(1);
        })
        .unwrap();

    if notes.is_empty() {
        println!("No notes found.");
        return;
    }

    println!("\nNotes:");
    for note in notes {
        println!("  - {}", note.title);
    }
}

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
