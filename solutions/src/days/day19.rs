use std::{
    collections::{HashMap, VecDeque},
    ops::Add,
    str::FromStr,
    sync::{Arc, Mutex},
};

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
        let (_, workflows) = self.parse_input();
        let range = RangeGroup {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        };
        let mut accept_sum = 0;
        let actions = workflows.get("in").unwrap().apply_to_range(&range);
        let mut actions = VecDeque::from(actions);

        while let Some((action, range)) = actions.pop_front() {
            match action {
                Action::Accept => {
                    // let count = range.count();
                    // dbg!(accept_sum, count);
                    dbg!(&range);
                    accept_sum += range.count();
                }
                Action::Reject => {}
                Action::Workflow(key) => {
                    let mut next_actions = workflows.get(&key).unwrap().apply_to_range(&range);
                    for next_action in next_actions {
                        actions.push_back(next_action);
                    }
                    // panic!()
                }
            }
        }
        format!("{accept_sum}")
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
        check: RangeGroup,
    ) -> (Option<(Action, RangeGroup)>, Option<RangeGroup>) {
        match self {
            Filter::GreaterThan { cutoff, action } => match cutoff {
                Criteria::X(v) => {
                    let (x_range_start, x_range_end) = check.x;
                    if x_range_start > *v {
                        // whole range is above cutoff
                        (Some((action.clone(), check)), None)
                    } else if x_range_end < *v {
                        // whole range is below cutoff
                        (None, Some(check))
                    } else {
                        // new range time
                        (
                            Some((
                                action.clone(),
                                RangeGroup {
                                    x: (*v + 1, x_range_end),
                                    m: check.m,
                                    a: check.a,
                                    s: check.s,
                                },
                            )),
                            Some(RangeGroup {
                                x: (x_range_start, *v),
                                m: check.m,
                                a: check.a,
                                s: check.a,
                            }),
                        )
                    }
                }
                Criteria::M(v) => {
                    let (m_range_start, m_range_end) = check.m;
                    if m_range_start > *v {
                        // whole range is above cutoff
                        (Some((action.clone(), check)), None)
                    } else if m_range_end < *v {
                        // whole range is below cutoff
                        (None, Some(check))
                    } else {
                        // new range time
                        (
                            Some((
                                action.clone(),
                                RangeGroup {
                                    m: (*v + 1, m_range_end),
                                    x: check.x,
                                    a: check.a,
                                    s: check.s,
                                },
                            )),
                            Some(RangeGroup {
                                x: check.x,
                                m: (m_range_start, *v),
                                a: check.a,
                                s: check.s,
                            }),
                        )
                    }
                }
                Criteria::A(v) => {
                    let (a_range_start, a_range_end) = check.a;
                    if a_range_start > *v {
                        // whole range above cutoff
                        (Some((action.clone(), check)), None)
                    } else if a_range_end < *v {
                        // whole range below cutoff
                        (None, Some(check))
                    } else {
                        (
                            Some((
                                action.clone(),
                                RangeGroup {
                                    x: check.x,
                                    m: check.m,
                                    a: (*v + 1, a_range_end),
                                    s: check.s,
                                },
                            )),
                            Some(RangeGroup {
                                x: check.x,
                                m: check.m,
                                a: (a_range_start, *v),
                                s: check.s,
                            }),
                        )
                    }
                }
                Criteria::S(v) => {
                    let (s_range_start, s_range_end) = check.s;
                    if s_range_start > *v {
                        (Some((action.clone(), check)), None)
                    } else if s_range_end < *v {
                        (None, Some(check))
                    } else {
                        (
                            Some((
                                action.clone(),
                                RangeGroup {
                                    x: check.x,
                                    m: check.m,
                                    a: check.a,
                                    s: (*v + 1, s_range_end),
                                },
                            )),
                            Some(RangeGroup {
                                x: check.x,
                                m: check.m,
                                a: check.a,
                                s: (s_range_start, *v),
                            }),
                        )
                    }
                }
            },
            Filter::LessThan { cutoff, action } => match cutoff {
                Criteria::X(v) => {
                    let (x_range_start, x_range_end) = check.x;
                    if x_range_start > *v {
                        // whole range is above cutoff
                        (None, Some(check))
                    } else if x_range_end < *v {
                        // whole range is below cutoff
                        (Some((action.clone(), check)), None)
                    } else {
                        // new range time
                        (
                            Some((
                                action.clone(),
                                RangeGroup {
                                    x: (x_range_start, *v - 1),
                                    m: check.m,
                                    a: check.a,
                                    s: check.s,
                                },
                            )),
                            Some(RangeGroup {
                                x: (*v, x_range_end),
                                m: check.m,
                                a: check.a,
                                s: check.s,
                            }),
                        )
                    }
                }
                Criteria::M(v) => {
                    let (m_range_start, m_range_end) = check.m;
                    if m_range_start > *v {
                        // whole range is above cutoff
                        (None, Some(check))
                    } else if m_range_end < *v {
                        // whole range is below cutoff
                        (Some((action.clone(), check)), None)
                    } else {
                        // new range time
                        (
                            Some((
                                action.clone(),
                                RangeGroup {
                                    x: check.x,
                                    m: (m_range_start, *v - 1),
                                    a: check.a,
                                    s: check.s,
                                },
                            )),
                            Some(RangeGroup {
                                x: check.x,
                                m: (*v, m_range_end),
                                a: check.a,
                                s: check.s,
                            }),
                        )
                    }
                }
                Criteria::A(v) => {
                    let (a_range_start, a_range_end) = check.a;
                    if a_range_start > *v {
                        // whole range is above cutoff
                        (None, Some(check))
                    } else if a_range_end < *v {
                        // whole range is below cutoff
                        (Some((action.clone(), check)), None)
                    } else {
                        // new range time
                        (
                            Some((
                                action.clone(),
                                RangeGroup {
                                    x: check.x,
                                    m: check.m,
                                    a: (a_range_start, *v - 1),
                                    s: check.s,
                                },
                            )),
                            Some(RangeGroup {
                                x: check.x,
                                m: check.m,
                                a: (*v, a_range_end),
                                s: check.s,
                            }),
                        )
                    }
                }
                Criteria::S(v) => {
                    let (s_range_start, s_range_end) = check.s;
                    if s_range_start > *v {
                        // whole range is above cutoff
                        (None, Some(check))
                    } else if s_range_end < *v {
                        // whole range is below cutoff
                        (Some((action.clone(), check)), None)
                    } else {
                        // new range time
                        (
                            Some((
                                action.clone(),
                                RangeGroup {
                                    x: check.x,
                                    m: check.m,
                                    a: check.a,
                                    s: (s_range_start, *v - 1),
                                },
                            )),
                            Some(RangeGroup {
                                x: check.x,
                                m: check.m,
                                a: check.a,
                                s: (*v, s_range_end),
                            }),
                        )
                    }
                }
                _ => todo!(),
            },
            Filter::Bare(a) => (Some((a.clone(), check)), None),
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
    fn apply_to_range(&self, check: &RangeGroup) -> Vec<(Action, RangeGroup)> {
        let mut range_actions = Vec::new();
        let mut ranges_to_check = vec![check.clone()];
        for filter in &self.filters {
            let mut next_ranges = Vec::new();
            for curr_check in ranges_to_check.clone() {
                let (action, new_ranges) = filter.apply_to_range(curr_check);
                if let Some((action, apply_range)) = action {
                    range_actions.push((action, apply_range));
                }
                if let Some(new_range) = new_ranges {
                    next_ranges.push(new_range);
                };
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

#[derive(Debug, Clone)]
struct RangeGroup {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl RangeGroup {
    fn count(&self) -> usize {
        let x_count = self.x.1 - self.x.0;
        let m_count = self.m.1 - self.m.0;
        let a_count = self.a.1 - self.a.0;
        let s_count = self.s.1 - self.s.0;
        x_count * m_count * a_count * s_count
    }
}
