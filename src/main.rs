mod blaadje;

use blaadje::{parse, run_program};

fn main() {
    let code = "
        (+ 1 2)
    ";

    let program = parse(code).unwrap();
    let result = run_program(&program).unwrap();

    println!("code: \t\t{:?}", code.trim());
    println!("program: \t{:?}", program);
    println!("result: \t{:?}", result);
}
