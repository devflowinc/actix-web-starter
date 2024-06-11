use crate::{
    commands::configure::ActixTemplateProfile, errors::DefaultError, DeleteProfile, SwitchProfile,
};

use super::configure::ActixTemplateProfileInner;

pub fn switch_profile(
    profile_data: SwitchProfile,
    profiles: Vec<ActixTemplateProfileInner>,
) -> Result<(), DefaultError> {
    let profile_name = profile_data.profile_name.unwrap_or_else(|| {
        let profile_name = inquire::Select::new(
            "Select a profile to switch to:",
            profiles.iter().map(|p| p.name.clone()).collect(),
        )
        .prompt()
        .unwrap();
        profile_name
    });

    profiles
        .iter()
        .find(|p| p.name == profile_name)
        .ok_or_else(|| DefaultError::new(format!("Profile '{}' not found.", profile_name).as_str()))
        .unwrap();

    let profiles = profiles
        .iter()
        .map(|p| {
            if p.name == profile_name {
                ActixTemplateProfileInner {
                    name: p.name.clone(),
                    selected: true,
                    settings: p.settings.clone(),
                }
            } else {
                ActixTemplateProfileInner {
                    name: p.name.clone(),
                    selected: false,
                    settings: p.settings.clone(),
                }
            }
        })
        .collect::<Vec<ActixTemplateProfileInner>>();

    confy::store(
        "actix_template",
        "profiles",
        ActixTemplateProfile { inner: profiles },
    )
    .map_err(|e| DefaultError::new(format!("Error saving configuration: {:?}", e).as_str()))
    .unwrap();

    println!("Switched to profile '{}'.", profile_name);

    Ok(())
}

pub fn delete_profile(
    profile_data: DeleteProfile,
    profiles: Vec<ActixTemplateProfileInner>,
) -> Result<(), DefaultError> {
    let profile_name = profile_data.profile_name.unwrap_or_else(|| {
        let profile_name = inquire::Select::new(
            "Select a profile to delete:",
            profiles.iter().map(|p| p.name.clone()).collect(),
        )
        .prompt()
        .unwrap();
        profile_name
    });

    let profile = profiles
        .iter()
        .find(|p| p.name == profile_name)
        .ok_or_else(|| DefaultError::new(format!("Profile '{}' not found.", profile_name).as_str()))
        .unwrap();

    let mut profiles = profiles
        .iter()
        .filter(|p| p.name != profile_name)
        .map(|p| p.clone())
        .collect::<Vec<ActixTemplateProfileInner>>();

    if profile.selected {
        if profiles.is_empty() {
            return Err(DefaultError::new("Cannot delete the last profile."));
        }

        profiles[0].selected = true;
    }

    confy::store(
        "actix_template",
        "profiles",
        ActixTemplateProfile { inner: profiles },
    )
    .map_err(|e| DefaultError::new(format!("Error saving configuration: {:?}", e).as_str()))?;

    println!("Deleted profile '{}'.", profile_name);

    Ok(())
}
