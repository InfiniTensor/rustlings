// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    // let my_option: Option<&str> = None;
    let my_option: Option<()> = None;
    // Assume that you don't know the value of `my_option`.
    // In the case of `Some`, we want to print its value.
    // 安全地处理 Option
    if let Some(val) = my_option {
        println!("{}", val);
    } else {
        println!("Got None!");
    }
    // if my_option.is_none() {
    //     println!("{}", my_option.unwrap());
    // }

    let my_arr = &[
        -1, -2, -3, // 不能没有逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(0, 5); // 或 my_empty_vec.clear();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    // value_a = value_b;
    // value_b = value_a;
    println!("value a: {value_a}; value b: {value_b}");
}