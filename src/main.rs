// mod print;
// mod vars;
// mod types;
// mod strings;
// mod tuples;
// mod arrays;
mod vectors;

use std::mem;

fn main() {
    // print::run();
    // vars::run();
    // types::run();
    // strings::run();
    // tuples::run();
    // arrays::run();
    vectors::run();
    let str1 = "赵东";
    println!("{}", mem::size_of_val(str1));
}
