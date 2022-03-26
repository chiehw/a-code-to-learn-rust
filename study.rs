/* 一份代码快速了解Rust */

// PS：可能会有很多 warning，暂时可以不考虑。
// 编译运行：clear && rustc study.rs  && ./study
// 食用指南：从上到下读懂代码即可。使用 todo 标注的部分，都需要你把注释删掉，尝试自行编译运行。

fn main() {
    /*||||||||||||||||||||||||||||| 变量 |||||||||||||||||||||||||||||||||||||| */
    // let 表示声明变量。
    let x;
    x = 42;

    // 还可以使用冒号来显示的指定变量的类型，即类型标注（Type Annotation）
    // 相同的变量名是可以重复声明的。重复声明后，上一个变量就不存在了。
    let x: i32;

    // 声明变量后一定要指定类型、在使用之前初始化，否则编译器会报错（consider giving `y` a type）
    // todo：let r;

    // 上面的代码可以写成一行
    let x: i32 = 42;

    // 使用一个变量之前需要初始化，如果没有初始化会在编译的时候报错（use of possibly-uninitialized variable）。
    let x;
    // todo: let y;
    let y: i32;
    // todo: y = x;
    x = 10;

    // 下划线 _ 是一种特殊的变量，表示丢弃。_ 无法被当作变量使用，但是可以给他赋值。
    let _ = 42;
    // todo: print!("x: {} _: {}\n", x, _);
    print!("x: {}\n", x);

    /*||||||||||||||||||||||||||||| 元组 |||||||||||||||||||||||||||||||||||||| */

    // 元组是一组值的集合，集合里面的值可以是不同类型的
    let pair = (1, 2, 'a');
    print!("pair: {} {} {}\n", pair.0, pair.1, pair.2);
    // 标注元组的类型
    let pair: (i32, i32, char) = (3, 4, 'b');

    // 元组还可以被拆分成不同的变量
    let (a, b, c) = pair;
    print!("pair: {} {} {}\n", a, b, c);

    // 这里还可以利用前面提到的下划线，下划线可以用来丢弃元组的部分数据
    let (a, _, _) = pair;

    /*||||||||||||||||||||||||||||| 表达式 |||||||||||||||||||||||||||||||||||||| */
    // 分号表示语句的结束
    let x = 3;
    let y = 5;

    // 同一个语句可以跨越多行
    let x = 3;
    // fn 声明一个函数，下面是一个没有返回值的函数
    fn greet() {
        println!("function greet");
    }

    // 返回 32 位有符号整数的函数
    // 函数末尾没有分号，表示返回该值
    fn roll() -> i32 {
        4
    }

    // 一对大括号声明一个代码块，代码块有自己的作用域。和其他语言一样，作用域能彼此包含。
    // 下面的代码会先输出 "scope: in World", 然后输出 "scope: Hello World"
    let x = "Hello";
    let y = "World";
    {
        let x = "in";
        println!("scope: {} {}", x, y);
    }
    println!("scope: {} {}", x, y);

    // 块也是表达式，块可以作为值
    let x = 42;
    let x = { 42 };
    let x = {
        let y = 1;
        let x = 2;
        x + y
    };
    // if 条件判断也是表达式（必须同时使用 if 和 else）
    let x = if true {
        let y = 3;
        let x = 4;
        x + y
    } else {
        45
    };
    // match 也是表达式，match 的作用类似 Switch
    // _ 作为"捕获全部"的模式
    let x = match 5 {
        5 => 6,
        _ => 8,
    };
    println!("x: {}", x);

    /*||||||||||||||||||||||||||||| 操作符 |||||||||||||||||||||||||||||||||||||| */

    // 点操作符（.），可以用来访问变量的字段，也可以访问一个变量的方法。
    let x = (10, 20);
    println!("x.0: {}", x.0);
    let y = "asur4s";
    println!("y.len(): {}", y.len());

    // 双冒号（::），类似点操作符，用于访问库（crate）或者模块（module）里的函数或者数据类型
    let least = std::cmp::min(3, 8);
    println!("least: {}", least);

    // use 指令，可以把其他模块的变量导入到当前作用域。
    use std::cmp::max;
    use std::cmp::min;

    // 下面的语句和上两句含义相同，重复引用会报错。
    // use std::cmp::{min, max};
    // use std::{cmp::min, cmp::max};

    // 使用 * 导入全部（Python也是这样的
    use std::cmp::*;

    /*||||||||||||||||||||||||||||| 数据类型 |||||||||||||||||||||||||||||||||||||| */

    // str是基本类型，但Vec不是基本数据类型。
    let mut v = Vec::new();
    // todo: let mut v = std::vec::Vec::new();
    v.push(1);
    // 上面的第一个 Vec 可以直接使用，是因为 rust 在每个模块的开头都插入了
    use std::prelude::v1::*;

    // 声明结构体,并初始化
    struct Vec2 {
        x: f64, // 64位浮点数, 也叫 "双精度"
        y: f64,
    }
    let v1 = Vec2 { x: 1.0, y: 3.0 }; 
    println!("v1:{} {}", v1.x, v1.y);

    // 使用其他结构体来更新当前结构体(更新未初始化的字段)
    let v2 = Vec2 {
        x: 14.0,
        ..v1
    };
    println!("v2:{} {}", v2.x, v2.y);
    let v3 = Vec2 {
        ..v1
    };
    println!("v3:{} {}", v3.x, v3.y);

    // 结构体和元组一样,可以被拆分
    let Vec2{ x, y} = v3;
    println!("split vec x, y: {} {}", x, y);
    
    // let可以在 if 中作为条件判断(使用结构体来作为判断条件)
    struct Number{
        odd: bool, 
        value: i32,
    }
    let one = Number{odd:true, value:1};
    let two = Number{odd:false, value:2};
    if let Number{ odd: true, value} = one {
        println!("Odd Number: {}", value);
    }else if let Number{odd: false, value} = one{
        println!("Odd Number: {}", value);
    }
    if let Number{ odd: true, value} = two {
        println!("Odd Number: {}", value);
    }else if let Number{odd: false, value} = two{
        println!("Odd Number: {}", value);
    }

    // match 也可以使用结构体作为判断条件
    match one{
        Number{odd: true, value}=> println!("match true, value: {}", value), 
        Number{odd: false, value}=> println!("match false, value: {}", value),
    }

    /*||||||||||||||||||||||||||||| 数据类型的方法 |||||||||||||||||||||||||||||||||||||| */
    
    // impl 关键字用于实现某个数据类型的方法
    // 其中 self 指当前变量(调用方法的变量)
    impl Number{
        fn is_strictly_positive(self)->bool{
            self.value > 0
        }
    }
    let minus_two = Number{
        odd: false,
        value: -2,
    };
    println!("minus_two positive ? {}", minus_two.is_strictly_positive());

    // 变量默认是不可变类型（immutable）
    let n = Number{
        odd: true,
        value: 17
    };
    // 如果尝试修改它，编译器会报错（cannot assign）
    // todo: n.odd = false;
    // 使用 mut 关键字，就能让变量变为可变类型。
    let mut n = Number {
        odd: true,
        value: 17,
    };
    n.value = 19; // all good

    /*||||||||||||||||||||||||||||| Trait |||||||||||||||||||||||||||||||||||||| */

    // Trait 可以在许多类型下被实现。
    trait Signed{
        // 在 Trait 中，通常只声明函数，不实现
        fn is_strictly_negative(self)->bool;
    }
    // 1. 为自定义类型实现 trait
    impl Signed for Number{
        fn is_strictly_negative(self)->bool{
            self.value < 0
        }
    }
    let minus_two = Number{
        odd: false,
        value: -2,
    };
    println!("minus_two negative ? {}", minus_two.is_strictly_negative());

    // 2. 为自定义类型重载运算符（通过实现标准库中的 Trait）
    impl std::ops::Neg for Number{
        type Output = Number;
        fn neg(self)->Number{
            Number{
                odd: self.odd,
                value: -self.value
            }
        }
    }
    let n = Number { odd: true, value: 987 };
    let m = -n;
    println!("neg 987: {}", m.value);

    // 3. 为原始数据类型设置 trait
    impl Signed for i32{
        fn is_strictly_negative(self)->bool{
            self < 0
        }
    }
    println!("2 negative ? {}", 2.is_strictly_negative());
    println!("-3 negative ? {}", (-3).is_strictly_negative());


    // 

    




 
}

// 拓展阅读：
// 1. Trait 孤儿原则
