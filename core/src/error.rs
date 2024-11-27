use powershell_script::PsError;



#[derive(Debug)]
pub enum VolumeError {
    ScriptInvalid(PsError),
    StepOutOfRange(u8),

    UB(PsError),
}
