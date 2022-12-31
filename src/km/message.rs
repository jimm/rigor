pub struct Message {
    pub name: String,
    pub events: Vec<portmidi::types::MidiMessage>,
}

impl Message {
    pub fn new() -> Message {
        Message {
            name: "".to_string(),
            events: Vec::new(),
        }
    }
}
