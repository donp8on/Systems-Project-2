#[derive(Clone, Debug)]
pub struct alloacted {
    start: usize,
    size: usize,
    id: u32,
    data_size: usize,
}

impl alloacted {
    pub fn new(start: usize, size: usize, id: u32, data_size: usize) -> Self {
        alloacted {
            start,
            size,
            id,
            data_size,
        }
    }
    
}

pub struct non_allocated {
    start: usize,
    size: usize,
}
