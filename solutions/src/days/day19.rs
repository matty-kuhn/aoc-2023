use std::{
    collections::HashMap,
    ops::Add,
    str::FromStr,
    sync::{Arc, Mutex},
};
use threadpool::ThreadPool;

use super::Day;

pub struct Day19 {
    input: String,
}

impl Day19 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> (Vec<Part>, HashMap<String, Workflow>) {
        let mut parts = Vec::new();
        let mut workflows = HashMap::new();
        let mut split = self.input.lines();
        while let Some(line) = split.next() {
            if line.is_empty() {
                continue;
            }
            if line.starts_with('{') {
                // part
                let part = line.parse().unwrap();
                parts.push(part);
            } else {
                // workflow
                let workflow: Workflow = line.parse().unwrap();
                workflows.insert(workflow.key.clone(), workflow);
            }
        }
        (parts, workflows)
    }
}

impl Day for Day19 {
    fn part1(&self) -> String {
        let start_time = std::time::Instant::now();
        let (parts, workflows) = self.parse_input();
        let mut accepts = Vec::new();
        let mut accept_sum = 0;
        for part in parts {
            let mut action = workflows.get("in").unwrap().apply(&part);

            loop {
                match action {
                    Action::Accept => {
                        accept_sum += part.sum();
                        accepts.push(part);
                        break;
                    }
                    Action::Reject => break,
                    Action::Workflow(key) => {
                        action = workflows.get(&key).unwrap().apply(&part);
                    }
                }
            }
        }
        println!("part 1 took: {:?}", start_time.elapsed());
        format!("{accept_sum}")
    }
    fn part2(&self) -> String {
        // let start_time = std::time::Instant::now();
        // let (parts, workflows) = self.parse_input();
        // let workflow_arc = Arc::new(workflows);
        // let num_runners = 32;
        // let threadpool = ThreadPool::new(num_runners);
        // let (tx, rx) = std::sync::mpsc::channel();
        // let bound_starts = vec![
        //     1, 126, 251, 376, 501, 626, 751, 876, 1001, 1126, 1251, 1376, 1501, 1626, 1751, 1876,
        //     2001, 2126, 2251, 2376, 2501, 2626, 2751, 2876, 3001, 3126, 3251, 3376, 3501, 3626,
        //     3751, 3876,
        // ];
        // let bound_ends = vec![
        //     125, 250, 375, 500, 625, 750, 875, 1000, 1125, 1250, 1375, 1500, 1625, 1750, 1875,
        //     2000, 2125, 2250, 2375, 2500, 2625, 2750, 2875, 3000, 3125, 3250, 3375, 3500, 3625,
        //     3750, 3875, 4000,
        // ];
        // for (start, end) in bound_starts.iter().zip(bound_ends.iter()) {
        //     let workflows = workflow_arc.clone();
        //     let tx_clone = tx.clone();
        //     let start_clone = start.clone();
        //     let end_clone = end.clone();
        //     threadpool.execute(move || {
        //         do_workflows(workflows, start_clone, end_clone, tx_clone);
        //     });
        // }
        // println!("waiting for threads");
        // let sum = rx.iter().take(num_runners).fold(0, |acc, x| acc + x);
        // println!("part 1 took: {:?}", start_time.elapsed());
        // format!("{sum}")
        let (parts, workflows) = self.parse_input();
        let ranges = vec![
            CriteriaRange::X((1, 4000)),
            CriteriaRange::M((1, 4000)),
            CriteriaRange::A((1, 4000)),
            CriteriaRange::S((1, 4000)),
        ];
        for range in ranges {
            let mut action = workflows.get("in").unwrap().apply_to_range(&range);
            todo!()

            // loop {
            //     match action {
            //         Action::Accept => {
            //             accept_sum += part.sum();
            //             accepts.push(part);
            //             break;
            //         }
            //         Action::Reject => break,
            //         Action::Workflow(key) => {
            //             action = workflows.get(&key).unwrap().apply(&part);
            //         }
            //     }
            // }
        }

        format!("todo")
    }
}

fn do_workflows(
    workflows: Arc<HashMap<String, Workflow>>,
    range_start: usize,
    range_end: usize,
    tx: std::sync::mpsc::Sender<usize>,
) {
    let mut sum = 0;
    for x in range_start..=range_end {
        for m in 1..=4000 {
            let x_start_time = std::time::Instant::now();
            for a in 1..=4000 {
                for s in 1..=4000 {
                    let part = Part {
                        x: Criteria::X(x),
                        m: Criteria::M(m),
                        a: Criteria::A(a),
                        s: Criteria::S(s),
                    };
                    let mut action = workflows.get("in").unwrap().apply(&part);
                    loop {
                        match action {
                            Action::Accept => {
                                sum += 1;
                                break;
                            }
                            Action::Reject => break,
                            Action::Workflow(key) => {
                                action = workflows.get(&key).unwrap().apply(&part);
                            }
                        }
                    }
                }
            }
            println!("x took: {:?}", x_start_time.elapsed(),);
        }
    }
    tx.send(sum).unwrap();
}

#[derive(Debug, Clone)]
enum Action {
    Accept,
    Reject,
    Workflow(String),
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Action::Accept),
            "R" => Ok(Action::Reject),
            _ => Ok(Action::Workflow(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
enum Filter {
    GreaterThan { cutoff: Criteria, action: Action },
    LessThan { cutoff: Criteria, action: Action },
    Bare(Action),
}

impl Filter {
    /// returns a tuple of an action to apply to a new range, and a range of unused numbers
    fn apply_to_range(
        &self,
        check: CriteriaRange,
    ) -> (Option<(Action, CriteriaRange)>, Vec<CriteriaRange>) {
        match self {
            Filter::GreaterThan { cutoff, action } => match cutoff {
                Criteria::X(v) => match check {
                    CriteriaRange::X((start, end)) => {
                        if start > *v {
                            // whole range is above cutoff
                            (
                                Some((action.clone(), CriteriaRange::X((start, end)))),
                                vec![],
                            )
                        } else if end < *v {
                            // whole range is below cutoff
                            (None, vec![CriteriaRange::X((start, end))])
                        } else {
                            // need new range
                            (
                                Some((action.clone(), CriteriaRange::X((*v + 1, end)))),
                                vec![CriteriaRange::X((start, *v - 1))],
                            )
                        }
                    }
                    _ => (None, vec![check]),
                },
                Criteria::M(v) => match check {
                    CriteriaRange::M((start, end)) => {
                        if start > *v {
                            // whole range is above cutoff
                            (
                                Some((action.clone(), CriteriaRange::M((start, end)))),
                                vec![],
                            )
                        } else if end < *v {
                            // whole range is below cutoff
                            (None, vec![CriteriaRange::M((start, end))])
                        } else {
                            // need new range
                            (
                                Some((action.clone(), CriteriaRange::M((*v + 1, end)))),
                                vec![CriteriaRange::M((start, *v - 1))],
                            )
                        }
                    }
                    _ => (None, vec![check]),
                },
                Criteria::A(v) => match check {
                    CriteriaRange::A((start, end)) => {
                        if start > *v {
                            // whole range is above cutoff
                            (
                                Some((action.clone(), CriteriaRange::A((start, end)))),
                                vec![],
                            )
                        } else if end < *v {
                            // whole range is below cutoff
                            (None, vec![CriteriaRange::A((start, end))])
                        } else {
                            // need new range
                            (
                                Some((action.clone(), CriteriaRange::A((*v + 1, end)))),
                                vec![CriteriaRange::A((start, *v - 1))],
                            )
                        }
                    }
                    _ => (None, vec![check]),
                },
                Criteria::S(v) => match check {
                    CriteriaRange::S((start, end)) => {
                        if start > *v {
                            // whole range is above cutoff
                            (
                                Some((action.clone(), CriteriaRange::S((start, end)))),
                                vec![],
                            )
                        } else if end < *v {
                            // whole range is below cutoff
                            (None, vec![CriteriaRange::S((start, end))])
                        } else {
                            // need new range
                            (
                                Some((action.clone(), CriteriaRange::S((*v + 1, end)))),
                                vec![CriteriaRange::S((start, *v - 1))],
                            )
                        }
                    }
                    _ => (None, vec![check]),
                },
            },
            Filter::LessThan { cutoff, action } => match cutoff {
                Criteria::X(v) => match check {
                    CriteriaRange::X((start, end)) => {
                        if start > *v {
                            // whole range is above cutoff
                            (None, vec![CriteriaRange::X((start, end))])
                        } else if end < *v {
                            // whole range is below cutoff
                            (
                                Some((action.clone(), CriteriaRange::X((start, end)))),
                                vec![],
                            )
                        } else {
                            // need new range
                            (
                                Some((action.clone(), CriteriaRange::X((start, *v - 1)))),
                                vec![CriteriaRange::X((*v + 1, end))],
                            )
                        }
                    }
                    _ => (None, vec![check]),
                },
                Criteria::M(v) => match check {
                    CriteriaRange::M((start, end)) => {
                        if start > *v {
                            // whole range is above cutoff
                            (None, vec![CriteriaRange::M((start, end))])
                        } else if end < *v {
                            // whole range is below cutoff
                            (
                                Some((action.clone(), CriteriaRange::M((start, end)))),
                                vec![],
                            )
                        } else {
                            // need new range
                            (
                                Some((action.clone(), CriteriaRange::M((start, *v - 1)))),
                                vec![CriteriaRange::M((*v + 1, end))],
                            )
                        }
                    }
                    _ => (None, vec![check]),
                },
                Criteria::A(v) => match check {
                    CriteriaRange::A((start, end)) => {
                        if start > *v {
                            // whole range is above cutoff
                            (None, vec![CriteriaRange::A((start, end))])
                        } else if end < *v {
                            // whole range is below cutoff
                            (
                                Some((action.clone(), CriteriaRange::A((start, end)))),
                                vec![],
                            )
                        } else {
                            // need new range
                            (
                                Some((action.clone(), CriteriaRange::A((start, *v - 1)))),
                                vec![CriteriaRange::A((*v + 1, end))],
                            )
                        }
                    }
                    _ => (None, vec![check]),
                },
                Criteria::S(v) => match check {
                    CriteriaRange::S((start, end)) => {
                        if start > *v {
                            // whole range is above cutoff
                            (None, vec![CriteriaRange::S((start, end))])
                        } else if end < *v {
                            // whole range is below cutoff
                            (
                                Some((action.clone(), CriteriaRange::S((start, end)))),
                                vec![],
                            )
                        } else {
                            // need new range
                            (
                                Some((action.clone(), CriteriaRange::S((start, *v - 1)))),
                                vec![CriteriaRange::S((*v + 1, end))],
                            )
                        }
                    }
                    _ => (None, vec![check]),
                },
            },
            Filter::Bare(a) => (Some((a.clone(), check)), vec![]),
        }
    }

    fn apply(&self, check: Criteria) -> Option<Action> {
        match self {
            Filter::GreaterThan { cutoff, action } => match cutoff {
                Criteria::X(v) => match check {
                    Criteria::X(c) => {
                        if c > *v {
                            Some(action.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                Criteria::M(v) => match check {
                    Criteria::M(c) => {
                        if c > *v {
                            Some(action.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                Criteria::A(v) => match check {
                    Criteria::A(c) => {
                        if c > *v {
                            Some(action.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                Criteria::S(v) => match check {
                    Criteria::S(c) => {
                        if c > *v {
                            Some(action.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
            },
            Filter::LessThan { cutoff, action } => match cutoff {
                Criteria::X(v) => match check {
                    Criteria::X(c) => {
                        if c < *v {
                            Some(action.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                Criteria::M(v) => match check {
                    Criteria::M(c) => {
                        if c < *v {
                            Some(action.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                Criteria::A(v) => match check {
                    Criteria::A(c) => {
                        if c < *v {
                            Some(action.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                Criteria::S(v) => match check {
                    Criteria::S(c) => {
                        if c < *v {
                            Some(action.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
            },
            Filter::Bare(action) => Some(action.clone()),
        }
    }
}

impl FromStr for Filter {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('>') {
            // greater than
            let mut split = s.split(':');
            let cutoff = split.next().unwrap().parse().unwrap();
            let action = split.next().unwrap().parse().unwrap();
            Ok(Filter::GreaterThan { cutoff, action })
        } else if s.contains('<') {
            // less than
            let mut split = s.split(':');
            let cutoff = split.next().unwrap().parse().unwrap();
            let action = split.next().unwrap().parse().unwrap();
            Ok(Filter::LessThan { cutoff, action })
        } else {
            Ok(Filter::Bare(s.parse().unwrap()))
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    key: String,
    filters: Vec<Filter>,
}

impl Workflow {
    /// returns a tuple of an action to apply to a new range, and a range of unused numbers
    fn apply_to_range(&self, check: &CriteriaRange) -> Vec<(Action, CriteriaRange)> {
        let mut range_actions = Vec::new();
        let mut ranges_to_check = vec![check.clone()];
        for filter in &self.filters {
            let mut next_ranges = Vec::new();
            for curr_check in ranges_to_check.clone() {
                let (action, mut new_ranges) = filter.apply_to_range(curr_check);
                if let Some((action, apply_range)) = action {
                    range_actions.push((action, apply_range));
                }
                next_ranges.append(&mut new_ranges);
            }
            ranges_to_check = next_ranges;
        }
        range_actions
    }

    fn apply(&self, part: &Part) -> Action {
        for filter in &self.filters {
            if let Some(action) = filter.apply(part.x.clone()) {
                return action;
            }
            if let Some(action) = filter.apply(part.m.clone()) {
                return action;
            }
            if let Some(action) = filter.apply(part.a.clone()) {
                return action;
            }
            if let Some(action) = filter.apply(part.s.clone()) {
                return action;
            }
        }
        panic!("no action found")
    }
}

impl FromStr for Workflow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // format is key{criteria>num:action, ,,,action}
        let mut split = s.split('{');
        let key = split.next().unwrap();
        let no_brackets = split.next().unwrap().trim_matches('}');
        let mut filters = Vec::new();
        let filter_split = no_brackets.split(',');
        for filter in filter_split {
            filters.push(filter.parse().unwrap());
        }

        Ok(Self {
            key: key.to_string(),
            filters,
        })
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: Criteria,
    m: Criteria,
    a: Criteria,
    s: Criteria,
}

impl Part {
    fn sum(&self) -> usize {
        (self.x + self.m) + (self.a + self.s)
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // format is {x=num,m=num,a=num,s=num}
        let no_brackets = s.trim_matches('{').trim_matches('}');
        let mut split = no_brackets.split(',');
        let x = split.next().unwrap().parse().unwrap();
        let m = split.next().unwrap().parse().unwrap();
        let a = split.next().unwrap().parse().unwrap();
        let s = split.next().unwrap().parse().unwrap();
        Ok(Part { x, m, a, s })
    }
}

#[derive(Debug, Clone, Copy)]
enum Criteria {
    X(usize),
    M(usize),
    A(usize),
    S(usize),
}

impl Add for Criteria {
    type Output = usize;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = match self {
            Criteria::X(v) => v,
            Criteria::M(v) => v,
            Criteria::A(v) => v,
            Criteria::S(v) => v,
        };
        let rhs = match rhs {
            Criteria::X(v) => v,
            Criteria::M(v) => v,
            Criteria::A(v) => v,
            Criteria::S(v) => v,
        };
        lhs + rhs
    }
}

impl FromStr for Criteria {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = if s.contains('>') {
            s.split('>')
        } else if s.contains('<') {
            s.split('<')
        } else {
            s.split('=')
        };
        let key = split.next().unwrap();
        let value = split.next().unwrap().parse().unwrap();
        match key {
            "x" => Ok(Criteria::X(value)),
            "m" => Ok(Criteria::M(value)),
            "a" => Ok(Criteria::A(value)),
            "s" => Ok(Criteria::S(value)),
            _ => Err(()),
        }
    }
}

/// lists of ranges for each criteria, denoted by starts and ends
#[derive(Debug, Clone)]
enum CriteriaRange {
    X((usize, usize)),
    M((usize, usize)),
    A((usize, usize)),
    S((usize, usize)),
}
