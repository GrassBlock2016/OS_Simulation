pub struct Area {
    start: u16,                 // 起址
    size: u16,                  // 空闲大小
    status: Option<String>,     // 状态, None 代表空闲分区，Some(T) 代表这里分配了作业 T
}

impl Area {
    pub fn new(start: u16, size: u16, status: Option<String>) -> Self {
        Area {
            start,
            size,
            status,
        }
    }
}

pub struct Memory {
    areas: Vec<Area>,
    capacity: u16,
    last_alloc_index: usize, // 记录上次分配的位置
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            areas: Vec::new(),
            capacity: 640,
            last_alloc_index: 0,
        }
    }

    pub fn init(&mut self) {
        self.areas.push(Area::new(0, self.capacity, None));
    }

    pub fn first_fit(&mut self, name: &str, size: u16) {
        let mut insert_index = None;
        let mut new_area_start = 0;
        // let mut new_area_size = 0;
    
        for (i, area) in self.areas.iter_mut().enumerate() {
            if area.status.is_none() && area.size >= size {
                insert_index = Some(i);
                new_area_start = area.start;
                // new_area_size = area.size;
                break;
            }
        }
        
        if insert_index.is_none() {
            println!("内存不足，无法为作业 {} 分配 {} KB 空间", name, size);
        }
        else if let Some(index) = insert_index {
            self.areas.insert(index, Area::new(new_area_start, size, Some(name.to_string())));
            self.areas[index + 1].start += size;
            self.areas[index + 1].size -= size;
        }
    }

    pub fn next_fit(&mut self, name: &str, size: u16) {
        let mut found = false;
        let mut new_area_start;

        // 从上次分配位置开始查找
        for i in self.last_alloc_index..self.areas.len() {
            let area = &mut self.areas[i];
            if area.status.is_none() && area.size >= size {
                self.last_alloc_index = i; // 更新最后分配位置
                new_area_start = area.start;
                self.areas.insert(i, Area::new(new_area_start, size, Some(name.to_string())));
                self.areas[i + 1].start += size;
                self.areas[i + 1].size -= size;
                found = true;
                break;
            }
        }

        // 如果没有找到可用分区，从头开始继续查找
        if !found {
            for i in 0..self.last_alloc_index {
                let area = &mut self.areas[i];
                if area.status.is_none() && area.size >= size {
                    self.last_alloc_index = i; // 更新最后分配位置
                    new_area_start = area.start;
                    self.areas.insert(i, Area::new(new_area_start, size, Some(name.to_string())));
                    self.areas[i + 1].start += size;
                    self.areas[i + 1].size -= size;
                    found = true;
                    break;
                }
            }
        }

        if !found {
            println!("内存不足，无法为作业 {} 分配 {} KB 空间", name, size);
        }
    }

    pub fn best_fit(&mut self, name: &str, size: u16) {
        let mut insert_index = None;
        let mut new_area_start = 0;
        let mut new_area_size = 0;

        for (i, area) in self.areas.iter_mut().enumerate() {
            if area.status.is_none() && area.size >= size {
                if insert_index.is_none() || area.size < new_area_size {
                    insert_index = Some(i);
                    new_area_start = area.start;
                    new_area_size = area.size;
                }
            }
        }

        if insert_index.is_none() {
            println!("内存不足，无法为作业 {} 分配 {} KB 空间", name, size);
        }
        else if let Some(index) = insert_index {
            self.areas.insert(index, Area::new(new_area_start, size, Some(name.to_string())));
            self.areas[index + 1].start += size;
            self.areas[index + 1].size -= size;
        }
    }

    pub fn worst_fit(&mut self, name: &str, size: u16) {
        let mut insert_index = None;
        let mut new_area_start = 0;
        let mut new_area_size = 0;

        for (i, area) in self.areas.iter_mut().enumerate() {
            if area.status.is_none() && area.size >= size {
                if insert_index.is_none() || area.size > new_area_size {
                    insert_index = Some(i);
                    new_area_start = area.start;
                    new_area_size = area.size;
                }
            }
        }

        if insert_index.is_none() {
            println!("内存不足，无法为作业 {} 分配 {} KB 空间", name, size);
        }
        else if let Some(index) = insert_index {
            self.areas.insert(index, Area::new(new_area_start, size, Some(name.to_string())));
            self.areas[index + 1].start += size;
            self.areas[index + 1].size -= size;
        }
    }

    pub fn release(&mut self, name: &str) {
        let mut release_index = None;
    
        for (i, area) in self.areas.iter_mut().enumerate() {
            if Some(name.to_string()) == area.status {
                release_index = Some(i);
                area.status = None;
                break;
            }
        }
    
        if release_index.is_none() {
            println!("未找到名为 {} 的作业", name);
            return;
        }
    
        if let Some(index) = release_index {
            // 检查是否可以与前后的空闲分区合并
            if index > 0 && index < self.areas.len() - 1 && self.areas[index - 1].status.is_none() && self.areas[index + 1].status.is_none() {
                self.areas[index - 1].size += self.areas[index].size + self.areas[index + 1].size;
                self.areas.remove(index);
                self.areas.remove(index); // 此时 index 指向原来 index+1 的位置
            } else if index > 0 && self.areas[index - 1].status.is_none() {
                self.areas[index - 1].size += self.areas[index].size;
                self.areas.remove(index);
            } else if index < self.areas.len() - 1 && self.areas[index + 1].status.is_none() {
                self.areas[index].size += self.areas[index + 1].size;
                self.areas.remove(index + 1);
            }
    
            // 更新 last_alloc_index
            if self.last_alloc_index >= self.areas.len() {
                // 如果 last_alloc_index 越界，重置为 0
                self.last_alloc_index = 0;
            } else if self.last_alloc_index > index {
                // 如果 last_alloc_index 在释放的分区之后，向前移动
                self.last_alloc_index -= 1;
            }
        }
    }

    pub fn print_table(&self) {
        println!("*******************分区列表*******************");
        println!("分区号\t分区大小(KB)\t分区始址(KB)\t状态");
        for (i, area) in self.areas.iter().enumerate() {
            println!("{}\t{}\t\t{}\t\t{}", i + 1, area.size, area.start, if area.status.is_none() { "未分配" } else { "已分配" });
        }
        println!();
    }
}