pub mod curve;
pub mod input;
pub mod message;
pub mod output;

pub struct KeyMaster<'a> {
    pub inputs: Vec<input::Input<'a>>,
    pub outputs: Vec<output::Output<'a>>,
    pub curves: Vec<curve::Curve>,
    pub messages: Vec<message::Message>,
}

impl<'a> KeyMaster<'a> {
    pub fn new() -> KeyMaster<'a> {
        KeyMaster {
            inputs: Vec::new(),
            outputs: Vec::new(),
            curves: curve::Curve::build_curves(),
            messages: Vec::new(),
        }
    }
}
