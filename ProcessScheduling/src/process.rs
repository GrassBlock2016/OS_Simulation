use std::collections::LinkedList;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Clone)]
pub enum ProcessState {
    Run,    // 运行态
    Wait,   // 就绪态
    Finish, // 完成态
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PCB {
    pid: u16,               // 进程标识符
    priority: u16,          // 优先级/轮转时间片数
    cpu_time: u16,          // 占用 CPU 时间片数
    need_time: u16,         // 进程所需时间片数
    state: ProcessState,    // 进程状态
}

impl PCB {
    pub fn new(pid: u16, priority: u16, need_time: u16) -> Self {
        if need_time == 0 {
            Self {
                pid,
                priority,
                cpu_time: 0,
                need_time,
                state: ProcessState::Finish,
            }
        } else {
            Self {
                pid,
                priority,
                cpu_time: 0,
                need_time,
                state: ProcessState::Wait,
            }
        }
    }
}

pub struct PCBLink {
    ready_queue: LinkedList<Rc<RefCell<PCB>>>,
    all_processes: LinkedList<Rc<RefCell<PCB>>>,
}

impl PCBLink {
    pub fn new() -> Self {
        Self {
            ready_queue: LinkedList::new(),
            all_processes: LinkedList::new(),
        }
    }

    fn push_priority(&mut self, pcb: PCB) {
        let pcb = Rc::new(RefCell::new(pcb));
        self.all_processes.push_back(Rc::clone(&pcb));
        if self.ready_queue.is_empty() {
            self.ready_queue.push_back(pcb);
        } else {
            let mut inserted = false;
            let mut temp_queue = LinkedList::new();

            while let Some(front) = self.ready_queue.pop_front() {
                if front.borrow().priority < pcb.borrow().priority && !inserted {
                    temp_queue.push_back(Rc::clone(&pcb));
                    inserted = true;
                }
                temp_queue.push_back(front);
            }
            if !inserted {
                temp_queue.push_back(pcb);
            }
            self.ready_queue = temp_queue;
        }
    }

    fn push_roundrobin(&mut self, pcb: PCB) {
        let pcb = Rc::new(RefCell::new(pcb));
        self.all_processes.push_back(Rc::clone(&pcb));
        self.ready_queue.push_back(pcb);
    }

    pub fn print_status(&self, run: &Rc<RefCell<PCB>>) {
        println!("= = = = = = = = = = = = = = = = = = = = = = = = = = = = = =");
        println!("RUNNING PROC.               WAITING QUEUE");
        print!("{}                           ", run.borrow().pid);
        for process in &self.ready_queue {
            print!("{}      ", process.borrow().pid);
        }
        println!("\n= = = = = = = = = = = = = = = = = = = = = = = = = = = = = =");
        print!("ID\t\t");
        for process in &self.all_processes {
            print!("{}\t", process.borrow().pid);
        }
        print!("\nPRIORITY\t");
        for process in &self.all_processes {
            print!("{}\t", process.borrow().priority);
        }
        print!("\nCPUTIME\t\t");
        for process in &self.all_processes {
            print!("{}\t", process.borrow().cpu_time);
        }
        print!("\nALLTIME\t\t");
        for process in &self.all_processes {
            print!("{}\t", process.borrow().need_time);
        }
        print!("\nSTATE\t\t");
        for process in &self.all_processes {
            print!("{:?}\t", process.borrow().state);
        }
        print!("\nNEXT\t\t");
        for process in &self.all_processes {
            let next_pid = self.ready_queue.iter()
                .skip_while(|p| p.borrow().pid != process.borrow().pid)
                .nth(1)
                .map_or(0, |next| next.borrow().pid);
            print!("{}\t", next_pid);
        }
        println!("\n= = = = = = = = = = = = = = = = = = = = = = = = = = = = = =");
        print!("Press Enter to continue...\n");
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    pub fn schedule(&mut self, mut processes: Vec<PCB>, algo: &str) {
        match algo {
            "priority" => {
                for process in processes.drain(..) {
                    self.push_priority(process);
                }
                self.run_priority();
            }
            "roundrobin" => {
                for process in processes.drain(..) {
                    self.push_roundrobin(process);
                }
                self.run_roundrobin();
            }
            _ => println!("Unknown scheduling algorithm"),
        }
    }

    fn run_priority(&mut self) {
        println!("OUTPUT OF PRIORITY");
        while let Some(run) = self.ready_queue.pop_front() {
            run.borrow_mut().state = ProcessState::Run;

            while run.borrow().need_time > 0 {
                run.borrow_mut().need_time -= 1;
                run.borrow_mut().cpu_time += 1;
                // run.borrow_mut().priority -= 3;  u16 无法为负数
                if run.borrow().priority >= 3 {
                    run.borrow_mut().priority -= 3;
                } else {
                    run.borrow_mut().priority = 0;
                }
                self.print_status(&run);
                
                if let Some(next) = self.ready_queue.front() {
                    if next.borrow().priority > run.borrow().priority {
                        run.borrow_mut().state = ProcessState::Wait;
                        // 将当前进程插入到就绪队列中，并按优先级排序
                        let mut temp_queue = LinkedList::new();
                        let mut inserted = false;
                        while let Some(front) = self.ready_queue.pop_front() {
                            if front.borrow().priority < run.borrow().priority && !inserted {
                                temp_queue.push_back(Rc::clone(&run));
                                inserted = true;
                            }
                            temp_queue.push_back(front);
                        }
                        if !inserted {
                            temp_queue.push_back(Rc::clone(&run));
                        }
                        self.ready_queue = temp_queue;
                        break;
                    }
                }
            }
            if run.borrow().need_time == 0 {
                run.borrow_mut().state = ProcessState::Finish;
            }
        }
        println!("All processes finished");
    }

    fn run_roundrobin(&mut self) {
        println!("OUTPUT OF ROUND ROBIN");
        while let Some(run) = self.ready_queue.pop_front() {
            run.borrow_mut().state = ProcessState::Run;

            if run.borrow().need_time > 0 {
                run.borrow_mut().need_time -= 1;
                run.borrow_mut().cpu_time += 1;
                self.print_status(&run);
                run.borrow_mut().state = ProcessState::Wait;
                self.ready_queue.push_back(run);
            } else {
                run.borrow_mut().state = ProcessState::Finish;
            }
        }
        println!("All processes finished");
    }
}