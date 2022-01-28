use marin::Marin;

fn main() {
    let input = "123 arg1 list: [1, 2, [\"nested\", \"list\"]] -flag ..10 -5..5 \"quoted strings\"";
    println!("input: {}", input);
    let res = Marin::parse(input).unwrap();
    println!("output:\n{}", res);
}
