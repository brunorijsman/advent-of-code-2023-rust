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
    dest_value: PulseValue,                  // Most recent sent pulse value
    src_values: HashMap<String, PulseValue>, // Most recent received pulse values, keyed by source module name
    pulse_count: usize,                      // Number of received pulses
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
            dest_value: PulseValue::Low, // Initial value
            src_values: HashMap::new(),  // Filled in later in Network::from_file
            pulse_count: 0,
        }
    }

    fn process_pulse(&mut self, pulse: &Pulse) -> PulseValue {
        assert!(pulse.value != PulseValue::None);
        // Empty string means in pulse src_module from button
        if pulse.src_module != "" {
            *self.src_values.get_mut(&pulse.src_module).unwrap() = pulse.value;
        }
        let out_pulse_value = match self.module_type {
            ModuleType::Broadcaster => {
                // Broadcaster just forwards the pulse
                pulse.value
            }
            ModuleType::FlipFlop => {
                // FlipFlop toggles its output on each Low pulse, nothing happens on High pulse
                if pulse.value == PulseValue::High {
                    PulseValue::None
                } else {
                    self.dest_value.opposite()
                }
            }
            ModuleType::Conjunction => {
                // Conjunction outputs Low only if all inputs are High, otherwise outputs High
                if self.src_values.values().all(|&v| v == PulseValue::High) {
                    PulseValue::Low
                } else {
                    PulseValue::High
                }
            }
        };
        if out_pulse_value != PulseValue::None {
            self.dest_value = out_pulse_value;
        }
        out_pulse_value
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
        let rx_module = Module {
            name: "rx".to_string(),
            module_type: ModuleType::Broadcaster, // Type doesn't matter for rx
            dest_modules: Vec::new(),
            dest_value: PulseValue::Low,
            src_values: HashMap::new(),
            pulse_count: 0,
        };
        modules.insert(rx_module.name.clone(), rx_module);
        let mut connections: Vec<(String, String)> = Vec::new();
        for src_module in modules.values() {
            for dest_module_name in &src_module.dest_modules {
                connections.push((src_module.name.clone(), dest_module_name.clone()));
            }
        }
        for (src_module_name, dest_module_name) in connections {
            if let Some(dest_module) = modules.get_mut(&dest_module_name) {
                dest_module
                    .src_values
                    .insert(src_module_name, PulseValue::Low); // Initial value
            }
        }
        Network { modules }
    }

    fn push_button(&mut self) {
        let mut pulse_queue: VecDeque<Pulse> = VecDeque::new();
        let button_pulse = Pulse {
            value: PulseValue::Low,
            src_module: "".to_string(), // Empty string means button
            dest_module: "broadcaster".to_string(),
        };
        pulse_queue.push_back(button_pulse);
        while let Some(pulse) = pulse_queue.pop_front() {
            // Dest module may not exist; second example contains an "untyped module named output"
            if !self.modules.contains_key(&pulse.dest_module) {
                continue;
            }
            let dest_module: &mut Module = self.modules.get_mut(&pulse.dest_module).unwrap();
            let out_pulse_value = dest_module.process_pulse(&pulse);
            if out_pulse_value == PulseValue::None {
                continue;
            }
            for next_dest_module_name in &dest_module.dest_modules {
                let next_pulse = Pulse {
                    value: out_pulse_value,
                    src_module: dest_module.name.clone(),
                    dest_module: next_dest_module_name.clone(),
                };
                pulse_queue.push_back(next_pulse);
                // println!(
                //     "{} -{}-> {}",
                //     dest_module.name,
                //     match out_pulse_value {
                //         PulseValue::None => "none",
                //         PulseValue::High => "high",
                //         PulseValue::Low => "low",
                //     },
                //     next_dest_module_name
                // );
            }
        }
    }

    fn keep_pressing_button(&mut self) {
        for nr in 1..100_000_000_000usize {
            println!("\nButton press #{nr}");
            self.push_button();
            println!("Module rx pulse count: {}", self.modules["rx"].pulse_count)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PulseValue {
    None,
    High,
    Low,
}

impl PulseValue {
    fn opposite(&self) -> PulseValue {
        match self {
            PulseValue::None => PulseValue::None,
            PulseValue::High => PulseValue::Low,
            PulseValue::Low => PulseValue::High,
        }
    }
}

#[derive(Debug)]
struct Pulse {
    value: PulseValue,
    src_module: String, // Empty string means from button
    dest_module: String,
}

fn main() {
    let mut network = Network::from_file("puzzle_input");
    network.keep_pressing_button();
}
