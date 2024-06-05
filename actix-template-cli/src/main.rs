use actix_web_starter_client::apis::configuration::Configuration;
use clap::{Args, Parser, Subcommand};
use commands::{configure::ActixTemplateProfile, notes, orgs};

mod commands;
mod errors;

#[derive(Parser)]
#[command(author, version)]
#[command(
    name = "actix-template-cli",
    about = "Actix Template CLI - CLI for Testing the Actix Web Template",
    long_about = "Actix Template CLI is a CLI for the Testing the Actix Web Template. 
    
    It allows you to login and create api keys to interact with the demo server."
)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// The name of the profile to use
    #[arg(short, long, env = "ACTIX_TEMPLATE_PROFILE")]
    profile: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Configures the Trieve CLI with your API key
    Login(Login),
    #[command(subcommand, about = "Commands for managing API Keys")]
    ApiKey(ApiKeyCommands),
    /// Command to manage profiles
    #[command(subcommand)]
    Profile(Profile),
    /// Command to manage organizations
    #[command(subcommand)]
    Orgs(OrgCommands),

    #[command(subcommand)]
    Notes(NoteCommands),
}

#[derive(Subcommand)]
enum NoteCommands {
    Create(CreateNote),
    Delete(DeleteNote),
    Edit(EditNote),
    List,
}

#[derive(Args)]
struct CreateNote {
    /// The title of the note you want to create
    title: Option<String>,
}

#[derive(Args)]
struct DeleteNote {
    /// The id of the note you want to delete
    id: Option<String>,
}

#[derive(Args)]
struct EditNote {
    /// The id of the note you want to edit
    id: Option<String>,
}

#[derive(Subcommand)]
enum OrgCommands {
    Create(CreateOrg),
    Delete,
    Rename,
    Invite(InviteToOrg),
    Leave(LeaveOrg),
}

#[derive(Args)]
struct CreateOrg {
    /// The name of the organization you want to create
    name: Option<String>,
}

#[derive(Args)]
struct LeaveOrg {
    /// The name of the organization you want to create
    id: Option<String>,
}

#[derive(Args)]
struct InviteToOrg {
    /// The id of the organization you want to invite the user to
    #[arg(short, long)]
    id: Option<String>,
    /// The user's email
    #[arg(short, long)]
    email: Option<String>,
}

#[derive(Subcommand)]
enum Profile {
    /// Switch to a different profile
    Switch(SwitchProfile),
    /// Delete a profile
    Delete(DeleteProfile),
}

#[derive(Subcommand)]
enum ApiKeyCommands {
    /// Generate a new API Key
    Generate(ApiKeyData),
    //TODO: List API Keys, Delete API Key
}

#[derive(Args)]
struct Login {
    /// API Key from the Actix Template Server
    #[arg(short, long, env = "ACTIX_TEMPLATE_API_KEY")]
    api_key: Option<String>,
    /// The URL of the Actix Template server
    #[arg(long, required = false)]
    api_url: Option<String>,
    /// Name the profile you are configuring
    #[arg(long, required = false)]
    profile_name: Option<String>,
}

#[derive(Args)]
struct ApiKeyData {
    /// The name of the API Key
    #[arg(short, long)]
    name: Option<String>,
}

#[derive(Args)]
struct SwitchProfile {
    /// The name of the profile to switch to
    profile_name: Option<String>,
}

#[derive(Args)]
struct DeleteProfile {
    /// The name of the profile to delete
    profile_name: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let profiles: ActixTemplateProfile = confy::load("actix_template", "profiles")
        .map_err(|e| {
            eprintln!("Error loading configuration: {:?}", e);
        })
        .unwrap_or_default();

    let settings = if args.profile.is_some() {
        let profile_name = args.profile.unwrap();
        let profile = profiles
            .inner
            .iter()
            .find(|p| p.name == profile_name)
            .ok_or_else(|| {
                eprintln!("Profile '{}' not found.", profile_name);
                std::process::exit(1);
            })
            .unwrap();

        profile.settings.clone()
    } else {
        profiles
            .inner
            .iter()
            .find(|p| p.selected)
            .cloned()
            .unwrap_or_default()
            .settings
    };

    match args.command {
        Some(Commands::Login(login)) => {
            commands::configure::login(login, settings).await;
        }
        Some(Commands::ApiKey(api_key)) => match api_key {
            ApiKeyCommands::Generate(api_key_data) => {
                commands::api_key::generate_api_key(settings, api_key_data)
                    .await
                    .map_err(|e| {
                        eprintln!("Error generating API Key: {:?}", e);
                        std::process::exit(1);
                    })
                    .unwrap();
            }
        },
        Some(Commands::Profile(profile)) => match profile {
            Profile::Switch(switch) => {
                commands::profile::switch_profile(switch, profiles.to_vec())
                    .map_err(|e| {
                        eprintln!("Error switching profile: {:?}", e);
                        std::process::exit(1);
                    })
                    .unwrap();
            }
            Profile::Delete(delete) => {
                commands::profile::delete_profile(delete, profiles.to_vec())
                    .map_err(|e| {
                        eprintln!("Error deleting profile: {:?}", e);
                        std::process::exit(1);
                    })
                    .unwrap();
            }
        },
        Some(Commands::Orgs(org)) => match org {
            OrgCommands::Create(org) => {
                _ = orgs::create_org(settings, org.name).await;
            }
            OrgCommands::Delete => orgs::delete_org(settings).await,
            OrgCommands::Rename => orgs::rename_org(settings).await,
            OrgCommands::Invite(invite) => {
                orgs::invite_user(invite.id, invite.email, settings).await
            }
            OrgCommands::Leave(leave_org) => {
                orgs::leave_org(leave_org.id, settings).await;
            }
        },

        Some(Commands::Notes(note_option)) => match note_option {
            NoteCommands::Create(create_note) => {
                notes::create_note_cmd(settings, create_note.title).await
            }
            NoteCommands::List => notes::list_notes_cmd(settings).await,
            NoteCommands::Edit(edit_args) => notes::edit_note_cmd(settings, edit_args.id).await,
            _ => unimplemented!("Notes command not implemented yet"),
        },
        _ => {
            println!("Command not implemented yet");
        }
    }
}
