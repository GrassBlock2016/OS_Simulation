mod files;
use std::io::Write;
use rand::Rng;

fn main() {
    // Predefined directories
    let mut usr = Vec::new();
    for i in 0..5 {
        let mut files = Vec::new();
        for _ in 0..5 {
            let name = rand::thread_rng().gen_range(0..1000).to_string();
            let length = rand::thread_rng().gen_range(0..1000);
            let protection = (rand::thread_rng().gen_bool(0.5), rand::thread_rng().gen_bool(0.5), rand::thread_rng().gen_bool(0.5));
            let file = files::File::new(name, length, protection);
            files.push(file);
        }
        let username = i.to_string();
        let directory = files::Directory::new(username, files);
        usr.push(directory);
    }

    loop {
        print!("Your name > ");
        std::io::stdout().flush().unwrap();
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).unwrap();

        if let Some(directory) = usr.iter_mut().find(|d| d.get_username() == username.trim()) {
            println!("Welcome, {}!", directory.get_username());
            directory.ls();
            loop {
                print!("Command name > ");
                std::io::stdout().flush().unwrap();
                let mut command = String::new();
                std::io::stdin().read_line(&mut command).unwrap();
                let command = command.trim().parse::<u16>().unwrap();
        
                match command {
                    1 => directory.create(),
                    2 => directory.delete(),
                    3 => directory.open(),
                    4 => {
                        directory.closeall();
                        directory.ls();
                        println!("Good bye!");
                        break;
                    }
                    5 => directory.close(),
                    6 => directory.read(),
                    7 => directory.write(),
                    8 => directory.run(),
                    _ => println!("Command name given is wrong!")
                }
            }
        } else {
            println!("Your name is not in the username table, try again!");
            continue;
        }
    }
}