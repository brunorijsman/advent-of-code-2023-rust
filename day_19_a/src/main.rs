use std::fs::read_to_string;
use std::collections::hash_map::HashMap;

#[derive(Debug)]
enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Clone, Debug)]
enum Action {
    Accept,
    Reject,
    Workflow(String),
}

impl Action {
    fn from_str(s: &str) -> Action {
        if s == "A" {
            Action::Accept
        } else if s == "R" {
            Action::Reject
        } else {
            Action::Workflow(s.to_string())
        }
    }
}   

#[derive(Debug)]
struct Rule {
    category: char,
    comparison: Comparison,
    value: usize,
    action:Action
}

impl Rule {
    fn from_str(s: &str) -> Rule {
        let segments = s.trim().split(':').collect::<Vec<&str>>();
        assert!(segments.len() == 2);
        let condition_str = segments[0];
        let category = condition_str.chars().next().unwrap();
        assert!("xmas".contains(category));
        let comparison_char = condition_str.chars().nth(1).unwrap();
        let comparison = match comparison_char {
            '<' => Comparison::LessThan,
            '>' => Comparison::GreaterThan,
            _ => panic!("Invalid comparison operator"),
        };
        let value = condition_str[2..].parse().unwrap();
        let action_str = segments[1];
        let action = Action::from_str(action_str);
        Rule{category, comparison, value, action}
    }

    fn apply_to_part(&self, part: &Part) -> Option<Action> {
        let part_value = part.get_category_value(self.category);
        let condition_satisfied = match self.comparison {
            Comparison::LessThan => part_value < self.value,
            Comparison::GreaterThan => part_value > self.value
        };
        if condition_satisfied {
            Some(self.action.clone())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_action: Action,
}

impl Workflow {
    fn from_string(s: &str) -> Workflow {
        let open_curly_pos = s.find('{').unwrap();
        let name = s[..open_curly_pos].to_string();
        let inside_curly_str = &s[open_curly_pos+1..s.len()-1];
        let part_str_vec = inside_curly_str.split(',').collect::<Vec<&str>>();
        let default_action_str = part_str_vec.last().unwrap().trim();
        let default_action = Action::from_str(default_action_str);
        let rule_str_vec = &part_str_vec[..part_str_vec.len()-1];
        let mut rules: Vec<Rule> = Vec::new();
        for rule_str in rule_str_vec {
            let rule = Rule::from_str(rule_str);
            rules.push(rule);
        }
        Workflow {name, rules, default_action}
    }

    fn apply_to_part(&self, part: &Part) -> Action {
        for rule in &self.rules {
            if let Some(action) = rule.apply_to_part(part) {
                return action
            }
        }
        self.default_action.clone()
    }
}

struct WorkflowSet {
    workflows: Vec<Workflow>,
    name_to_index: HashMap<String, usize>,
}

impl WorkflowSet {
    fn new() -> WorkflowSet {
        WorkflowSet{workflows: Vec::new(), name_to_index: HashMap::new()}
    }

    fn add_workflow(&mut self, workflow: Workflow) {
        let index = self.workflows.len();
        self.name_to_index.insert(workflow.name.clone(), index);
        self.workflows.push(workflow);
    }

    fn is_part_accepted(&self, part: &Part) -> bool {
        let mut workflow_index = self.name_to_index["in"];
        loop {
            let workflow = self.workflows.get(workflow_index).unwrap();
            let action = workflow.apply_to_part(part);
            match action {
                Action::Accept => return true,
                Action::Reject => return false,
                Action::Workflow(workflow_name) => {
                    workflow_index = self.name_to_index[&workflow_name]
                }
            }
        }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn from_string(s: &str) -> Part {
        assert!(s.chars().nth(0) == Some('{'));
        assert!(s.chars().last() == Some('}'));
        let parts = s[1..s.len()-1].split(',').collect::<Vec<&str>>();
        assert!(parts.len() == 4);
        Part {
            x: Self::parse_assignments(parts[0], 'x'),
            m: Self::parse_assignments(parts[1], 'm'),
            a: Self::parse_assignments(parts[2], 'a'),
            s: Self::parse_assignments(parts[3], 's'),
        }
    }

    fn parse_assignments(s: &str, category: char) -> usize {
        assert!(s.chars().next() == Some(category));
        s[2..].parse().unwrap()
    }

    fn get_category_value(&self, category: char) -> usize {
        match category {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Unknown category {category}")
        }
    }

    fn total_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

}

type PartSet = Vec<Part>;

fn main() {
    let (workflows, parts) = read_input();
    let mut total_ratings_sum = 0;
    for part in parts {
        if workflows.is_part_accepted(&part) {
            total_ratings_sum += part.total_rating();
        }
    }
    println!("Sum of ratings of all accepted parts = {total_ratings_sum}");
}

fn read_input() -> (WorkflowSet, PartSet) {
    let lines = read_to_string("puzzle_input").unwrap();
    let mut workflows: WorkflowSet = WorkflowSet::new();
    let mut parts: PartSet = Vec::new();
    let mut reading_workflows = true;
    for line in lines.lines() {
        if reading_workflows {
            if line.trim().is_empty() {
                reading_workflows = false;
            } else {
                let workflow = Workflow::from_string(&line);
                workflows.add_workflow(workflow);
            }
        } else {
            let part = Part::from_string(&line);
            parts.push(part);
        }
    }
    (workflows, parts)
}
