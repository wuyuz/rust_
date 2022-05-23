
use std::fmt;

//如果在外部类型上实现外部特征必须使用 newtype 的方式，否则你就得遵循孤儿规则：
    //要为类型 A 实现特征 T，那么 A 或者 T 必须至少有一个在当前的作用范围内。
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}


//上面的代码将顺利编译通过，但是如果你使用 newtype 模式，该代码将无情报错，简单做个总结：
    //1、类型别名仅仅是别名，只是为了让可读性更好，并不是全新的类型，newtype 才是！
    //2、类型别名无法实现为外部类型实现外部特征等功能，而 newtype 可以

//不定长和定长，如果从编译器何时能获知类型大小的角度出发，可以分成两类：
    //1、定长类型( sized )，这些类型的大小在编译时是已知的
    //2、不定长类型( unsized )，与定长类型相反，它的大小只有到了程序运行时才能动态获知，这种类型又被称之为 DST   

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}