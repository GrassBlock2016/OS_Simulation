// 实验要求三：不同物理块数量下页面置换算法的性能比较
use std::io::Write;
use crate::page_conversion::{generate_address, convert_to_page};
use crate::page_replacement::{opt, lru, fifo};

fn print_result(page: &Vec<u32>, algor: fn(&Vec<u32>, u32, bool) -> u32) {
    println!("MEMORY\t\t\tMISS COUNT\t\tHIT RATE");
    for i in 1..17 {
        let miss_count = algor(page, i, false);
        println!("{}\t\t\t{}\t\t\t{:.3}%", 2 * i, miss_count, 100.0 - miss_count as f64 / page.len() as f64 * 100.0);
    }
}

pub fn exp3() {
    let n = 256;
    let address = generate_address(n);
    let page_size = 1024;
    let page = convert_to_page(&address, page_size);

    println!("= = = = = = = = = = = = = = = = = = = = = = = = = = = = = =");
    print!("The algorithm is [OPT = 1, LRU = 2, FIFO = 3]:");
    std::io::stdout().flush().unwrap();
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("无效选项，请重新输入\n");
            return;
        }
    };
    match choice {
        1 => print_result(&page, opt),
        2 => print_result(&page, lru),
        3 => print_result(&page, fifo),
        _ => {
            println!("无效选项，请重新输入\n");
            return;
        }
    }
    println!("= = = = = = = = = = = = = = = = = = = = = = = = = = = = = =\n");
}