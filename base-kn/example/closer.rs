
struct Cacher<T> 
    where T: Fn(u32) -> u32,
{
    query:T,
    value:Option<u32>
}

impl<T> Cacher<T> 
    where T: Fn(u32) -> u32,
{
    fn new(query:T) -> Cacher<T> {
        Cacher { query, value: None }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg:u32) -> u32 {
        match  self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct SuperCacher<T,E> 
    where T: Fn(E)->E,
    E:Copy,
{
    query: T,
    value: Option<E>
}

impl<T,E> SuperCacher<T,E> 
    where T: Fn(E)->E,
    E:Copy,
{
    fn new(query:T)-> SuperCacher<T,E> {
        SuperCacher { 
            query,
            value:None
        }
    }

    fn value(&mut self, arg:E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let c = (self.query)(arg);
                self.value = Some(c);
                c
            }
        }
    }
}


fn main() {
    let x:u32 = 5;
    let mut s = Cacher{
        query: |x|{x+1},
        value: None,
    };
    println!("s 第一次取值：{:?}", s.value(34));  // 35
    let mut s = Cacher::new(|x|{x*2});
    println!("s 第二次取值：{:?}", s.value(4));   // 8

    // 上面方案只能针对u32类型
    let mut c = SuperCacher::new(|a| a);

    // let v1 = c.value(1);  // 一个范型只能说定义一次，因为范性在编译的时候会根据上下自动生成相关代码
    let v2 = c.value("xxx");  // 可字符串，可数字
    println!("s 第三次取值：{:?}", v2);   // xxx
}

mod test {
    //闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：
        //转移所有权、可变借用、不可变借用，因此相应的 Fn 特征也有三种
    //1.FnOnce，获取所有权，意思是只能用一次
    fn func_one<F>(f:F)
        where F:(FnOnce(usize)->bool) + Copy  // 必须使用copy，确保f可以复制
    {
        println!("{}", f(3));  // 传值3给f
        println!("{}", f(4));  // 传值4给f
    }

    //也可以通过move强制使用所有权，这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程
    fn func_move() {
        use std::thread;
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();    
    }

    #[test]
    fn test_once() {
        let v = vec![1,2,3];
        func_one(|z|{z==v.len()}) // 调用func_one，传入必报函数，得到 true、false
    }

    //2.FnMut，它以可变借用的方式捕获了环境中的值，因此可以修改该值
    #[test]
    fn test_mut() {
        let mut s = String::new();
        // 需要提前预判这个变量是可变的
        let mut update_string =  |str| s.push_str(str);
        update_string("hello");

        println!("{:?}",s); // "hello"

        // 方法二
        let update_string_2 = |str| s.push_str(str);
        exec(update_string_2);
        println!("{:?}",s);  // "hellohello"
    }

    fn exec<'a,F>(mut f:F)   // 需要给定字符串的生命周期和函数的关系
        where F:FnMut(&'a str)
    {
        f("hello")
    }

    //3. Fn 特征，它以不可变借用的方式捕获环境中的值 
    #[test]
    fn test_fn() {
        let s = "hello, ".to_string();
        let update_string =  |str| println!("{},{}",s,str);
        exec_1(update_string);
    
        println!("{:?}",s);
    }
    
    fn exec_1<'a, F: Fn(String) -> ()>(f: F)  {
        f("world".to_string()) // 因为无需改变 s，因此闭包中只对 s 进行了不可变借
    }

    //move 关键字对于 FnOnce 特征的重要性，但是实际上使用了 move 的闭包依然可能实现了 Fn 或 FnMut 特征
        //根据你对抓取变量的使用环境，定义Fn、Fnmut、Fnonce的那一种
        //实际上，一个闭包并不仅仅实现某一种 Fn 特征，规则如下：
            //1、所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
            //2、没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
            //3、不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
        // 从特征约束能看出来 Fn 的前提是实现 FnMut，FnMut 的前提是实现 FnOnce，因此要实现 Fn 就要同时实现 FnMut 和 FnOnce
        // Fn 获取 &self，FnMut 获取 &mut self，而 FnOnce 获取 self。 在实际项目中，建议先使用 Fn 特征，然后编译器会告诉你正误以及该如何选择
    
    //闭包作为返回值时，需要使用impl来包裹，因为Fn(i32)->i32在编译，Rust要求函数的参数和返回类型，绝大部分类型都有固定的大小，但是不包括特征
    // fn factory(x:i32) -> impl Fn(i32) -> i32 {
    //     let num = 5;

    //     if x > 1{  
    //         move |x| x + num
    //     } else {
    //         move |x| x - num   // 依旧报错，因为返回的类型不一样，为什么？看起来一样呀？就算签名一样的闭包，类型也是不同的
    //     }
    // }

    // 通过Box解决问题
    fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
    
        if x > 1{
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x - num)
        }
    }
}