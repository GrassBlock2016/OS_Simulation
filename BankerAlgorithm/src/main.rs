mod process_resource;

use process_resource as pr;

fn print_initial_table(processes: &[pr::Process], avail: &pr::Resource) {
    println!("\tMax\t\t\tAllocation\t\tNeed\t\t\tAvailbale");
    for _ in 0..4 {
        print!("\tA\tB\tC");
    }
    print!("\n");
    for (index, p) in processes.iter().enumerate() {
        print!("p{}\t", index);
        print!("{}\t{}\t{}\t", p.get_max(), p.get_allocation(), p.get_need());
        if index == 0 {
            print!("{}", avail);
        }
        print!("\n");
    }
}

fn requested_banker(processes: &mut [pr::Process], requestor: usize, request: pr::Resource, avail: &mut pr::Resource) {
    // Check if there's a safe sequence after request
    if requestor > processes.len() {
        println!("Invalid requestor.");
    }
    if request <= *processes[requestor].get_need() {
        println!("(1) Request <= Need");
        if request <= *avail {
            println!("(2) Request <= Available");
            processes[requestor].set_allocation(&(*processes[requestor].get_allocation() + request));
            processes[requestor].set_need(&(*processes[requestor].get_need() - request));
            *avail = *avail - request;
            banker(processes, &avail);
        }
        else {
            println!("(2) Request > Available, P{} must wait.", requestor);
        }
    }
    else {
        println!("(1) Request > Need, not allowed to allocate.");
    }
}

fn banker(processes: &mut [pr::Process], avail: &pr::Resource) {
    // Banker's algorithm
    let mut work = avail.clone();
    let mut allocated_any = false;
    for p in processes.iter_mut() {
        if p.get_need() <= &work {
            allocated_any = true;
            break;
        }
    }
    if !allocated_any {
        println!("No safe sequence. Not allowed to allocate the resource");
        return;
    }
    println!("\tWork\t\t\tNeed\t\t\tAllocation\t\tWork+Allocation\t\tFinish");
    for _ in 0..4 {
        print!("\tA\tB\tC");
    }
    print!("\n");
    loop {
        let mut finish = false;
        for (index, p) in processes.iter_mut().enumerate() {
            if !*p.get_finish() && p.get_need() <= &work {
                print!("p{}\t", index);
                print!("{}\t{}\t{}\t", work, p.get_need(), p.get_allocation());
                let new_work = work.clone() + p.get_allocation().clone();
                print!("{}\t", new_work);
                p.set_finish(&new_work);
                work = new_work;
                finish = true;
                print!("{}\t", p.get_finish());
                print!("\n");
            }
        }
        if !finish {
            break;
        }
    }
}

fn main() {
    let p0 = pr::Process::new(
        pr::Resource::new(7, 5, 3), 
        pr::Resource::new(0, 1, 0), 
        pr::Resource::new(7, 4, 3));
    let p1 = pr::Process::new(
        pr::Resource::new(3, 2, 2), 
        pr::Resource::new(2, 0, 0), 
        pr::Resource::new(1, 2, 2));
    let p2 = pr::Process::new(
        pr::Resource::new(9, 0, 2), 
        pr::Resource::new(3, 0, 2), 
        pr::Resource::new(6, 0, 0));
    let p3 = pr::Process::new(
        pr::Resource::new(2, 2, 2), 
        pr::Resource::new(2, 1, 1), 
        pr::Resource::new(0, 1, 1));
    let p4 = pr::Process::new(
        pr::Resource::new(4, 3, 3), 
        pr::Resource::new(0, 0, 2), 
        pr::Resource::new(4, 3, 1));
    let available = pr::Resource::new(3, 3, 2);
    let processes = vec![p0, p1, p2, p3, p4];
    
    println!("最初资源分配情况如下：");
    print_initial_table(&processes, &available);

    println!("一、例题演示");
    println!("二、自由分配");
    println!("三、直接检查安全性");
    println!("请选择选项（1-3）：");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().parse::<u8>().unwrap();
    
    match choice {
        1 => {
            println!("1. 检查安全性");
            banker(&mut processes.clone(), &available);
            println!("\n2. P1 请求资源：P1 发出请求向量 Requests(1, 0 ,2)");
            let mut processes2 = processes.clone();
            let mut available2 = available.clone();
            requested_banker(&mut processes2, 1, pr::Resource::new(1, 0, 2), &mut available2);
            println!("\n3. 在上一题基础之上，P4 请求资源：P4 发出请求向量 Requests(3, 3, 0)");
            requested_banker(&mut processes2, 4, pr::Resource::new(3, 3, 0), &mut available2);
            println!("\n4. 在上一题基础之上，P0 请求资源：P0 发出请求向量 Requests(0, 2, 0)");
            requested_banker(&mut processes2, 4, pr::Resource::new(0, 2, 0), &mut available2);
        }
        2 => {
            println!("请输入要请求资源的进程（0-4）：");
            let mut process_index = String::new();
            std::io::stdin().read_line(&mut process_index).unwrap();
            let process_index = process_index.trim().parse::<usize>().unwrap();
            println!("请输入请求的资源（A B C，使用空格隔开）：");
            let mut resources = String::new();
            std::io::stdin().read_line(&mut resources).unwrap();
            let resources: Vec<u16> = resources
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            if resources.len() != 3 {
                println!("Invalid resource input.");
            } else {
                let request = pr::Resource::new(resources[0], resources[1], resources[2]);
                requested_banker(&mut processes.clone(), process_index, request, &mut available.clone());
            }
        }
        3 => banker(&mut processes.clone(), &available),
        _ => println!("Unknown choice: {}", choice),
    }
}
