use std::io;
use rand::Rng;
use std::cmp::Ordering;
// use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    println!("请输入数字");
    let secret_number = rand::thread_rng().gen_range(1,101);
    loop {
       // println!("随机数是 {}",secret_number);
        let mut  guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取错误");
        println!("这个是{}",guess);
        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue
        };
        match guess.cmp(&secret_number) {
            Ordering::Less=>println!("太小"),
            Ordering::Greater=>println!("太大"),
            Ordering::Equal=>{
                println!("就是它了");
                break;
            },
        }
    }

}
