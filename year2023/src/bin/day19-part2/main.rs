use std::collections::HashMap;

#[derive(Clone, Debug)]
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
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum ExecutionOutcome {
    Accepted,
    Rejected,
    NextWorkflow(String),
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
    fn count_successful(&self) -> i64 {
        let mut final_restrictions_count = 0;
        let mut queue: Vec<(Workflow, PartRestrictions)> = vec![(
            self.collection.get("in").unwrap().clone(),
            PartRestrictions::maximum(),
        )];

        while !queue.is_empty() {
            let (workflow, mut current_restrictions) = queue.pop().unwrap();

            for condition in workflow.conditions {
                let left_operand_name = condition.left_operand_name.as_str();

                match &condition.successful_outcome {
                    ExecutionOutcome::Accepted => {
                        final_restrictions_count += current_restrictions
                            .intersect_single(
                                left_operand_name,
                                &condition.successful_restriction(),
                            )
                            .count()
                    }
                    ExecutionOutcome::Rejected => (),
                    ExecutionOutcome::NextWorkflow(next_workflow_name) => {
                        let next_workflow = self
                            .collection
                            .get(next_workflow_name.as_str())
                            .unwrap()
                            .clone();
                        let next_restrictions = current_restrictions.intersect_single(
                            left_operand_name,
                            &condition.successful_restriction(),
                        );
                        queue.push((next_workflow, next_restrictions))
                    }
                }

                current_restrictions = current_restrictions
                    .intersect_single(left_operand_name, &condition.unsuccessful_restriction());
            }

            match workflow.default {
                ExecutionOutcome::Accepted => {
                    final_restrictions_count += current_restrictions.count();
                }
                ExecutionOutcome::Rejected => (),
                ExecutionOutcome::NextWorkflow(next_workflow_name) => {
                    let next_workflow = self
                        .collection
                        .get(next_workflow_name.as_str())
                        .unwrap()
                        .clone();
                    queue.push((next_workflow, current_restrictions))
                }
            }
        }

        final_restrictions_count
    }
}

#[derive(Clone, Debug)]
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
}

#[derive(Clone, Debug)]
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

    fn successful_restriction(&self) -> Restriction {
        match self.operator {
            Operator::LessThan => Restriction::with_upper(self.right_operand),
            Operator::GreaterThan => Restriction::with_lower(self.right_operand),
        }
    }

    fn unsuccessful_restriction(&self) -> Restriction {
        match self.operator {
            Operator::LessThan => Restriction::with_lower(self.right_operand - 1),
            Operator::GreaterThan => Restriction::with_upper(self.right_operand + 1),
        }
    }
}

#[derive(Clone, Debug)]
struct Restriction {
    lower: i32,
    upper: i32,
}

impl Restriction {
    const DEFAULT_LOWER: i32 = 1 - 1;
    const DEFAULT_UPPER: i32 = 4000 + 1;

    fn empty() -> Self {
        Self { lower: 0, upper: 0 }
    }

    fn maximum() -> Self {
        Self {
            lower: Self::DEFAULT_LOWER,
            upper: Self::DEFAULT_UPPER,
        }
    }

    fn with_lower(lower: i32) -> Self {
        Self {
            lower,
            upper: Self::DEFAULT_UPPER,
        }
    }

    fn with_upper(upper: i32) -> Self {
        Self {
            lower: Self::DEFAULT_LOWER,
            upper,
        }
    }

    fn count(&self) -> i32 {
        // neither bound is included
        // e.g. if lower 1 (i.e. > 1) and upper 2 (i.e. < 2): 2 - 1 - 1 = 0
        self.upper - self.lower - 1
    }

    fn intersect(&self, other: &Self) -> Self {
        let (smaller, greater) = if self.lower < other.lower {
            (self, other)
        } else {
            (other, self)
        };

        if smaller.upper <= greater.lower {
            Self::empty()
        } else {
            Self {
                lower: std::cmp::max(smaller.lower, greater.lower),
                upper: std::cmp::min(smaller.upper, greater.upper),
            }
        }
    }
}

#[derive(Debug, Clone)]
struct PartRestrictions {
    mapping: HashMap<String, Restriction>,
}

impl PartRestrictions {
    const PROPS: [&str; 4] = ["x", "m", "a", "s"];

    fn maximum() -> Self {
        Self {
            mapping: HashMap::from_iter(Self::PROPS.map(|s| (s.into(), Restriction::maximum()))),
        }
    }

    fn count(&self) -> i64 {
        self.mapping
            .values()
            .map(|r| r.count() as i64)
            .reduce(|acc, e| acc * e)
            .unwrap()
    }

    fn intersect_single(&self, prop: &str, restriction: &Restriction) -> Self {
        let mut new_mapping = self.mapping.clone();
        new_mapping.insert(
            prop.into(),
            new_mapping.get(prop).unwrap().intersect(restriction),
        );
        Self {
            mapping: new_mapping,
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

    println!("{}", workflows.count_successful());
}
