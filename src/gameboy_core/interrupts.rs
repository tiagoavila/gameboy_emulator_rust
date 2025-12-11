pub enum InterruptType {
    VBlank,
    LCD,
    Timer,
    Serial,
    Joypad,
}

pub struct InterruptsHandler {

}

impl InterruptsHandler {
    pub fn new() -> Self {
        InterruptsHandler {

        }
    }
}