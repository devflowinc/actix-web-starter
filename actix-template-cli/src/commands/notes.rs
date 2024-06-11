use super::configure::ActixTemplateConfiguration;
use crate::errors::DefaultError;
use actix_web_starter_client::{
    apis::notes_api::{
        self, CreateNoteParams, CreateNoteSuccess, GetNoteByIdParams, GetNoteByIdSuccess,
        GetNotesForOrgParams, GetNotesForOrgSuccess, UpdateNoteParams, UpdateNoteSuccess,
    },
    models::{CreateNoteReqPayload, Note, UpdateNoteReqPayload},
};
use clap::{Args, Subcommand};
use inquire::Editor;
use std::fmt::{self, Display};

#[derive(Subcommand)]
pub enum NoteCommands {
    Create(CreateNote),
    Delete(DeleteNote),
    Edit(EditNote),
    List,
    View(ViewNote),
}

#[derive(Args)]
pub struct CreateNote {
    /// The title of the note you want to create
    pub title: Option<String>,
}

#[derive(Args)]
pub struct DeleteNote {
    /// The id of the note you want to delete
    pub id: Option<String>,
}

#[derive(Args)]
pub struct EditNote {
    /// The id of the note you want to edit
    pub id: Option<String>,
}

#[derive(Args)]
pub struct ViewNote {
    /// The title of the note you want to view
    pub id: Option<String>,
}

async fn get_org_notes(config: ActixTemplateConfiguration) -> Result<Vec<Note>, DefaultError> {
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
        GetNotesForOrgSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Could not parse response body getting notes for orgs",
        )),
    }
}

async fn set_note_body(
    configuration: ActixTemplateConfiguration,
    note_id: String,
    new_body: String,
) -> Result<Note, DefaultError> {
    let params = UpdateNoteParams {
        organization: configuration.clone().org_id,
        note_id,
        update_note_req_payload: UpdateNoteReqPayload { body: new_body },
    };
    let UpdateNoteSuccess::Status200(new_note) =
        actix_web_starter_client::apis::notes_api::update_note(&configuration.into(), params)
            .await?
            .entity
            .unwrap()
    else {
        return Err(DefaultError::new("Could not update note"));
    };
    Ok(new_note)
}

async fn edit_note(
    config: ActixTemplateConfiguration,
    note: Note,
    message: &str,
) -> Result<Note, DefaultError> {
    let result = Editor::new(message)
        .with_predefined_text(&note.body)
        .prompt()?;

    let new_note = set_note_body(config, note.id, result).await?;
    Ok(new_note)
}

async fn get_note_by_id(
    config: ActixTemplateConfiguration,
    note_id: String,
) -> Result<Note, DefaultError> {
    let note = notes_api::get_note_by_id(
        &config.clone().into(),
        GetNoteByIdParams {
            organization: config.org_id,
            note_id,
        },
    )
    .await?
    .entity
    .ok_or(DefaultError::new("Could not get note from id"))?;

    match note {
        GetNoteByIdSuccess::Status200(note) => Ok(note),
        GetNoteByIdSuccess::UnknownValue(_) => Err(DefaultError::new(
            "Could not parse response body getting note by id",
        )),
    }
}

pub async fn edit_note_cmd(
    config: ActixTemplateConfiguration,
    note_id: Option<String>,
) -> Result<(), DefaultError> {
    let note = if note_id.is_none() {
        select_note(config.clone(), "Select a note to edit:").await?
    } else {
        get_note_by_id(config.clone(), note_id.unwrap()).await?
    };

    let _ = edit_note(
        config.clone(),
        note.clone(),
        &format!("Editing note: {}", note.title),
    )
    .await?;
    Ok(())
}

pub async fn view_note_cmd(
    config: ActixTemplateConfiguration,
    note_id: Option<String>,
) -> Result<(), DefaultError> {
    let note = if note_id.is_none() {
        select_note(config.clone(), "Select a note to view:").await?
    } else {
        get_note_by_id(config.clone(), note_id.unwrap()).await?
    };

    println!("\nNote: {}\n{}\n", note.title, note.id);
    println!("{}", note.body);
    Ok(())
}

pub async fn list_notes_cmd(config: ActixTemplateConfiguration) -> Result<(), DefaultError> {
    let notes = get_org_notes(config.clone()).await?;
    if notes.is_empty() {
        println!("No notes found.");
        return Ok(());
    }

    println!("\nNotes:");
    for note in notes {
        println!("  - {}", note.title);
    }
    Ok(())
}

struct NoteSelectOption {
    note: Note,
}

impl From<Note> for NoteSelectOption {
    fn from(note: Note) -> Self {
        NoteSelectOption { note }
    }
}

impl Display for NoteSelectOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.note.title)
    }
}

async fn select_note(
    config: ActixTemplateConfiguration,
    prompt: &str,
) -> Result<Note, DefaultError> {
    let notes = get_org_notes(config.clone())
        .await?
        .iter()
        .map(|n| n.clone().into())
        .collect::<Vec<NoteSelectOption>>();
    let note = inquire::Select::new(prompt, notes)
        .prompt()
        .expect("Prompt configured correctly");
    Ok(note.note)
}

pub async fn delete_note_cmd(
    config: ActixTemplateConfiguration,
    note_id: Option<String>,
) -> Result<(), DefaultError> {
    let note = if note_id.is_none() {
        select_note(config.clone(), "Select a note to delete:").await?
    } else {
        get_note_by_id(config.clone(), note_id.unwrap()).await?
    };

    match actix_web_starter_client::apis::notes_api::delete_note(
        &config.clone().into(),
        notes_api::DeleteNoteParams {
            organization: config.clone().org_id,
            note_id: note.id,
        },
    )
    .await
    .map_err(|e| DefaultError::new(format!("Error deleting note: {:?}", e).as_str()))?
    .status
    .is_success()
    {
        true => {
            println!("Note deleted successfully.");
            Ok(())
        }
        false => {
            eprintln!("Error deleting note.");
            Err(DefaultError::new("Error deleting note"))
        }
    }
}

pub async fn create_note_cmd(
    config: ActixTemplateConfiguration,
    title: Option<String>,
) -> Result<(), DefaultError> {
    let title = if title.is_none() {
        inquire::Text::new("Enter a title for the note:")
            .prompt()
            .map_err(|e| DefaultError::new(format!("Error getting title: {:?}", e).as_str()))?
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
    .await?
    .entity;
    let note = match created_note {
        None => Err(DefaultError::new("Unknown error creating note.")),
        Some(CreateNoteSuccess::UnknownValue(_)) => {
            Err(DefaultError::new("Unknown error creating note."))
        }
        Some(CreateNoteSuccess::Status201(note)) => Ok(note),
    }?;

    let note = edit_note(config, note, "Edit note").await?;

    println!("\nNote created successfully with id: {}\n", note.id);
    Ok(())
}
