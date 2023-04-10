fn main() {
    let preved = start_with("preved");
    let medved = start_with("medved");

    let ab = start_with("ab");

    let orr = or(&preved, &medved);

    let ab_or_preved = or(&ab, &orr);

    let result = orr.parse("preved medved");
    println!("{:?}", result);

    let result = ab_or_preved.parse("preved ab");
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

fn start_with<'init, 'src>(with: &'init str) -> impl Parser<&'src str, &'src str> + 'init {
    move |input: &'src str| {
        if input.starts_with(with) {
            Ok(input.split_at(with.len()))
        } else {
            Err("doesn't start with".to_string())
        }
    }
}

fn or<'fun, 'src>(
    var1: &'fun dyn Parser<&'src str, &'src str>,
    var2: &'fun dyn Parser<&'src str, &'src str>,
) -> impl Parser<&'src str, &'src str> + 'fun {
    move |input: &'src str| {
        let result = var1.parse(input);
        if result.is_ok() {
            return result;
        }
        var2.parse(input)
    }
}
