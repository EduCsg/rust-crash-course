mod arrays;
mod conditionals;
mod functions;
mod loops;
mod pointer_ref;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod variables;
mod vectors;

fn main() {
    print::run();
    variables::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    structs::run();
}
