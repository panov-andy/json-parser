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

fn start_with<'a:'b, 'b>(with: &'b str) -> impl Parser<&'b str, &'b str> + '_ {
    move |input: &'b str| {
        if input.starts_with(with) {
            Ok(input.split_at(with.len()))
        } else {
            Err("doesn't start with".to_string())
        }
    }
}

fn or<'a:'b,'b>(var1: &'b dyn Parser<&'a str, &'a str>, var2: &'b dyn Parser<&'a str, &'a str>) -> impl Parser<&'a str, &'a str>+'b  {
    move |input: &'a str| {
        let result = var1.parse(input);
        if result.is_ok() {
            return result;
        }
        var2.parse(input)
    }
}
