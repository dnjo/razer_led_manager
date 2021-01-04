pub enum Effect {
    Static,
    Wave,
}

impl Effect {
    pub fn name(&self) -> &str {
        return match self {
            Effect::Static => "Static",
            Effect::Wave => "Wave"
        }
    }

    pub fn command(&self) -> EffectCommand {
        return match self {
            Effect::Static => EffectCommand {
                command_id: 1
            },
            Effect::Wave => EffectCommand {
                command_id: 2
            }
        }
    }
}

/// The command definition for a specific effect that is sent to a device.
#[derive(Debug)]
pub struct EffectCommand {
    pub command_id: u8,
}
