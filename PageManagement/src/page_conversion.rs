// 实验要求一：指令地址流→页号的转换
use rand;

pub fn generate_address(n: u32) -> Vec<u32> {
    let mut address = Vec::new();
    for _ in 0..n {
        address.push(rand::random::<u32>() % 100000);
    }
    address
}

pub fn convert_to_page(address: &Vec<u32>, page_size: u32) -> Vec<u32> {
    let mut page = Vec::new();
    for i in address {
        page.push(i / page_size);
    }
    page
}

pub fn exp1() {
    let n = 256;
    let address = generate_address(n);
    let page_size = 1024;
    let page = convert_to_page(&address, page_size);
    
    for (i, a) in address.iter().enumerate() {
        print!("a[{}]={}", i, a);
        if i % 10 == 9 {
            println!();
        } else {
            print!(", ");
        }
    }
    println!();

    for (i, p) in page.iter().enumerate() {
        print!("page[{}]={}", i, p);
        if i % 10 == 9 {
            println!();
        } else {
            print!(", ");
        }
    }
    println!();
}