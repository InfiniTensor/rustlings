// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    // 1. 安全处理 Option，不再调用 unwrap()
    let my_option: Option<()> = None;
    if let Some(v) = my_option {
        println!("Got value: {:?}", v);
    } else {
        println!("Got None!");
    }

    // 2. 数组语法已正确，无需变动
    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    // 3. 清空 Vec，不需要捕获返回值
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // .resize() 返回 ()，不赋值给变量
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // 4. 使用 std::mem::swap
    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
