use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part2(&input));
}

fn part2(input: &str) -> u32 {
    let (_, workflows) = parse::input(&input).expect("Could not parse input");

    let workflow_map = WorkflowMap::from(workflows);

    0
}


struct WorkflowMap {
    map: HashMap<String, Workflow>
}

impl WorkflowMap {
    pub fn from(workflows: Vec<Workflow>) -> Self {
        let map: HashMap<String, Workflow> = workflows
            .into_iter()
            .map(|workflow| (workflow.id.clone(), workflow))
            .collect();

        assert!(map.contains_key("in"), "Does not contain workflow `in`");

        Self { map }
    }

    pub fn get_start(&self) -> &Workflow {
        self.map.get("in").unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Destination {
    Workflow(String),
    Accepted,
    Rejected
}

#[derive(Debug, PartialEq)]
struct Condition {
    category: char,
    operator: char,
    value: u32,
}

impl Condition {
    pub fn new(category: char, operator: char, value: u32) -> Self {
        Self { category, operator, value }
    }
}

#[derive(Debug, PartialEq)]
struct Rule {
    condition: Condition,
    destination: Destination,
}

impl Rule {
    pub fn new(condition: Condition, destination: Destination) -> Self {
        Self { condition, destination }
    }
}

#[derive(Debug, PartialEq)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
    otherwise: Destination,
}

#[derive(Debug, PartialEq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    pub fn new(x: u32, m: u32, a: u32, s: u32) -> Self {
        Self { x, m, a, s }
    }

    pub fn get_category(&self, c: &char) -> u32 {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Noooooo")
        }
    }
}

mod parse {
    use nom::{
        branch::alt,
        character::complete::{alpha1,char,newline,one_of,u32},
        combinator::{map,value},
        multi::separated_list1,
        IResult,
        Parser,
        sequence::{delimited,separated_pair,terminated,tuple}
    };
    use super::*;

    fn destination(input: &str) -> IResult<&str, Destination> {
        alt((
            value(Destination::Accepted, char('A')),
            value(Destination::Rejected, char('R')),
            alpha1.map(|id: &str| Destination::Workflow(id.to_owned()))
        ))(input)
    }

    fn condition(input: &str) -> IResult<&str, Condition> {
        let (remaining, (category, operator, value)) = tuple((
            one_of("xmas"),
            one_of("<>"),
            u32
        ))(input)?;

        Ok((remaining, Condition { category, operator, value }))
    }

    fn parse_rule(input: &str) -> IResult<&str, Rule> {
        map(
            separated_pair(
                condition,
                char(':'),
                destination
            ),
            |(condition, destination)| Rule { condition, destination }
        )(input)
    }

    fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
        let (remaining, (id, (rules, otherwise))) = tuple((
            alpha1.map(|s: &str| s.to_string()),
            delimited(
                char('{'),
                separated_pair(
                    separated_list1(
                        char(','),
                        parse_rule
                    ),
                    char(','),
                    destination
                ),
                char('}'),
            )
        ))(input)?;

        Ok((remaining, Workflow { id, rules, otherwise }))
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Workflow>> {
        let (remaining, workflows) = terminated(
            separated_list1(
                newline,
                parse_workflow
            ),
            newline.and(newline)
        )(input)?;
        
        Ok((remaining, workflows))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn destination_ok() {
            assert_eq!(destination("A"), Ok(("", Destination::Accepted)));
            assert_eq!(destination("R"), Ok(("", Destination::Rejected)));
            assert_eq!(destination("px"), Ok(("", Destination::Workflow("px".to_string()))));
        }

        #[test]
        fn condition_ok() {
            assert_eq!(condition("a<2006"), Ok(("", Condition::new('a', '<', 2006))));
            assert_eq!(condition("m>2090"), Ok(("", Condition::new('m', '>', 2090))));
        }

        #[test]
        fn workflow_ok() {
            let input = r"px{a<2006:qkq,m>2090:A,rfg}";

            let expected = Workflow {
                id: "px".to_string(),
                rules: vec![
                    Rule::new(Condition::new('a', '<', 2006), Destination::Workflow("qkq".to_string())),
                    Rule::new(Condition::new('m', '>', 2090), Destination::Accepted),
                ],
                otherwise: Destination::Workflow("rfg".to_string())
            };
            
            assert_eq!(parse_workflow(&input), Ok(("", expected)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part2_ok() {
        let input = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

        assert_eq!(19114, part2(&input));
    }
}

