#![cfg(windows)]

use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::dialog::WarningDialogModel;
use crate::settings::EditorSettings;

#[derive(Resource, Clone)]
pub struct StudioSettings(pub EditorSettings);

#[derive(Resource, Clone)]
pub struct StudioDialog(pub WarningDialogModel);

impl Default for StudioDialog {
    fn default() -> Self {
        Self(WarningDialogModel::default())
    }
}

impl Deref for StudioSettings {
    type Target = EditorSettings;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StudioSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for StudioDialog {
    type Target = WarningDialogModel;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StudioDialog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
