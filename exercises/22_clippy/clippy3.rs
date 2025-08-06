// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    // 1. 用 unwrap() 之前没有先确认 Option 是 Some，会 panic。且 if my_option.is_none() 时，调用 unwrap() 是错误的。
    // Clippy 建议用 match 或 if let 结构。
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 这里 unwrap() 会 panic，改成打印 None 即可
        println!("{:?}", my_option);
    }

    // 2. 数组定义语法错误，缺少逗号分隔元素
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {my_arr:?}");

    // 3. vec![].resize() 返回的是 (), 不能赋值给变量
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    // 4. 交换两个变量的值，Clippy 建议用 std::mem::swap
    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
