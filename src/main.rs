fn main() {
    let preved = start_with("preved");
    let medved = start_with("medved");

    let orr = or(&preved, &medved);

    let result = orr.parse("preved medved");
    println!("{:?}", result);
}

type IResult<In, Out> = Result<(In, Out), String>;

trait Parser<In, Out> {
    fn parse(&self, input: In) -> IResult<In, Out>;
}

impl<In, Out, F> Parser<In, Out> for F
    where
        F: Fn(In) -> IResult<In, Out>,
{
    fn parse(&self, input: In) -> IResult<In, Out> {
        self(input)
    }
}

fn start_with<'a>(with: &str) -> impl Parser<&'a str, &'a str> + '_ {
    move |input: &'a str| {
        if input.starts_with(with) {
            Ok(input.split_at(with.len()))
        } else {
            Err("doesn't start with".to_string())
        }
    }
}

fn or<'a>(var1: &'a dyn Parser<&'a str, &'a str>, var2: &'a dyn Parser<&'a str, &'a str>) -> impl Parser<&'a str, &'a str>  {
    move |input: &'a str| {
        let result = var1.parse(input);
        if result.is_ok() {
            return result;
        }
        var2.parse(input)
    }
}
