fn main() {
    let preved =  start_with("preved");
    let medved =  start_with("medved");

    let any = any_parser(vec![preved, medved]);

    let result = any.parse("preved medved");
    println!("{:?}", result);
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

fn any_parser<In: Clone, Out>(parsers: Vec<dyn Parser<In, Out>>) -> impl Parser<In, Out> {
    move |input: In| {
        for parser in parsers {
            let res = parser.parse(input.clone());
            if res.is_ok() {
                return res;
            }
        }
        Err("no parsers matched")
    }
}

type IResult<In, Out> = Result<(In, Out), String>;

trait Parser<In, Out> {
    fn parse(&self, input: In) -> IResult<In, Out>;
    fn or<P>(self, or_parser: P) -> Or<Self, P>
        where
            P: Parser<In, Out>,
            Self: Sized, //todo unclear why it's required here
    {
        return Or { first: self, second: or_parser };
    }
}

impl<In, Out, F> Parser<In, Out> for F
    where
        F: Fn(In) -> IResult<In, Out>,
{
    fn parse(&self, input: In) -> IResult<In, Out> {
        self(input)
    }
}

struct Or<Parser1, Parser2> {
    first: Parser1,
    second: Parser2,
}

//todo no way to avoid Clone? worth to check if &str really clones
impl<In: Clone, Out, ParserType> Parser<In, Out> for Or<ParserType, ParserType>
    where ParserType: Parser<In, Out>
{
    fn parse(&self, input: In) -> IResult<In, Out> {
        match self.first.parse(input.clone()) {
            Err(_) => self.second.parse(input),
            res => res,
        }
    }
}