use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1,char,newline,one_of,u32},
    combinator::{map_res,value,verify},
    error::{Error,ParseError},
    multi::separated_list1,
    IResult,
    Parser,
    sequence::{delimited,pair,preceded,separated_pair,tuple}
};
use std::collections::HashMap;
use std::error;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(&input));
}

fn part1(input: &str) -> usize {
    let (_, (workflows, parts)) = parse_input(&input).expect("nuh uh");

    let workflow_map = WorkflowMap::from(workflows);

    let mut total_x = 0;
    let mut total_m = 0;
    let mut total_a = 0;
    let mut total_s = 0;
    
    
    for p in parts.iter() {
        println!("{:?}", p);
    }
    
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

    pub fn is_accepted(&self, part: &Part) -> Result<bool, Box<dyn error::Error>> {
        let mut workflow = self.get_start();

        loop {
            
        }
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

    pub fn test_part(&self, part: &Part) -> bool {
        let part_value = part.get_category(&self.category);

        match self.operator {
            '>' => part_value > self.value,
            '<' => part_value < self.value,
            _ => panic!("Paaants")
        }
    }
}

#[derive(Debug, PartialEq)]
enum Rule {
    If(Condition, Destination),
    Else(Destination)
}

#[derive(Debug, PartialEq)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn route_part(&self, part: &Part) -> Destination {
        // for rule in self.rules.iter() {
        //     match rule {
        //         If(cond, dest) if cond.test_part(part) => {
        //             return dest
        //         },
        //         Else(dest) => return dest
        //     }
        // }
        todo!();
    }
}

fn parse_destination(input: &str) -> IResult<&str, Destination> {
    alt((
        value(Destination::Accepted, char('A')),
        value(Destination::Rejected, char('R')),
        alpha1.map(|id: &str| Destination::Workflow(id.to_owned()))
    ))(input)
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (remaining, (category, operator, value)) = tuple((
        one_of("xmas"),
        one_of("<>"),
        u32
    ))(input)?;

    Ok((remaining, Condition { category, operator, value }))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((
        separated_pair(
            parse_condition,
            char(':'),
            parse_destination
        ).map(|(condition, destination)| Rule::If(condition, destination)),
        parse_destination.map(|destination| Rule::Else(destination))
    ))(input)
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (remaining, (id, rules)) = tuple((
        alpha1.map(|s: &str| s.to_string()),
        delimited(
            char('{'),
            separated_list1(
                char(','),
                parse_rule
            ),
            char('}'),
        )
    ))(input)?;

    Ok((remaining, Workflow { id, rules }))
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

fn create_category_parser<'a>(
    c: char
) -> impl Parser<&'a str, u32, Error<&'a str>> {
    preceded(
        char(c).and(char('=')),
        u32
    )
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (remaining, (x, _1, m, _2, a, _3, s)) = delimited(
        tag("{"),
        tuple((
            create_category_parser('x'),
            tag(","),
            create_category_parser('m'),
            tag(","),
            create_category_parser('a'),
            tag(","),
            create_category_parser('s'),
        )),
        tag("}")
    )(input)?;

    Ok((remaining, Part { x, m, a, s }))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Workflow>, Vec<Part>)> {
    let (remaining, (workflows, parts)) = separated_pair(
        separated_list1(
            newline,
            parse_workflow
        ),
        newline.and(newline),
        separated_list1(
            newline,
            parse_part
        )
    )(input)?;
    
    Ok((remaining, (workflows, parts)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_destination_ok() {
        assert_eq!(parse_destination("A"), Ok(("", Destination::Accepted)));
        assert_eq!(parse_destination("R"), Ok(("", Destination::Rejected)));
        assert_eq!(parse_destination("px"), Ok(("", Destination::Workflow("px".to_string()))));
    }

    #[test]
    fn parse_condition_ok() {
        assert_eq!(parse_condition("a<2006"), Ok(("", Condition::new('a', '<', 2006))));
        assert_eq!(parse_condition("m>2090"), Ok(("", Condition::new('m', '>', 2090))));
    }

    #[test]
    fn parse_workflow_ok() {
        let input = r"px{a<2006:qkq,m>2090:A,rfg}";

        let expected = Workflow {
            id: "px".to_string(),
            rules: vec![
                Rule::If(Condition::new('a', '<', 2006), Destination::Workflow("qkq".to_string())),
                Rule::If(Condition::new('m', '>', 2090), Destination::Accepted),
                Rule::Else(Destination::Workflow("rfg".to_string()))
            ]
        };
        
        assert_eq!(parse_workflow(&input), Ok(("", expected)));
    }
    
    #[test]
    fn create_category_parser_ok() {
        let input = r"x=787";
        let mut parser = create_category_parser('x');
        let result = parser.parse(&input);

        assert_eq!(result, Ok(("", 787)));
    }

    #[test]
    fn parse_part_ok() {
        let expected = Part::new(787, 2655, 1222, 2876);
        
        let input = r"{x=787,m=2655,a=1222,s=2876}";

        let (_, value) = parse_part(&input).expect("oh sh");

        assert_eq!(expected, value);
    }

    #[test]
    fn part1_ok() {
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

        assert_eq!(19114, part1(&input));
    }
}

