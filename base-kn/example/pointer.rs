
fn main() {
    // const T形式
    let x = 5;
    let raw : *const i32 = &x;  // let raw = x as *const i32  // 两种方式都可以
    let point_at = unsafe{
        *raw
    };
    println!("指针raw:{:?},值：{:?}",raw,point_at); // 指针raw:0x16b646edc,值：5
    // mut *T形式
    let mut x = 2;
    let y = &mut x as *mut i32;
    
    // Box<T>的转换
    let a: Box<i32> = Box::new(10);
    // 我们需要先解引用a，再隐式把 & 转换成 *
    let _b: *const i32 = &*a;
    // 使用 into_raw 方法
    let _c: *const i32 = Box::into_raw(a);
    // 如上说所，引用和裸指针之间可以隐式转换，但隐式转换后再解引用需要使用unsafe：
    // 显式
    let a = 1;
    let b: *const i32 = &a as *const i32; //或者let b = &a as *const i32；
    // 隐式
    let c: *const i32 = &a;
    unsafe {
        println!("隐式转换：{},{}", *c,*b); // 1,1,都需要在unsafe中使用
    }

    // 可变静态变量
    static mut N:i32 = 3;
    unsafe {
        N += 1;
        println!("{:?}",N);  // 4
    }

    // 不安全函数
    unsafe fn foo() {
        print!("unsafe function");
    }
    unsafe {
        foo()
    }


}

mod test {
    use std::{slice::from_raw_parts, str::from_utf8_unchecked};

    fn get_memory_location() -> (usize, usize) {
    // “Hello World” 是字符串字面量，因此它的生命周期是 `'static`.
    // 但持有它的变量 `string` 的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
    // `string` 在这里被 drop 释放
    // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
    }

    fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    // 使用裸指针需要 `unsafe{}` 语句块
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
    }

    #[test]
    fn test_str() {
        let (pointer, length) = get_memory_location();
        let message = get_str_at_location(pointer, length);
        println!(
            "The {} bytes at 0x{:X} stored: {}",
            length, pointer, message
        );
        // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
        let message = get_str_at_location(1000, 10);
    }

    #[test]
    fn test_T() {
        use std::fmt::Debug;
        fn print_it<T: Debug + 'static>(input:&T) {
            println!( "'static value passed in is: {:?}", input );
        }

        fn print_it1(input: impl Debug+'static){
            println!( "'static value passed in is: {:?}", input );
        }

        let i = 5;
        print_it(&i);
        print_it1(&i); // 显然这个函数的参数无法满足， 可修改为const i:i32 = 5;

    }   
}