use std::collections::HashMap;

use regex::Regex;

struct Workflow {
    conditions: Vec<Condition>,
    default: ExecutionOutcome,
}

impl Workflow {
    fn from_str(s: &str) -> Self {
        let segments: Vec<&str> = s.split(",").collect();

        let (last, rest) = segments.split_last().unwrap();

        Self {
            conditions: rest
                .iter()
                .map(|segment| Condition::from_str(*segment))
                .collect(),
            default: ExecutionOutcome::from_str(last),
        }
    }

    fn execute(&self, part: &Part) -> ExecutionOutcome {
        for condition in self.conditions.iter() {
            match condition.execute(part) {
                outcome @ (ExecutionOutcome::Accepted
                | ExecutionOutcome::Rejected
                | ExecutionOutcome::NextWorkflow(_)) => return outcome,
                ExecutionOutcome::NextCondition => (),
            }
        }

        self.default.clone()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum ExecutionOutcome {
    Accepted,
    Rejected,
    NextWorkflow(String),
    NextCondition,
}

impl ExecutionOutcome {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            other => Self::NextWorkflow(other.into()),
        }
    }
}

struct Workflows {
    collection: HashMap<String, Workflow>,
}

impl Workflows {
    fn execute(&self, part: &Part) -> ExecutionOutcome {
        let mut workflow = self.collection.get("in").unwrap();
        loop {
            let workflow_outcome = workflow.execute(part);
            assert_ne!(workflow_outcome, ExecutionOutcome::NextCondition);
            if let ExecutionOutcome::NextWorkflow(next_workflow_name) = workflow_outcome {
                workflow = self.collection.get(&next_workflow_name).unwrap();
            } else {
                return workflow_outcome;
            }
        }
    }
}

enum Operator {
    LessThan,
    GreaterThan,
}

impl Operator {
    fn from_str(s: &str) -> Self {
        match s {
            ">" => Self::GreaterThan,
            "<" => Self::LessThan,
            other => panic!("Invalid operator: {other}"),
        }
    }

    fn execute(&self, v1: i32, v2: i32) -> bool {
        match self {
            Operator::LessThan => v1 < v2,
            Operator::GreaterThan => v1 > v2,
        }
    }
}

struct Condition {
    left_operand_name: String,
    right_operand: i32,
    operator: Operator,
    successful_outcome: ExecutionOutcome,
}

impl Condition {
    fn from_str(s: &str) -> Self {
        let condition_parts: Vec<&str> = s.split(":").collect();
        assert_eq!(condition_parts.len(), 2);
        let conditional_str = condition_parts[0];
        let outcome_str = condition_parts[1];

        Condition {
            left_operand_name: String::from(&conditional_str[..1]),
            right_operand: String::from(&conditional_str[2..]).parse().unwrap(),
            operator: Operator::from_str(&conditional_str[1..2]),
            successful_outcome: ExecutionOutcome::from_str(outcome_str),
        }
    }

    fn execute(&self, part: &Part) -> ExecutionOutcome {
        let left_operand = part.get(&self.left_operand_name);
        match self.operator.execute(left_operand, self.right_operand) {
            true => self.successful_outcome.clone(),
            false => ExecutionOutcome::NextCondition,
        }
    }
}

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn from_str(s: &str) -> Self {
        let re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
        let (_, caps) = re.captures(s).unwrap().extract::<4>();
        let parsed_caps = caps
            .iter()
            .map(|c| c.parse().unwrap())
            .collect::<Vec<i32>>();
        Self {
            x: parsed_caps[0],
            m: parsed_caps[1],
            a: parsed_caps[2],
            s: parsed_caps[3],
        }
    }

    fn sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }

    fn get(&self, name: &str) -> i32 {
        match name {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            other => panic!("Invalid name: {other}"),
        }
    }
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|l| l.unwrap());

    let mut workflow_vec: HashMap<String, Workflow> = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let line_parts: Vec<&str> = line.split("{").collect();
        let workflow_name = line_parts[0];
        let workflow_str = line_parts[1].strip_suffix("}").unwrap();
        workflow_vec.insert(workflow_name.into(), Workflow::from_str(&workflow_str));
    }
    let workflows = Workflows {
        collection: workflow_vec,
    };

    let mut parts: Vec<Part> = vec![];
    while let Some(line) = lines.next() {
        parts.push(Part::from_str(&line));
    }

    let sol: i32 = parts
        .iter()
        .filter(|p| workflows.execute(&p) == ExecutionOutcome::Accepted)
        .map(|p| p.sum())
        .sum();
    println!("{sol}");
}
