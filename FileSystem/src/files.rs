use std::io::Write;

#[derive(Clone)]
pub struct File {
    name: String,
    length: u64,
    protection: (bool, bool, bool), // read, write, execute
}

impl File {
    pub fn new(name: String, length: u64, protection: (bool, bool, bool)) -> Self {
        File {
            name,
            length,
            protection,
        }
    }

    pub fn read(&self) {
        if self.protection.0 {
            println!("This file can be read.");
        } else {
            println!("Permission denied: this file cannot be read!");
        }
    }

    pub fn write(&self) {
        if self.protection.1 {
            println!("This file can be written.");
        } else {
            println!("Permission denied: this file cannot be written!");
        }
    }

    pub fn run(&self) {
        if self.protection.2 {
            println!("This file can be executed.");
        } else {
            println!("Permission denied: this file cannot be executed!");
        }
    }
}

// each user has only a directory
pub struct Directory {
    username: String,
    files: Vec<File>,
    opened: Vec<File>,
}

impl Directory {
    pub fn new(username: String, files: Vec<File>) -> Self {
        Directory {
            username,
            files,
            opened: Vec::new(),
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn create(&mut self) {
        print!("The new file's name > ");
        std::io::stdout().flush().unwrap();
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();

        if self.files.iter().any(|file| file.name == name) {
            println!("The file already exists!");
            return;
        }
        
        print!("The new file's code length > ");
        std::io::stdout().flush().unwrap();
        let mut length = String::new();
        std::io::stdin().read_line(&mut length).unwrap();
        let length = length.trim().parse::<u64>().unwrap();

        print!("The new file's protection > ");
        std::io::stdout().flush().unwrap();
        let mut protection = String::new();
        std::io::stdin().read_line(&mut protection).unwrap();
        let protection = match protection.trim() {
            "000" => (false, false, false),
            "001" => (false, false, true),
            "010" => (false, true, false),
            "011" => (false, true, true),
            "100" => (true, false, false),
            "101" => (true, false, true),
            "110" => (true, true, false),
            "111" => (true, true, true),
            _ => panic!("Invalid protection code!"),
        };

        self.files.push(File::new(name, length, protection));
        println!("The new file is created successfully!");
        self.ls();
    }

    pub fn delete(&mut self) {
        print!("The name of file to delete > ");
        std::io::stdout().flush().unwrap();
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        if self.files.iter().any(|file| file.name == name) {
            self.files.retain(|file| file.name != name);
            println!("The file is deleted successfully!");
            self.ls();
        } else {
            println!("The file does not exist!");
        }
    }

    pub fn open(&mut self) {
        print!("The name of file to open > ");
        std::io::stdout().flush().unwrap();
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        print!("Enter the open mode > ");
        std::io::stdout().flush().unwrap();
        let mut mode = String::new();
        std::io::stdin().read_line(&mut mode).unwrap();
        let mode = match mode.trim() {
            "000" => (false, false, false),
            "001" => (false, false, true),
            "010" => (false, true, false),
            "011" => (false, true, true),
            "100" => (true, false, false),
            "101" => (true, false, true),
            "110" => (true, true, false),
            "111" => (true, true, true),
            _ => panic!("Invalid mode code!"),
        };

        if self.files.iter().any(|file| file.name == name) {
            let file = self.files.iter().find(|file| file.name == name).unwrap();
            if (!file.protection.0 && mode.0) || (!file.protection.1 && mode.1) || (!file.protection.2 && mode.2) {
                if !file.protection.0 && mode.0 {
                    println!("Permission denied: this file cannot be read!");
                }
                if !file.protection.1 && mode.1 {
                    println!("Permission denied: this file cannot be written!");
                }
                if !file.protection.2 && mode.2 {
                    println!("Permission denied: this file cannot be executed!");
                }
                return;
            }
            self.opened.push(file.clone());
            println!("The file is opened, its open number is {}", self.opened.len());
        } else {
            println!("The file does not exist!");
        }
    }

    pub fn close(&mut self) {
        print!("The name of file to close > ");
        std::io::stdout().flush().unwrap();
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        if self.opened.iter().any(|file| file.name == name) {
            self.opened.retain(|file| file.name != name);
            println!("The file is closed.");
        } else {
            println!("The file is not opened!");
        }
    }

    pub fn read(&self) {
        print!("The name of file to read > ");
        std::io::stdout().flush().unwrap();
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        if self.opened.iter().any(|file| file.name == name) {
            let file = self.opened.iter().find(|file| file.name == name).unwrap();
            file.read();
        } else {
            println!("The file is not opened!");
        }
    }

    pub fn write(&self) {
        print!("The name of file to write > ");
        std::io::stdout().flush().unwrap();
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        if self.opened.iter().any(|file| file.name == name) {
            let file = self.opened.iter().find(|file| file.name == name).unwrap();
            file.write();
        } else {
            println!("The file is not opened!");
        }
    }

    pub fn run(&self) {
        print!("The name of file to run > ");
        std::io::stdout().flush().unwrap();
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        if self.opened.iter().any(|file| file.name == name) {
            let file = self.opened.iter().find(|file| file.name == name).unwrap();
            file.run();
        } else {
            println!("The file is not opened!");
        }
    }

    pub fn closeall(&mut self) {
        self.opened.clear();
    }

    pub fn ls(&self) {
        println!("= = = = = = = = = Your Directory = = = = = = = = =");
        println!("File name\tCode length\tProtection");
        for file in self.files.iter() {
            print!("{}\t\t{}\t\t", file.name, file.length);
            if file.protection.0 {
                print!("1");
            } else {
                print!("0");
            }
            if file.protection.1 {
                print!("1");
            } else {
                print!("0");
            }
            if file.protection.2 {
                print!("1");
            } else {
                print!("0");
            }
            println!();
        }
        println!("The commands are as follows: ");
        println!("1. create, 2. delete, 3. open, 4. bye, 5. close");
        println!("6. read, 7. write, 8. run");
    }
}