use km::KeyMaster;
use portmidi::PortMidi;
use std::env;

pub mod km;

const USAGE_LINES: [&str; 6] = [
    "[COMMAND]",
    "",
    "Commands:",
    "",
    "    list         Lists all input and output MIDI devices and exits.",
    "    help         This help.",
];

mod device_list {
    use portmidi::DeviceInfo;
    use portmidi::PortMidi;

    pub fn list_all_devices(pm: PortMidi) {
        let num_devices = pm.device_count();
        let mut inputs: Vec<DeviceInfo> = Vec::new();
        let mut outputs: Vec<DeviceInfo> = Vec::new();

        for i in 0..num_devices {
            let info = pm.device(i).expect("Device id error");
            if info.is_input() {
                inputs.push(info)
            } else {
                outputs.push(info)
            }
        }

        list("Inputs", inputs);
        list("Outputs", outputs);
    }

    fn list(title: &str, infos: Vec<DeviceInfo>) {
        println!("{title}");
        for info in infos {
            println!("  {:>2}: {}", info.id(), info.name());
        }
    }
}

fn usage(status: i32) {
    for line in USAGE_LINES {
        println!("{}", line)
    }
    if status != 0 {
        std::process::exit(status);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // See https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
    if args.len() == 2 {
        match &args[1][..] {
            "help" => usage(0),
            "list" => {
                let pm = PortMidi::new().expect("PortMidi initialization error");
                device_list::list_all_devices(pm);
            }
            _ => usage(1),
        }
    } else {
        let km = KeyMaster::new();
        println!("km has {} messages", km.messages.len());
        for m in &km.messages {
            println!("  message has {} events", m.events.len());
        }
    }
}
