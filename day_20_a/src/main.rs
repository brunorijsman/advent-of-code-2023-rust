use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    dest_modules: Vec<String>,
    src_modules: Vec<String>,
    src_values: Vec<PulseValue>,
}

impl Module {
    fn from_line(line: &str) -> Self {
        let source_and_dest: Vec<&str> = line.split("->").collect();
        assert!(source_and_dest.len() == 2);
        let source = source_and_dest[0].trim();
        let dest = source_and_dest[1].trim();
        let (name, module_type) = if source == "broadcaster" {
            (source.to_string(), ModuleType::Broadcaster)
        } else {
            let first_char = source.chars().nth(0).unwrap();
            let name = source.chars().skip(1).collect();
            match first_char {
                '%' => (name, ModuleType::FlipFlop),
                '&' => (name, ModuleType::Conjunction),
                _ => panic!("Unknown module type"),
            }
        };
        let dest_modules: Vec<String> = dest.split(',').map(|s| s.trim().to_string()).collect();
        Module {
            name,
            module_type,
            dest_modules,
            src_modules: Vec::new(), // Filled in later
            src_values: Vec::new(),  // Filled in later
        }
    }
}

#[derive(Debug)]
struct Network {
    modules: HashMap<String, Module>, // Keyed by module name
}

impl Network {
    fn from_file(filename: &str) -> Self {
        let lines = read_to_string(filename).unwrap();
        let mut modules: HashMap<String, Module> = HashMap::new();
        for line in lines.lines() {
            let module = Module::from_line(line);
            modules.insert(module.name.clone(), module);
        }
        let mut connections: Vec<(String, String)> = Vec::new();
        for src_module in modules.values() {
            for dest_module_name in &src_module.dest_modules {
                connections.push((src_module.name.clone(), dest_module_name.clone()));
            }
        }
        for (src_module_name, dest_module_name) in connections {
            if let Some(dest_module) = modules.get_mut(&dest_module_name) {
                dest_module.src_modules.push(src_module_name);
                dest_module.src_values.push(PulseValue::Low); // Initial value
            }
        }
        Network { modules }
    }

    fn push_button(&self) {
        let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();
        let button_pulse = Pulse {
            value: PulseValue::Low,
            dest_module: "broadcaster".to_string(),
        };
        pulse_queue.push_back(button_pulse);
        while let Some(pulse) = pulse_queue.pop_front() {
            println!("Processing pulse: {:?}", pulse);
            let dest_module = self.modules.get(&pulse.dest_module).unwrap();
            println!("Send to module: {:?}", dest_module.name);
            match dest_module.module_type {
                ModuleType::Broadcaster => {
                    for dest in &dest_module.dest_modules {
                        let new_pulse = Pulse {
                            value: PulseValue::High,
                            dest_module: dest.clone(),
                        };
                        pulse_queue.push_back(new_pulse);
                    }
                }
                ModuleType::FlipFlop => {
                    // FlipFlop logic would go here
                }
                ModuleType::Conjunction => {
                    // Conjunction logic would go here
                }
            }
            // Further processing logic would go here
        }
    }
}

#[derive(Debug)]
enum PulseValue {
    High,
    Low,
}

#[derive(Debug)]
struct Pulse {
    value: PulseValue,
    dest_module: String,
}

fn main() {
    let network = Network::from_file("example_input");
    println!("Network: {:?}", network);
    network.push_button();
}
