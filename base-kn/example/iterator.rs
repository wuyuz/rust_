#![feature(test)]
extern crate test as other_test;  // 引入rust自带的test模块


fn main() {

    //迭代器之所以成为迭代器，就是因为实现了 Iterator 特征，要实现该特征，最主要的就是实现其中的 next 方法，
    //该方法控制如何从集合中取值，最终返回值的类型是关联类型 Item。

    // 模拟for循环
    let values = vec![1,2,4,6];
    {
        let result = match IntoIterator::into_iter(values) {  // ?为什么这里可以match，明明没有None，
            mut iter => loop {  // 相当于封装了一层，不停的loop对接iter，看是否为None
                match iter.next() {
                    Some(x) => { println!("{}", x); },
                    None => break,
                }
            },
        };
        println!("{:?}",result);  // ()
        result
    }

    let v = vec![Some(3),Some(6),Some(9),None]; 
    let r = match v.into_iter() {
        mut iter =>loop {
            match iter.next() {
                Some(x) => { println!("{:?}", x); },
                None => break,
            }
        }
    };

}

#[cfg(test)]
mod test {
    // 1、into_iter 会夺走所有权
    // 2、iter 是借用
    // 3、iter_mut 是可变借用
    //你会发现这种问题一眼就能看穿，into_ 之类的，都是拿走所有权，_mut 之类的都是可变借用，剩下的就是不可变借用。

    #[test]
    fn test_iter(){
        let values = vec![1, 2, 3];
        for v in values.into_iter() {
            println!("{}", v)
        }
        // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
        // println!("{:?}",values);

        let values = vec![1, 2, 3];
        let _values_iter = values.iter();

        // 不会报错，因为 values_iter 只是借用了 values 中的元素
        println!("{:?}", values);

        let mut values = vec![1, 2, 3];
        // 对 values 中的元素进行可变借用
        let mut values_iter_mut = values.iter_mut();

        // 取出第一个元素，并修改为0
        if let Some(v) = values_iter_mut.next() {
            *v = 0;
        }

        // 输出[0, 2, 3]
        println!("{:?}", values);
    }

    //Iterator 和 IntoIterator 的区别
    //这两个其实还蛮容易搞混的，但我们只需要记住，Iterator 就是迭代器特征，只有实现了它才能称为迭代器，才能调用 next。
    //而 IntoIterator 强调的是某一个类型如果实现了该特征，它可以通过 into_iter，iter 等方法变成一个迭代器。

    #[test]
    fn test_collect() {
        use std::collections::HashMap;
        //与消费者适配器不同，迭代器适配器是惰性的，意味着你需要一个消费者适配器来收尾，最终将迭代器转换成一个具体的值
        let v = vec![1,4,6];

        let iter_func = v.iter().map(|x| x + 1);  // 迭代器适配器，惰性
        let v2: Vec<_> =iter_func.collect();  // 需要消费适配器消费
        println!("{:?}",v2); //[0, 2, 3]

        let names = ["sunface", "sunfei"];
        let ages = [18, 18];
        let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    
        println!("{:?}",folks);
        // zip 是一个迭代器适配器，它的作用就是将两个迭代器的内容压缩到一起，形成 Iterator<Item=(ValueFromA, ValueFromB)> 这样的新的迭代

        // filter迭代适配器
        struct Shoe {
            size: u32,
            style: String,
        }
        
        fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        }   
    }

    #[test]
    fn test_Itorater() {
        // 自定义Itorater
        struct Counter {
            counter: u32,
        }

        impl Counter {
            fn new()->Counter {
                Counter { counter: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> { // 可变引用
                if self.counter < 5 {
                    self.counter += 1;
                    Some(self.counter)
                }else{
                    None
                }
            }
        }

        let mut counter = Counter::new();
        for c in counter.into_iter() {
            println!("xxx{:?}",c)
        }

        // 实现了Itorater特征后，其他的迭代器适配器都可以用了，其它方法都具有默认实现，所以无需像 next 这
        //样手动去实现，而且这些默认实现的方法其实都是基于 next 方法实现的
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1)) // 新建另一个counter，并跳过1，[2,3,4,5]=> [(1,2),(3,4)...]
            .map(|(a, b)| a * b)  // 俩俩相乘, [2,6,12,20]
            .filter(|x| x % 3 == 0) //[6,12]
            .sum(); //消费者
        assert_eq!(18, sum);

        let mut counter = Counter::new();
        for (i,v) in counter.enumerate() {
            println!("第{}个值是{}",i,v)
        }

    }
}


// 迭代器和for的性能对比
fn sum_for(x: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}

#[cfg(test)]
mod bench {
    use other_test::Bencher;
    use rand::{Rng,thread_rng};
    use super::*;

    const LEN: usize = 1024*1024;

    fn rand_array(cnt: u32) -> Vec<f64> {
        let mut rng = thread_rng();
        (0..cnt).map(|_| rng.gen::<f64>()).collect()
    }

    #[bench]
    fn bench_for(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| {
            sum_for(&samples)
        })
    }

    #[bench]
    fn bench_iter(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| {
            sum_iter(&samples)
        })
    }
}