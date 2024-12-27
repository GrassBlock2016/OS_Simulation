mod process;

fn generate_random_processes(number: u16) -> Vec<process::PCB> {
    let mut processes = Vec::new();
    for i in 1..=number {
        let priority = rand::random::<u16>() % 40;
        let time = rand::random::<u16>() % 10;
        processes.push(process::PCB::new(i, priority, time));
    }
    processes
}

fn main() {
    // test_example
    // let p1 = process::PCB::new(1, 9, 3);
    // let p2 = process::PCB::new(2, 36, 4);
    // let p3 = process::PCB::new(3, 30, 4);
    // let p4 = process::PCB::new(4, 29, 3);
    // let p5 = process::PCB::new(5, 0, 4);
    // let processes = vec![p1, p2, p3];

    println!("Please input the number of processes: ");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).unwrap();
    let number = number.trim().parse::<u16>().unwrap();
    let processes = generate_random_processes(number);
    let mut queue = process::PCBLink::new();

    println!("Choose the scheduling algorithm (1.priority / 2.roundrobin): ");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().parse::<i32>().unwrap();
    match choice {
        1 => queue.schedule(processes, "priority"),
        2 => queue.schedule(processes, "roundrobin"),
        _ => println!("Unknown choice: {}", choice),
    }
}