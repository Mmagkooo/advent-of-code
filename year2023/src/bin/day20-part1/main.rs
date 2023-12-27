use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

type ModuleName = String;
type ModuleMap = HashMap<ModuleName, Module>;

const BROADCAST_MODULE_NAME: &str = "broadcaster";
const BUTTON_MODULE_NAME: &str = "button";

#[derive(Debug)]
struct FlipFlipModule {
    name: ModuleName,
    on: bool,
    outputs: Vec<ModuleName>,
}

#[derive(Debug)]
struct ConjunctionModule {
    name: ModuleName,
    memory: HashMap<ModuleName, Pulse>,
    outputs: Vec<ModuleName>,
}

impl ConjunctionModule {
    fn add_input(&mut self, module_name: &str) {
        self.memory.insert(module_name.into(), Pulse::Low);
    }
}

#[derive(Debug)]
struct BroadcastModule {
    name: ModuleName,
    outputs: Vec<ModuleName>,
}

#[derive(Debug)]
enum Module {
    FlipFlop(FlipFlipModule),
    Conjunction(ConjunctionModule),
    Broadcast(BroadcastModule),
    DeadEnd { name: String, outputs: Vec<String> },
}

impl Module {
    fn from_str(s: &str) -> Self {
        let split_content: Vec<&str> = s.split(" -> ").collect();
        assert_eq!(split_content.len(), 2);
        let left = split_content[0];
        let right = split_content[1];
        let outputs = right.split(", ").map(|s| s.into()).collect();

        match &left[0..1] {
            "%" => Self::FlipFlop(FlipFlipModule {
                name: left[1..].to_string(),
                on: false,
                outputs,
            }),
            "&" => Self::Conjunction(ConjunctionModule {
                name: left[1..].to_string(),
                memory: HashMap::new(),
                outputs,
            }),
            _ => {
                assert_eq!(left, BROADCAST_MODULE_NAME, "Invalid module specifier: {s}");
                Self::Broadcast(BroadcastModule {
                    name: BROADCAST_MODULE_NAME.into(),
                    outputs,
                })
            }
        }
    }

    fn propagate(&mut self, input_module_name: &str, input_pulse: &Pulse) -> Option<Pulse> {
        match self {
            Module::FlipFlop(module) => match input_pulse {
                Pulse::High => None,
                Pulse::Low => {
                    module.on = !module.on;
                    match module.on {
                        true => Some(Pulse::High),
                        false => Some(Pulse::Low),
                    }
                }
            },
            Module::Conjunction(module) => {
                let memorized_pulse = module.memory.get_mut(input_module_name).unwrap();

                *memorized_pulse = input_pulse.clone();

                if module.memory.values().all(|p| p == &Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Module::Broadcast(_) => Some(Pulse::Low),
            Module::DeadEnd { .. } => None,
        }
    }

    fn name(&self) -> &String {
        match self {
            Module::FlipFlop(m) => &m.name,
            Module::Conjunction(m) => &m.name,
            Module::Broadcast(m) => &m.name,
            Module::DeadEnd { name, .. } => name,
        }
    }

    fn outputs(&self) -> &Vec<ModuleName> {
        match self {
            Module::FlipFlop(m) => &m.outputs,
            Module::Conjunction(m) => &m.outputs,
            Module::Broadcast(m) => &m.outputs,
            Module::DeadEnd { outputs, .. } => outputs,
        }
    }
}

fn main() {
    let mut modules: ModuleMap = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| Module::from_str(&l))
        .map(|m| (m.name().clone(), m))
        .collect();

    let module_names: Vec<String> = modules.keys().cloned().collect();
    for module_name in module_names.iter() {
        let module = modules.get(module_name).unwrap();
        let output_module_names = module.outputs().clone();
        for output_module_name in output_module_names {
            if !modules.contains_key(&output_module_name) {
                modules.insert(
                    output_module_name.clone(),
                    Module::DeadEnd {
                        name: output_module_name.clone(),
                        outputs: vec![],
                    },
                );
            }
            if let Module::Conjunction(output_module) =
                modules.get_mut(&output_module_name).unwrap()
            {
                output_module.add_input(module_name);
            }
        }
    }

    let mut pulse_counter: HashMap<Pulse, usize> = Default::default();
    for _ in 0..1000 {
        // queue of (receipient, pulse); initially populated with: button -low-> broadcast
        let mut pulse_queue = VecDeque::from([(
            BUTTON_MODULE_NAME.to_string(),
            BROADCAST_MODULE_NAME.to_string(),
            Pulse::Low,
        )]);

        while !pulse_queue.is_empty() {
            let (source_name, recipient_name, pulse) = pulse_queue.pop_front().unwrap();
            *pulse_counter.entry(pulse.clone()).or_default() += 1;

            let recipient_module = modules.get_mut(&recipient_name).unwrap();
            if let Some(new_pulse) = recipient_module.propagate(&source_name, &pulse) {
                let new_source_name = recipient_name;
                for new_recipient_name in recipient_module.outputs() {
                    pulse_queue.push_back((
                        new_source_name.clone(),
                        new_recipient_name.clone(),
                        new_pulse.clone(),
                    ));
                }
            }
        }
    }

    println!("{}", pulse_counter.values().product::<usize>());
}
