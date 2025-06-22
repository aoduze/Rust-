use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    //这句话的意思大概是我们在本地当前执行线程的环境中操作获取seed,然后为我们调用
    //随机数生成函数,
loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line ");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
        //如果是ok,那么我们就把num的值赋给前面的guess
        //否则就跳过这一次,重新执行loop的开头
    };


    println!("your guess : {}",guess);
    //Rust允许用一个新的guess来遮盖旧的guess,复用是这样复用的
    //u32表示32位无符号数字,不过Rust默认使用的是i32
    //有一点需要注意,parse只可以用于逻辑转换为数字的字符,如果字符是一些错乱的例如:*&%等
    //trim方法会帮助我们取出字符串靠头和结尾的空白字符


        println!("Your guessed : {}",
        guess);

        match guess.cmp( & secret_number) {
        Ordering::Less => println ! ("Too small"),
        Ordering::Greater => println ! ("Too big"),
        Ordering::Equal => {
            println!("You win!");
            break;
           }
        }
    }
}
