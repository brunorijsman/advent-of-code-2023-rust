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

    fn match_workflow(&self, workflow_name: &String, in_partition: &Partition) -> Partition {
        let workflow_index = self.name_to_index[workflow_name];
        let workflow = &self.workflows[workflow_index];
        let mut workflow_match_partition = Partition::no_parts();
        let mut rest_partition = in_partition.clone();
        for rule in &workflow.rules {
            let (rule_match_partition, rest_partition) = rest_partition.split(rule);
            workflow_match_partition = self.apply_action_to_partition(&rule.action, &workflow_match_partition, &rule_match_partition,);
        }
        self.apply_action_to_partition(&workflow.default_action, &workflow_match_partition, &rest_partition);
        workflow_match_partition
    }

    fn apply_action_to_partition(&self, action: &Action, old_workflow_match_partition: &Partition, rule_match_partition: &Partition) -> Partition {
        match action {
            Action::Accept => {
                old_workflow_match_partition.extend(rule_match_partition)
            }
            Action::Reject => {
                old_workflow_match_partition.clone()
            } 
            Action::Workflow(next_workflow_name) => {
                let next_workflow_match_partition = self.match_workflow(next_workflow_name, rule_match_partition);
                old_workflow_match_partition.extend(&next_workflow_match_partition)
            }
        }
    }

    fn count_matching_parts(&self) -> usize {
        let in_partition = Partition::all_parts();
        let match_partition = self.match_workflow(&"in".to_string(), &in_partition);
        let count = match_partition.number_of_matches();
        count
    }
}

type Range = (usize, usize);

#[derive(Clone, Debug)]
struct Partition {
    category_ranges: HashMap<char, Vec<Range>>,
}

impl Partition {
    fn no_parts() -> Partition {
        let mut category_ranges = HashMap::new();
        for category in ['x', 'm', 'a', 's'] {
            category_ranges.insert(category, vec![]);
        }
        Partition { category_ranges }
    }

    fn all_parts() -> Partition {
        let mut category_ranges = HashMap::new();
        for category in ['x', 'm', 'a', 's'] {
            category_ranges.insert(category, vec![(1, 4000)]);
        }
        Partition { category_ranges }
    }

    fn clone_except_category(&self, category: char) -> Partition {
        let mut new_category_ranges = HashMap::new();
        for (&cat, ranges) in &self.category_ranges {
            if cat != category {
                new_category_ranges.insert(cat, ranges.clone());
            }
        }
        Partition { category_ranges: new_category_ranges }
    }   

    fn number_of_matches(&self) -> usize {
        let mut total = 1;
        for ranges in self.category_ranges.values() {
            let mut category_total = 0;
            for (start, end) in ranges {
                category_total += end - start + 1;
            }
            total *= category_total;
        }
        total
    }

    fn split(&self, rule: &Rule) -> (Partition, Partition) {
        let mut match_partition = self.clone_except_category(rule.category);
        let mut non_match_partition = self.clone_except_category(rule.category);
        for range in self.category_ranges.get(&rule.category).unwrap() {
            let (match_range, non_match_range) = Partition::split_range(range, rule);
            if let Some(mr) = match_range {
                match_partition.category_ranges.entry(rule.category).or_insert(Vec::new()).push(mr);
            }
            if let Some(nmr) = non_match_range {
                non_match_partition.category_ranges.entry(rule.category).or_insert(Vec::new()).push(nmr);
            }
        }   
        (match_partition, non_match_partition)
    }

    fn split_range(range: &Range, rule: &Rule) -> (Option<Range>, Option<Range>) {
        let match_range = Partition::match_sub_range(range, rule);
        let non_match_range = Partition::non_match_sub_range(range, rule);
        (match_range, non_match_range)
    }   

    fn match_sub_range(range: &Range, rule: &Rule) -> Option<Range> {
        let (range_start, range_end) = range;
        let value = rule.value;
        match rule.comparison {
            Comparison::LessThan => {
                if *range_start < value {
                    Some((*range_start, if *range_end < value - 1 { *range_end } else { value - 1 }))
                } else {
                    None
                }
            },
            Comparison::GreaterThan => {
                if *range_end > value {
                    Some((if *range_start > value + 1 { *range_start } else { value + 1 }, *range_end))
                } else {
                    None
                }
            },
        }
    }

    fn non_match_sub_range(range: &Range, rule: &Rule) -> Option<Range> {
        let (range_start, range_end) = range;
        let value = rule.value;
        match rule.comparison {
            Comparison::LessThan => {
                if *range_end >= value {
                    Some((if *range_start < value { value } else { *range_start }, *range_end))
                } else {
                    None
                }
            },
            Comparison::GreaterThan => {
                if *range_start <= value {
                    Some((*range_start, if *range_end > value { value } else { *range_end }))
                } else {
                    None
                }
            },
        }
    }

    fn extend(&self, other: &Partition) -> Partition {
       let mut new_category_ranges = self.category_ranges.clone();
        for (&category, other_ranges) in &other.category_ranges {
            let entry = new_category_ranges.entry(category).or_insert(Vec::new());
            for &other_range in other_ranges {
                entry.push(other_range);
            }
        }
        Partition { category_ranges: new_category_ranges }
    }
}

fn main() {
    let workflow_set = read_workflow_set();
    let count = workflow_set.count_matching_parts();
    println!("Number of matching parts: {}", count);
}

fn read_workflow_set() -> WorkflowSet {
    let lines = read_to_string("example_input").unwrap();
    let mut workflows: WorkflowSet = WorkflowSet::new();
    for line in lines.lines() {
        if line.trim().is_empty() {
            return workflows;
        }
        let workflow = Workflow::from_string(&line);
        workflows.add_workflow(workflow);
    }
    panic!("No empty line separating workflows and parts");
}
