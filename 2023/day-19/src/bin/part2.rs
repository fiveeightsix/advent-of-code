use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part2(&input));
}

fn part2(input: &str) -> u32 {
    let (_, (workflows, parts)) = parse::input(&input).expect("Could not parse input");

    let workflow_map = WorkflowMap::from(workflows);

    let mut total_x = 0;
    let mut total_m = 0;
    let mut total_a = 0;
    let mut total_s = 0;
        
    for part in parts.iter() {
        if workflow_map.is_accepted(part) {
            total_x += part.x;
            total_m += part.m;
            total_a += part.a;
            total_s += part.s;
        }
    }
    
    total_x + total_m + total_a + total_s
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

    pub fn is_accepted(&self, part: &Part) -> bool {
        let mut workflow = self.get_start();

        loop {
            let next_destination = workflow.route_part(part);

            match next_destination {
                Destination::Workflow(w_id) => {
                    workflow = self.map.get(&w_id).expect("Workflow not found");
                },
                Destination::Accepted => return true,
                Destination::Rejected => return false
            }
        };
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

    pub fn is_true_for(&self, part: &Part) -> bool {
        let part_value = part.get_category(&self.category);

        match self.operator {
            '>' => part_value > self.value,
            '<' => part_value < self.value,
            _ => panic!("Paaants")
        }
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

impl Workflow {
    pub fn route_part(&self, part: &Part) -> Destination {
        for rule in self.rules.iter() {
            if rule.condition.is_true_for(part) {
                return rule.destination.clone()
            }
        }
        
        self.otherwise.clone()
    }
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
        bytes::complete::tag,
        character::complete::{alpha1,char,newline,one_of,u32},
        combinator::{map,value},
        error::Error,
        multi::separated_list1,
        IResult,
        Parser,
        sequence::{delimited,preceded,separated_pair,tuple}
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

    fn create_category_parser<'a>(
        c: char
    ) -> impl Parser<&'a str, u32, Error<&'a str>> {
        preceded(
            char(c).and(char('=')),
            u32
        )
    }

    fn part(input: &str) -> IResult<&str, Part> {
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

    pub fn input(input: &str) -> IResult<&str, (Vec<Workflow>, Vec<Part>)> {
        let (remaining, (workflows, parts)) = separated_pair(
            separated_list1(
                newline,
                parse_workflow
            ),
            newline.and(newline),
            separated_list1(
                newline,
                part
            )
        )(input)?;
        
        Ok((remaining, (workflows, parts)))
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
        
        #[test]
        fn create_category_parser_ok() {
            let input = r"x=787";
            let mut parser = create_category_parser('x');
            let result = parser.parse(&input);

            assert_eq!(result, Ok(("", 787)));
        }

        #[test]
        fn part_ok() {
            let expected = Part::new(787, 2655, 1222, 2876);
            
            let input = r"{x=787,m=2655,a=1222,s=2876}";

            let (_, value) = part(&input).expect("oh sh");

            assert_eq!(expected, value);
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

