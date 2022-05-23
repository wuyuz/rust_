
fn main() {
    let c = '中';

    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
    let ref r2 = c; // ref 与 & 类似，可以用来获取一个值的引用，但是它们的用法有所不同。

    assert_eq!(*r1, *r2);
    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr(r1),get_addr(r2));

    // ---
     // 通过修改下面一行代码来修复错误
     let mut s = String::from("hello, ");

     borrow_object(&mut s);
}

// 获取传入引用的内存地址的字符串形式
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

fn borrow_object(_s: &mut String) {}

mod test {
    use std::collections::HashMap;
    use std::hash::Hash;

    fn get_default<'m, K, V>(map: &'m mut HashMap<K,V>, key: K) -> &'m mut V
    where
    K:Clone+Eq+Hash,
    V:Default,
    {
        map.entry(key).or_insert(V::default())
    }

    fn get_default_third<'a>(map: &'a mut HashMap<usize,&str>, key: usize) -> &'a str {
        if let Some(v)=map.get_mut(&key){
            return v
        }
        map.entry(key).or_insert("third")
    }

    #[test]
    fn get_map(){
        let mut m:HashMap<usize,&str> = HashMap::new();
        let v= get_default(&mut m,0);
        println!("test map lifetime {:?}",v);  // test map lifetime ""

        *v = "second";
        let v2= get_default(&mut m,0);
        println!("test map lifetime second {:?}",v2); // test map lifetime "second"

        let v3= get_default_third(&mut m,1);
        println!("test map lifetime third {:?}",v3); // test map lifetime "third"
    }
}
