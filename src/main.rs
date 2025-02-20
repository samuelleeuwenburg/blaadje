mod blaadje;

use blaadje::{parse, run_program};

fn main() {
    let code = "
        (let x 10)
        (+ 2 x)
    ";

    let program = parse(code).unwrap();
    let result = run_program(&program).unwrap();

    println!("code: \t{:?}", code);
    println!("program: \t\t{:?}", program);
    println!("result: \t{:?}", result);
}
