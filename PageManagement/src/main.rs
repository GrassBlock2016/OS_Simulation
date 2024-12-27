mod page_conversion;    // 实验要求一
mod page_replacement;   // 实验要求二
mod block_num_compare;  // 实验要求三
mod algorithm_compare;  // 实验要求四

fn main() {
    loop {
        println!("1. 实验要求一");
        println!("2. 实验要求二");
        println!("3. 实验要求三");
        println!("4. 实验要求四");
        println!("5. 退出");
        println!("请选择要执行的实验：");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("无效选项，请重新输入\n");
                continue;
            }
        };
        match choice {
            1 => page_conversion::exp1(),
            2 => page_replacement::exp2(),
            3 => block_num_compare::exp3(),
            4 => algorithm_compare::exp4(),
            5 => break,
            _ => {
                println!("无效选项，请重新输入\n");
                continue;
            }
        }
    }
}
