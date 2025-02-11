mod blaadje;

use blaadje::{eval, parse};

fn main() {
    let prog = "(+ 8 (+ 10 2))";
    let ast = parse(prog);
    let result = eval(&ast);

    println!("program: \t{:?}", prog);
    println!("ast: \t\t{:?}", ast);
    println!("result: \t{:?}", result);
}
