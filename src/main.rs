mod blaadje;

use blaadje::{parse, run_program};

fn main() {
    let code = "
        (let foo (lambda (x) (+ x 1)))
        (let bar (lambda (y) (+ y 1)))
        (let x 1)
        (+ 1 2)
    ";

    let program = parse(code).unwrap();
    let result = run_program(&program).unwrap();

    println!("code: \t{:?}", code);
    println!("program: \t\t{:?}", program);
    println!("result: \t{:?}", result);
}
