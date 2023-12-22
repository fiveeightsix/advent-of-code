use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1,anychar,char,one_of,u32},
    combinator::{map_res,value,verify},
    error::{Error,ParseError},
    multi::separated_list1,
    IResult,
    Parser,
    sequence::{delimited,pair,preceded,separated_pair,tuple}
};

fn main() {
    let input = include_str!("input.txt");

    println!("{}", part1(&input));
}

fn part1(input: &str) -> usize {
    0
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
enum Rule {
    If(Condition, Destination),
    Else(Destination)
}

#[derive(Debug, PartialEq)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
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

    Ok((remaining, Part { x, m, a, s}))
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
}

