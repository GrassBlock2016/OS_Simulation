use std::io::Write;
mod memory;

fn partition(algor: &str) {
    let mut memory = memory::Memory::new();
    memory.init();
    loop {
        memory.print_table();
        print!("申请 or 释放 or 退出？(a/r/q)：");
        std::io::stdout().flush().unwrap();
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "a" => {
                print!("请输入作业名：");
                std::io::stdout().flush().unwrap();
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                print!("请输入作业大小(KB)：");
                std::io::stdout().flush().unwrap();
                let mut size = String::new();
                std::io::stdin().read_line(&mut size).unwrap();
                let size: u16 = size.trim().parse().unwrap();
                match algor {
                    "first_fit" => memory.first_fit(&name, size),
                    "next_fit" => memory.next_fit(&name, size),
                    "best_fit" => memory.best_fit(&name, size),
                    "worst_fit" => memory.worst_fit(&name, size),
                    _ => println!("无效的算法 {}", algor),
                }
            }
            "r" => {
                print!("请输入作业名：");
                std::io::stdout().flush().unwrap();
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                memory.release(&name);
            }
            "q" => return,
            _ => println!("无效的选项 {}", choice),
        }
    }
}

fn main() {
    println!("1. 首次适应算法");
    println!("2. 循环首次适应算法");
    println!("3. 最佳适应算法");
    println!("4. 最坏适应算法");
    println!("5. 退出");
    print!("请输入对应序号选择相应的算法：");
    std::io::stdout().flush().unwrap();
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    match choice.trim() {
        "1" => partition("first_fit"),
        "2" => partition("next_fit"),
        "3" => partition("best_fit"),
        "4" => partition("worst_fit"),
        "5" => return,
        _ => println!("无效的选项 {}", choice),
    }
}
