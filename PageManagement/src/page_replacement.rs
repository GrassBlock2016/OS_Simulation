// 实验要求二：页面置换算法的实现与演示
fn print_result(result: &Vec<Vec<Option<&u32>>>, replace_or_not: &Vec<bool>, pages: &Vec<u32>, out_of_range: u32, algor: &str) {
    println!("{}算法所得缺页次数为 {}", algor, out_of_range);
    println!("{}算法缺页率为 {:.4}", algor, out_of_range as f32 / pages.len() as f32);
    println!("页面号序列为：");
    for p in pages {
        print!("{} ", p);
    }
    println!("\n结果数列为：");
    for i in 0..result[0].len() {
        for j in 0..result.len() {
            if result[j][i].is_none() || replace_or_not[j] == false {
                print!("_ ");
            } else {
                print!("{} ", result[j][i].unwrap());
            }
        }
        println!();
    }
    println!();
}

pub fn opt(pages: &Vec<u32>, block_num: u32, print_or_not: bool) -> u32 {
    let mut out_of_range = 0;
    let mut block = vec![vec![None; block_num as usize]; pages.len()];
    let mut replace_or_not = vec![false; pages.len()];

    for (i, p) in pages.iter().enumerate() {
        if i > 0 {
            block[i] = block[i - 1].clone();
        }
        if block[i].contains(&Some(p)) {
            continue;
        } else {
            out_of_range += 1;
            replace_or_not[i] = true;
            if !block[i].contains(&None) {
                let mut max = 0;
                let mut max_index = 0;
                for (j, b) in block[i].iter().enumerate() {
                    let mut flag = false;
                    for k in i + 1..pages.len() {
                        if &Some(&pages[k]) == b {
                            if k > max {
                                max = k;
                                max_index = j;
                            }
                            flag = true;
                            break;
                        }
                    }
                    if !flag {  // 如果在后续页面中找不到该页面，则直接替换
                        max_index = j;
                        break;
                    }
                }
                block[i][max_index] = Some(p);
            } else {
                // 处理开始时内存块中有空位的情况
                for j in 0..block_num as usize {
                    if block[i][j].is_none() {
                        block[i][j] = Some(p);
                        break;
                    }
                }
            }
        }
    }

    if print_or_not == true {
        print_result(&block, &replace_or_not, &pages, out_of_range, "OPT");
    }
    out_of_range
}

pub fn lru(pages: &Vec<u32>, block_num: u32, print_or_not: bool) -> u32 {
    let mut out_of_range = 0;
    let mut block = vec![vec![None; block_num as usize]; pages.len()];
    let mut replace_or_not = vec![false; pages.len()];

    for (i, p) in pages.iter().enumerate() {
        if i > 0 {
            block[i] = block[i - 1].clone();
        }
        if block[i].contains(&Some(p)) {
            continue;
        } else {
            out_of_range += 1;
            replace_or_not[i] = true;
            if !block[i].contains(&None) {
                let mut min = pages.len();
                let mut min_index = 0;
                for (j, b) in block[i].iter().enumerate() {
                    let mut flag = false;
                    for k in (0..i).rev() {
                        if &Some(&pages[k]) == b {
                            if k < min {
                                min = k;
                                min_index = j;
                            }
                            flag = true;
                            break;
                        }
                    }
                    if !flag {  // 如果在前面的页面中找不到该页面，则直接替换
                        min_index = j;
                        break;
                    }
                }
                block[i][min_index] = Some(p);
            } else {
                // 处理开始时内存块中有空位的情况
                for j in 0..block_num as usize {
                    if block[i][j].is_none() {
                        block[i][j] = Some(p);
                        break;
                    }
                }
            }
        }
    }

    if print_or_not == true {
        print_result(&block, &replace_or_not, &pages, out_of_range, "LRU");
    }
    out_of_range
}

pub fn fifo(pages: &Vec<u32>, block_num: u32, print_or_not: bool) -> u32{
    let mut out_of_range = 0;
    let mut block = vec![vec![None; block_num as usize]; pages.len()];  // 对第 i 个页面判断时内存块中的页面
    let mut replace_or_not = vec![false; pages.len()];
    let mut front = 0;  // 记录队首

    for (i, p) in pages.iter().enumerate() {
        if i > 0 {
            block[i] = block[i - 1].clone();
        }
        if block[i].contains(&Some(p)) {
            continue;
        } else {
            replace_or_not[i] = true;
            out_of_range += 1;
            block[i][front] = Some(p);
            front = (front + 1) % block_num as usize;
        }
    }

    if print_or_not == true {
        print_result(&block, &replace_or_not, &pages, out_of_range, "FIFO");
    }
    out_of_range
}

pub fn exp2() {
    println!("请输入页面号序列(用空格隔开）：");
    let mut pages = String::new();
    std::io::stdin().read_line(&mut pages).expect("Failed to read line");
    let pages: Vec<u32> = pages.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("请输入内存块数：");
    let mut block_num = String::new();
    std::io::stdin().read_line(&mut block_num).expect("Failed to read line");
    let block_num: u32 = block_num.trim().parse().unwrap();

    println!("请选择页面置换算法(1.OPT / 2.LRU / 3.FIFO)：");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().unwrap();
    match choice {
        1 => std::mem::drop(opt(&pages, block_num, true)),
        2 => std::mem::drop(lru(&pages, block_num, true)),
        3 => std::mem::drop(fifo(&pages, block_num, true)),
        _ => {
            println!("无效选项，请重新输入\n");
        }
    }
}