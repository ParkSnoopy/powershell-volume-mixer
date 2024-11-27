use std::fmt;

use powershell_script::{
    PsScriptBuilder,
};

use crate::{
    error::VolumeError,
    script,
};



#[derive(Clone)]
pub struct Audio {
    is_mute: bool,
    step: u8,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            is_mute: false,
            step: 050u8,
        }
    }

    fn run_script(&self, script: impl AsRef<str>) -> Result<(), VolumeError> {
        let powershell = PsScriptBuilder::new()
            .no_profile(true)
            .non_interactive(true)
            .hidden(true)
            .print_commands(false)
            .build();

        let full_script = format!("{}\n{}",
            script::PS_VOLUME_SCRIPT,
            script.as_ref()
        );

        if let Err(e) = powershell.run(&full_script) {
            return Err(VolumeError::ScriptInvalid(e));
        };

        Ok(())
    }

    #[inline]
    pub fn is_muted(&self) -> bool {
        self.is_mute
    }

    #[inline]
    pub fn get_volume(&self) -> u8 {
        self.step
    }

    pub fn set_volume(&mut self, step: u8) -> Result<(), VolumeError> {
        if step > 100 {
            return Err(VolumeError::StepOutOfRange(step));
        }

        self.step = step;
        self.run_script(
            format!("[audio]::Volume = {}",
                (step as f32) / 100.0,
            )
        )
    }

    pub fn mute_unmute(&mut self) -> Result<(), VolumeError> {
        if self.is_muted() {
            self.unmute()
        } else {
            self.mute()
        }
    }

    pub fn mute(&mut self) -> Result<(), VolumeError>  {
        if self.is_muted() {
            return Ok(());
        };

        let result = self.run_script("[audio]::Mute = $true");

        if result.is_ok() {
            self.is_mute = true;
        }

        result
    }

    pub fn unmute(&mut self) -> Result<(), VolumeError>  {
        if !self.is_muted() {
            return Ok(());
        };

        let result = self.run_script("[audio]::Mute = $false");

        if result.is_ok() {
            self.is_mute = false;
        }

        result
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Audio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self.is_muted() {
            true => "(Muted)".to_string(),
            false => self.step.to_string(),
        };

        write!(f, "[Audio]::Volume={}", state)
    }
}
