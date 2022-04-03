/* 一份代码快速了解Rust */

// PS：可能会有很多 warning，暂时可以不考虑。
// 编译运行：clear && rustc study.rs  && ./study
// 食用指南：从上到下读懂代码即可。使用 todo 标注的部分，都需要你把注释删掉，尝试自行编译运行。

fn main() -> Result<(), std::str::Utf8Error> {
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


    // Copy：只是起标识作用，会直接影响编译器的编译，如果没有使用 Copy 标识，那么就不能被多次拷贝。
    // Number 结构体使用 Copy 来标识
    let n = Number { odd: true, value: 987 };
    let m = n;
    // todo：let o = n;
    let a: i32 = 15;
    let b = a; // `a` 被拷贝
    let c = a; // `a` 再次被拷贝

    // 如果不改变原有值，可以考虑使用不可变引用。
    let n = Number { odd: true, value: 987 };
    let m: &Number = &n;
    println!("m value: {}", m.value);
    let o: &Number = &n;
    println!("o value: {}", o.value);
    // 如果需要改变，使用可变应用（mutable reference）
    let mut n = Number { odd: true, value: 987 };
    let m: &mut Number = &mut n;
    m.value = 123;
    println!("m value: {}", m.value);
    let o: &mut Number = &mut n;
    println!("o value: {}", o.value);
    
    // 实现 Clone Trait
    // Self 指当前数据的数据类型，self 指数据本身
    impl std::clone::Clone for Number{
        fn clone(&self)->Self{
            Self { ..*self }
        }
    }
    // 执行 trait 方法时,接收者（receiver）是隐式借用
    let n = Number{odd: true, value:51};
    let mut m = n.clone();
    m.value = 100;
    println!("m value: {}, n value: {}", m.value, n.value);
    // 下面的写法和上面是等价的
    let m = std::clone::Clone::clone(&n);
    
    // 标识 copy 
    impl std::marker::Copy for Number {}
    let n = Number { odd: true, value: 51 };
    let mut m = n; 
    let o = n;
    m.value = 150;
    println!("m value: {}, n value: {}", m.value, n.value);


    // Clone 和 Copy 是经常使用的 Trait，并且编写并不复杂，所以可以使用 derive 属性来自动生成。
    #[derive(Copy, Clone)]
    struct Num{
        odd: bool,
        value: i32
    }
    let n = Num { odd: true, value: 51 };
    let mut m = n; 
    let o = n;
    m.value = 200;
    println!("Num Struct -> m value: {}, n value: {}", m.value, n.value);

    /*||||||||||||||||||||||||||||| 泛型 |||||||||||||||||||||||||||||||||||||| */

    // 函数设置为泛型（没有数据类型，只有参数）
    use std::fmt::{Debug, Display};
    fn print<T:Display>(value: T){
        println!("value = {}", value);
    }
    // 当然还可以对泛型进行约束，标识使用范围（要求一个数据类型实现多个 Trait）。
    fn compare<T>(left: T, right: T) where T: Debug + PartialEq,{
        println!("{:?} {} {:?}", left, if left == right{"=="}else{"!="}, right);
    }
    compare("tea", "coffee");
    compare(1, 1);

    // 泛型函数可以被当作命名空间，因为里面包含了很多具体的函数。
    // 所以可以使用 :: 来绑定泛型的类型（turbofish语法，::<>看起来像一条鱼）
    let v = Vec::<bool>::new();
    println!("{:?}", v);
    let v: Vec<bool> = Vec::new();
    println!("{:?}", v);

    // 提到 Vec，还有一个 vec! 的宏（Vec字面量），用于快速创建 Vec。
    let v1 = vec![1, 2, 3];
    let v2 = vec![true, false, false];
    println!("macro: {:?}", v1);
    println!("macro: {:?}", v2);
    // vec!()， vec![]，vec!{} 都可以调用这个宏
    let v1 = vec!(1, 2, 3);
    let v2 = vec!(true, false, false);
    println!("macro: {:?}", v1);
    println!("macro: {:?}", v2);

    // 事实上，println! 也是一个宏。
    println!("{}", "Hello World!");
    // 等价的写法
    use std::io::{self, Write};
    io::stdout().lock().write_all(b"Hello World!\n").unwrap();

    // panic 也是一个宏，它会输出一个错误信息，并且强制终止程序。
    // todo: panic!("panic!");
    
    // 一些方法也会造成 panic，比如 Option 类型不包含任何东西的时候调用 unwrap 函数。
    enum MyOption<T> {
        None,
        Some(T),
    }
    
    impl<T> MyOption<T> {
        fn unwrap(self) -> T {
            // enums variants can be used in patterns:
            match self {
                Self::Some(t) => t,
                Self::None => panic!(".unwrap() called on a None option"),
            }
        }
    }

    /*||||||||||||||||||||||||||||| 生命周期 |||||||||||||||||||||||||||||||||||||| */
    
    // 变量声明是有生命周期的
    {
        let x = 42;
        println!("x = {}", x);
    }
    // x 声明周期结束，无法找到 x

    // 类似，变量引用也有生命周期。
    {
        let x = 42;
        let x_ref = &x;
        println!("x_ref = {}", x_ref);
    }

    // 1. 引用的生命周期不能超过原变量的声明周期
    let x_ref = {
        let x = 1;
        1
        // todo: &x
    };
    println!("x_ref = {}", x_ref);

    // 2. 变量可以多次不可变借用。
    let x = 42;
    let x_ref1 = &x;
    let x_ref2 = &x;
    let x_ref3 = &x;
    println!("x_ref1 = {}, x_ref2 = {}, x_ref3 = {}", x_ref1, x_ref2, x_ref3);

    // 3. 可变变量在被借用（不可变借用+可变借用）的时候，无法修改
    let mut x = 32;
    let x_ref = &x;
    // todo：let mut x_ref = &mut x;
    // todo: x = 99;
    println!("x_ref = {}", x_ref);

    // 4. 如果变量已经被不可变借用，就不能再被可变借用
    let mut x = 22;
    let x_ref1 = &x;
    // todo: let x_ref2 = &mut x;
    println!("x_ref1 = {}", x_ref1);


    // 函数参数也有生命周期
    // 带有引用参数的函数都是泛型。
    fn printl(x: &i32){}

    // 生命周期可以使用 ' 来进行标注（命名），这里标注了引用的生命周期，所以写成 &'a
    fn printlt<'a>(x: &'a i32){}

    // 使用命名后的生命周期，可以让函数返回引用，引用的生命周期依赖于函数参数的生命周期。
    struct NewNumber{
        value: i32
    }
    fn number_value<'a>(num: &'a NewNumber) -> &'a i32{
        &num.value
    }
    let n = NewNumber{ value: 47};
    let v = number_value(&n);       // 直接把函数内部的引用传出来了。上面的写法比较多余，不使用命名生命周期也可以实现。

    // 结构体也可以使用命名生命周期
    struct NumRef<'a>{
        x: &'a i32
    }
    let x: i32 = 99;
    let x_ref = NumRef{
        x: &x
    };
    
    // impl 代码块也可以使用命名生命周期
    impl<'a> NumRef<'a>{
        fn as_i32_ref(&'a self)->&'a i32{
            self.x
        }
    }
    let x = 99;
    let x_num_ref = NumRef{ x: &x};
    let x_i32_ref = x_num_ref.as_i32_ref();
    println!("x_i32_ref = {}", x_i32_ref);

    // 如果嫌弃命名生命周期写起来太麻烦，可以使用省略写法
    impl<'a> NumRef<'a>{
        fn as_i32_refElide(&self)->&i32{
            self.x
        }
    }
    impl NumRef<'_>{
        fn as_i32_refElideElide(&self)->&i32{
            self.x
        }
    }

    // 'static 是一种特殊的生命周期，可以让变量在整个运行时都有效
    struct Person{
        name: &'static str,
    }
    let p = Person{
        name: &"Bob"
    };

    // 但由堆分配的字符串（owned strings）不能设置为 'static
    let n = format!("name: {}", "Alice");
    // todo: let pa = Person{name: &n};

    // 想要存储一个普通字符串（non-'static）到 Person 中，可以用下面两种办法
    // 1. 借用
    struct PersonA<'a>{
        name: &'a str
    }
    let name = format!("name: {}", "Alice");
    let p = PersonA{ name: &name};
    // 2. 申请一个新的字符串。
    struct PersonB{
        name: String
    }
    let name = format!("name: {}", "Alice");
    let p = PersonB{ name: name};
    // 顺便提一句，当字段名和变量名相同时，可以省略
    let name = format!("name: {}", "Alice");
    let q = PersonB{name};

    /*||||||||||||||||||||||||||||| 数组切片 |||||||||||||||||||||||||||||||||||||| */

    // 切片可以获得数组 Vec 中连续的一组元素。
    let v = vec![1, 2, 3, 4, 5, 6];
    let v2 = &v[2..4];          // .. 在结构体中可以更新字段。
    println!("v2 = {:?}", v2); 

    // 在这里，.. 的含义是 range。类似 Python 中的 range，都是半闭半开，2..4 表示集合 [2, 4)。
    // 如果想要包含最右边的元素，可以使用等号来处理
    println!("(0..) contains 100? {:?}", (0..).contains(&100));
    println!("(..20) contains 20? {:?}", (..20).contains(&20));
    println!("(..=20) contains 20? {:?}", (..=20).contains(&20));
    println!("(3..6) contains 4? {:?}", (3..6).contains(&4));

    // 借用的时候也可以使用切片
    fn tail(s: &[u8]) -> &[u8] {
        &s[1..]
    }
    let x = [1, 2, 3, 4, 5];
    let y = tail(&x);
    println!("y = {:?}", y);

    // 下面这样也是合法的，因为 [1,2, 3, 4, 5] 的生命周期是 'static
    let y = {
        let x = &[1, 2, 3, 4, 5];
        // todo（不合法）: let x = [1, 2, 3, 4, 5]; tail(&x)
        // Vector 是分配在堆上的，生命周期不是 'static。
        // todo（不合法）: let x = &vec![1, 2, 3, 4, 5];
        tail(x)
    };
    println!("y = {:?}", y);

    // 上面的 tail 函数还可以写成
    fn new_tail<'a>(s: &'a [u8]) -> &'a [u8]{
        &s[1..]
    }
    let x = [1, 2, 3, 4, 5];
    let y = new_tail(&x);
    println!("y = {:?}", y);


    // &str 也可以使用切片。
    fn file_ext(name: &str) -> Option<&str>{
        name.split(".").last()
    }
    let name = "hello.txt";
    if let Some(ext) = file_ext(name){
        println!("file extension: {}", ext);
    }else{
        println!("no file extension.");
    }

    /*||||||||||||||||||||||||||||| 错误处理 |||||||||||||||||||||||||||||||||||||| */
    // 当函数调用失败，通常会返回一个 Result 类型的数据
    let s = std::str::from_utf8(&[240, 159, 141, 137]);
    println!("{:?}", s);
    let s = std::str::from_utf8(&[195, 40]);
    println!("{:?}", s);

    // 如果想要让程序直接暂停，可以使用 unwrap()
    // todo: let s = std::str::from_utf8(&[195, 40]).unwrap();
    println!("{:?}", s);

    // 如果想要输出自定义的错误信息，可以使用 expect()
    // todo: let s = std::str::from_utf8(&[195, 40]).expect("valid utf-8");
    println!("{:?}", s);
    // 使用 match 也可以输出自定义的消息
    match std::str::from_utf8(&[240, 159, 141, 137]){
        Ok(s) => println!("{:?}", s),
        Err(e) => panic!(e),
    }
    
    // 还可以使用 match 把错误抛到上一层
    match std::str::from_utf8(&[240, 159, 141, 137]){
        Ok(s) => println!("{:?}", s),
        Err(e) => return Err(e),
    }

    /*||||||||||||||||||||||||||||| 解引用 |||||||||||||||||||||||||||||||||||||| */
    // * 操作符可以用于解引用，访问内部字段或者调用方法时可以不解引用。
    struct NewPoint {
        x: f64, 
        y: f64
    }
    let p = NewPoint{x: 1.0, y: 2.0};
    let p_ref = &p;
    println!("({}, {})", p_ref.x, p_ref.y);
    
    // 解引用只能用于实现了 Copy Trait 的类型上使用
    #[derive(Clone, Copy)]
    struct CopyPoint {
        x: f64, 
        y: f64
    }
    fn negative(p: CopyPoint) -> CopyPoint{
        CopyPoint{
            x: -p.x,
            y: -p.y
        }
    }
    let p = CopyPoint{x: 1.0, y: 2.0};
    let p_ref = &p;
    let np = negative(*p_ref);
    println!("({}, {})", np.x, np.y);

    /*||||||||||||||||||||||||||||| 闭包 |||||||||||||||||||||||||||||||||||||| */

    // 闭包是 Fn、FnMut、FnOnce 类型的函数，并且可以捕获上下文
    // 闭包的参数放在一对竖线（|）之间，并且不需要大括号，除非你需要写多个语句。
    fn for_each_planet<F>(f: F) where F:Fn(&'static str){
        f("Earth");
        f("Mars");
        f("Jupiter");
    }
    for_each_planet(| planet | println!("Hello {}", planet));

    // 闭包中的借用规则
    let greeting  = String::from("Good to see you");
    // 借用 greeting
    for_each_planet(| planet | println!("{} {}", greeting, planet));

    fn for_each_planet_static<F>(f: F) where F: Fn(&'static str) + 'static{
        f("Earth");
        f("Mars");
        f("Jupiter");
    }
    let greeting  = String::from("Static");
    // greeting 生命周期虽然比函数长，但不符合 'static 
    // todo: for_each_planet_static(| planet | println!("{} {}", greeting, planet));
    // 使用move强制获取变量所有权
    for_each_planet_static(move |planet| println!("{}, {}", greeting, planet));

    // FnMut 参数必须是可变借用，所以在使用时只能调用一次。但 Fn 却可以多次调用
    fn foobar_fn<F>(f: F) where F: Fn(i32) -> i32{
        println!("{}", f(f(2)));
    }
    foobar_fn(| x | x*2);

    fn foobar_fnmut<F>(mut f: F) where F: FnMut(i32) -> i32{
        // todo: println!("{}", f(f(2)));
        let tmp = f(2);
        println!("{}", f(tmp));
    }
    foobar_fnmut(| x | x*2);

    // FnMut 可变借用局部变量，但 Fn 却不能实现。
    let mut acc = 2;
    foobar_fnmut(| x |{
        acc += 1;
        x * acc
    });
    println!("acc {}", acc);

    // FnOnce 传入的闭包只能被调用一次。有时传入的闭包中含有 moved 进来的变量，只调用一次是最好的。
    fn foobar_fnonce<F>(f: F) where F: FnOnce() -> String{
        println!("{}", f());
        //todo：println!("{}", f());
    }
    let s = String::from("call FnOnce");
    foobar_fnonce(move || s);

    // 带有两个参数的闭包
    fn foobar_two<F>(x: i32, y: i32, is_greater: F) where F:Fn(i32, i32) -> bool{
        let (greater, smaller) = if is_greater(x, y){
            (x, y)
        }else{
            (y, x)
        };
        println!("{} is greater than {}", greater, smaller);
    }
    foobar_two(32, 64, |x, y| x>y);

    // 马桶闭包：什么都不做
    fn countdown<F>(count: usize, tick: F) where F:Fn(usize){
        for i in (1..=count).rev(){
            tick(i);
        }
    }
    countdown(3, |i|println!("tick {}", i));
    countdown(3, |_|());

    /*||||||||||||||||||||||||||||| 可迭代类型 |||||||||||||||||||||||||||||||||||||| */

    


    // main 函数的返回值
    Ok(())
    // https://juejin.cn/post/6844904079915745288
    // https://juejin.cn/post/6844904080314204173
}

// 拓展阅读：
// 1. Trait 孤儿原则
// 2. Clone VS Copy：https://zhuanlan.zhihu.com/p/21730929
// 3. Lifetime: 
// 4. str Vs String
// 5. deref、derefmut 对应智能指针，常规解引用估计（C 语言常识
// 6. 命名空间
