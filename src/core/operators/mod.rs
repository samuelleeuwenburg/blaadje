mod conditional;
mod equality;
mod lambda;
mod list;
mod math;
mod variables;

pub use conditional::process_if;
pub use equality::{process_equal, process_greater_than, process_less_than};
pub use lambda::{process_lambda, process_lambda_call};
pub use list::{process_do, process_head, process_list, process_tail};
pub use math::{process_add, process_subtract};
pub use variables::process_let;
