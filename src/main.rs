
// 默认情况下，rust会将prelude隐式地导入到每个程序的作用域中
// 若使用的程序不在prelude中，则必须显式导入
use std::io; // std是rust的标准库，use是rust导包的关键字
use rand::Rng; // trait，类似于接口
use std::cmp::Ordering; //标准库中的枚举类型

fn main() {
    println!("猜数字！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("神秘数字是{}", secret_number);

    loop {
        println!("猜测一个数！");
        /* 
        let mut foo = 1;
        let bar = foo; // 默认情况下，rust的变量为不可变变量 immutable
        // 默认为不可变变量的原因是，经统计程序中值不变的变量更多
        // 要声明一个可变变量，需要使用mut关键字
        foo = 2;
        
        let（不可变变量）与const（常量）是不同的
        let可以只声明不赋值，比如 “let a:i32;”（注意，a后面跟冒号与类型）
        比如在某个函数中，用let a接受了参数，随着参数的改变，a的值在每次初始化时，也会改变
        */

        /*
        ::new()表明new()函数是String的关联函数
        意为该函数是针对类型本身实现的，而非针对某个特定实例实现
        类似于java中的静态方法
        */
        let mut guess = String::new();
        /*
        参数中的mut是对传参方式的指定
        mut前的取地址符号&表示该参数是一个引用，即方法的参数是按引用进行传递的
        通过引用，可以在代码的不同位置访问同一块数据
        在rust中，引用也默认为不可变

        expect()用于抛出异常，若不写会报警告
        */
        io::stdin().read_line(&mut guess).expect("无法读取行");

        // 将字符串型guess转为整型
        // shadow机制，允许复用变量名，常用于类型转换场景
        // trim()去除字符串两端的空白 parse()将字符串解析为数字
        // u32是rust内置的无符号整数类型
        let guess: u32 = match guess.trim().parse(){
            // 不使用expect函数抛出异常，那样会使得程序崩溃
            // 转而使用match，对不同的输入进行不同的处理可以增强健壮性
            // 使用match处理错误是rust的惯用手段
            Ok(num) => num,
            // 使用_下划线表示不
            Err(_) => continue
        };
        // 花括号是占位符，依次对应后面的参数
        println!("你猜测的数是：{}",guess);


        // 比较两个数字的大小
        // 这里的比较函数会使得secret_number的类型随着guess改变而改变
        // 枚举类型(此处为cmp::Ordering)可以用match关键字进行逐一匹配
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!");break;}
        }
    }   
}
