use portmidi::OutputPort;

pub struct Output<'a> {
    port: &'a OutputPort<'a>,
    pub name: String,
    pub device_name: String,
}

impl<'a> Output<'a> {
    pub fn new(port: &'a OutputPort, name: String, device_name: String) -> Output<'a> {
        Output {
            port: port,
            name: name,
            device_name: device_name,
        }
    }
}
