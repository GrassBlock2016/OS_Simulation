use std::ops::Add;
use std::ops::Sub;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
pub struct Resource {
    A: u16,
    B: u16,
    C: u16,
}

impl Add for Resource {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            A: self.A + other.A,
            B: self.B + other.B,
            C: self.C + other.C,
        }
    }
}

impl Sub for Resource {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            A: self.A - other.A,
            B: self.B - other.B,
            C: self.C - other.C,
        }
    }
}

impl PartialEq for Resource {
    fn eq(&self, other: &Self) -> bool{
        self.A == other.A && self.B == other.B && self.C == other.C
    }
}

impl PartialOrd for Resource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Attention:
        // While comparing, request must be on the left,
        // need or available must be on the right.
        if self.A == other.A && self.B == other.B && self.C == other.C {
            Some(Ordering::Equal)
        } else if self.A <= other.A && self.B <= other.B && self.C <= other.C {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}", self.A, self.B, self.C)
    }
}

#[allow(non_snake_case)]
impl Resource {
    pub fn new(A: u16, B: u16, C: u16) -> Self {
        Self {
            A, B, C
        }
    }
}

#[derive(Debug, Clone)]
pub struct Process {
    max: Resource,
    allocation: Resource,
    need: Resource,
    finish: bool
}

impl Process {
    pub fn new(max: Resource, allocation: Resource, need: Resource) -> Self {
        Self {
            max, allocation, need, finish: false
        }
    }

    pub fn get_max(&self) -> &Resource {
        &self.max
    }

    pub fn get_allocation(&self) -> &Resource {
        &self.allocation
    }

    pub fn get_need(&self) -> &Resource {
        &self.need
    }

    pub fn get_finish(&self) -> &bool {
        &self.finish
    }

    pub fn set_allocation(&mut self, res: &Resource) {
        self.allocation = res.clone();
    }

    pub fn set_need(&mut self, res: &Resource) {
        self.need = res.clone();
    }

    pub fn set_finish(&mut self, work: &Resource) {
        if self.need <= *work {
            self.finish = true;
        } else {
            self.finish = false;
        }
    }
}