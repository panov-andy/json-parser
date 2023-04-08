fn main() {
    let preved = start_with("preved");

    let result = preved.parse("preved medved");
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
