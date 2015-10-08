fn main() {
    let value = std::env::args().nth(1);
    let convert_and_double = compose(convert, double);

    match convert_and_double(value) {
        Some(n) => println!("{}", n),
        None => println!("No value"),
    }

}

fn convert(n: Option<String>) -> Option<f32> {
    n.and_then(|n| n.parse().ok())
}

fn double(n: Option<f32>) -> Option<f32> {
    n.map(|n| n * 2.0)
}

fn compose<'f, T1, T2, T3, F1, F2>(a: F1, b: F2) -> Box<Fn(T1) -> T3 + 'f>
    where   F1:Fn(T1) -> T2 + 'f,
            F2:Fn(T2) -> T3 + 'f
{
    Box::new(move |input| b(a(input)))
}