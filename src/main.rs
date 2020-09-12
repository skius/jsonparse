use jsonparse::{example, Parser};

fn main() {
    example();

    let p = Parser::new("[1, 2]");
    println!("{}", p.parse().unwrap()[0])
}
