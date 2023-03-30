fn main() {
    let preved = begin("preved");

    let result = preved.parse("preved medved");
    println!("{:?}", result);
}

type IResult<IN, OUT> = Result<(IN, OUT), String>;

trait Parser<IN, OUT> {
    fn parse(&self, input: IN) -> IResult<IN, OUT>;
}

fn begin(with: &str) -> impl Parser<&str, &str> {
    move |input: &str| {
        if input.starts_with(&with) {
            return Ok((&input[..with.len()], &input[with.len()..]));
        }
        return Err("doesn't start with".to_string());
    }
}

