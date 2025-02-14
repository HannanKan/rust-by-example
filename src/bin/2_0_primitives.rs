fn main() {
    // 变量可以给出类型说明。
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 常规说明
    let an_integer   = 5i32; // 后缀说明

    // 否则会按默认方式决定类型。
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // 类型也可根据上下文自动推断。
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64 类型
    println!("initial inferred_type: {} \n", inferred_type);
    inferred_type = 4294967296i64;

    // 可变的（mutable）变量，其值可以改变。
    let mut mutable = 12; // Mutable `i32`
    println!("initial mutable: {} \n", mutable);
    mutable = 21;

    println!("logical: {} \na_float: {} \nan_integer: {}, \ndefault_float: {} \ndefault_integer: {} \ninferred_type: {} \nmutable: {} \n",
            logical, a_float, an_integer, default_float, default_integer, inferred_type, mutable);

    // 报错！变量的类型并不能改变。
    // mutable = true;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let mutable = true;
    println!("variable shadow, mutable: {}", mutable)
}
