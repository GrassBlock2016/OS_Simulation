// 实验要求四：不同页面置换算法之间的比较
use crate::page_conversion::{generate_address, convert_to_page};
use crate::page_replacement::{opt, lru, fifo};

fn print_result(page: &Vec<u32>) {
    for i in 0..6 {
        println!("= = = = = = = = = = = = = TEST {} = = = = = = = = = = = = =", i);
        println!("vmsize = 32k\tpagesize = 1k\tmemcount = {}", 2 * i + 2);
        println!("OPT\tMISS COUNT: {}\t\tHIT RATE: {:.3}%", opt(page, 2 * i + 2, false), 100.0 - opt(page, 2 * i + 2, false) as f64 / page.len() as f64 * 100.0);
        println!("LRU\tMISS COUNT: {}\t\tHIT RATE: {:.3}%", lru(page, 2 * i + 2, false), 100.0 - lru(page, 2 * i + 2, false) as f64 / page.len() as f64 * 100.0);
        println!("FIFO\tMISS COUNT: {}\t\tHIT RATE: {:.3}%", fifo(page, 2 * i + 2, false), 100.0 - fifo(page, 2 * i + 2, false) as f64 / page.len() as f64 * 100.0);
    }
}

pub fn exp4() {
    let n = 256;
    let address = generate_address(n);
    let page_size = 1024;
    let page = convert_to_page(&address, page_size);

    print_result(&page);
}