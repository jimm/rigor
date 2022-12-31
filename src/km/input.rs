use portmidi::InputPort;

pub struct Input<'a> {
    port: &'a InputPort<'a>,
    pub name: String,
    pub device_name: String,
}

impl<'a> Input<'a> {
    pub fn new(port: &'a InputPort, name: String, device_name: String) -> Input<'a> {
        Input {
            port: port,
            name: name,
            device_name: device_name,
        }
    }
}
