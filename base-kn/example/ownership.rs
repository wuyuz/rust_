fn main() {
    let s = String::from("hello,");
    let mut s1 = s;  // 可变性没法变化，需要加mut后才可以后行
    s1.push_str("world");

    //---
    let s = give_ownership();
    println!("{}", s);

    //---
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone(); // 使用Clone是可以的，也可以使用Copy
    println!("{:?}, {:?}", x, y);

    //---
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
    // 仅修改下面这行代码，且不要使用 `_s`, 
    println!("{:?}", t.1); // 不能是否t.0

    //---
    let t = (String::from("hello"), String::from("world"));
    // 填空，不要修改其它代码
    let (ref s1, ref s2) = t;  // 部分move
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")

    // 引用
    
}

// 只能修改下面的代码!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    // let _s = s.into_bytes();  // 报错，因为s用了后后面不能用了，所以需要改变
    let _s = s.clone().into_bytes(); 
    let _s1 = s.as_bytes(); 
    s
}
