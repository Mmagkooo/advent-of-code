use std::collections::{HashMap, HashSet, VecDeque};

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

const FINAL_MODULE_NAME: &str = "rx";

fn get_parent_modules(module_name: &String, modules: &ModuleMap) -> Vec<String> {
    modules
        .values()
        .filter(|m| m.outputs().contains(module_name))
        .map(|m| m.name().clone())
        .collect()
}

/// pairs of (greatgrandparent -> grandparent)
fn get_target_pairs(modules: &ModuleMap) -> HashSet<(ModuleName, ModuleName)> {
    let final_parents = get_parent_modules(&FINAL_MODULE_NAME.to_string(), &modules);
    assert_eq!(final_parents.len(), 1);
    let final_parent = &final_parents[0];

    let final_grandparents = get_parent_modules(final_parent, &modules);
    let final_greatgrandparents: Vec<String> = final_grandparents
        .iter()
        .map(|grandparent| {
            let greatgrandparents = get_parent_modules(&grandparent, &modules);
            assert_eq!(greatgrandparents.len(), 1);
            greatgrandparents[0].clone()
        })
        .collect();

    final_greatgrandparents
        .into_iter()
        .zip(final_grandparents)
        .collect()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
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

    println!("Assumption: final module has one conjunction module as a parent, which has N conjunction modules as its parent\
        Each of these N modules has one parent module - a conjunction one.");

    let target_pairs = get_target_pairs(&modules);
    let mut periods: HashMap<(ModuleName, ModuleName), usize> = HashMap::new();
    let mut button_presses = 0;

    'button_pressing: loop {
        button_presses += 1;
        // queue of (receipient, pulse); initially populated with: button -low-> broadcast
        let mut pulse_queue = VecDeque::from([(
            BUTTON_MODULE_NAME.to_string(),
            BROADCAST_MODULE_NAME.to_string(),
            Pulse::Low,
        )]);

        while !pulse_queue.is_empty() {
            let (source_name, recipient_name, pulse) = pulse_queue.pop_front().unwrap();

            let pair = (source_name.clone(), recipient_name.clone());
            if target_pairs.contains(&pair) && pulse == Pulse::Low {
                periods.insert(pair, button_presses);
                if target_pairs.len() == periods.len() {
                    break 'button_pressing;
                }
            }

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

    println!("{}", periods.into_values().reduce(lcm).unwrap());
}
