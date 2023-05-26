pub struct CircularBuffer<T> {
    capacity: usize,
    r_head: usize,
    w_head: usize,
    buffer: Vec<Option<T>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize_with(capacity + 1, || None);
        Self {
            capacity,
            r_head: 0,
            w_head: 0,
            buffer,
        }
    }

    fn next_idx(&self, idx: usize) -> usize {
        (idx + 1) % (self.capacity + 1)
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        self.w_head = match self.next_idx(self.w_head) {
            idx if idx == self.r_head => return Err(Error::FullBuffer),
            idx => idx,
        };
        Ok(self.buffer[self.w_head] = Some(element))
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.r_head = match self.r_head == self.w_head {
            true => return Err(Error::EmptyBuffer),
            _ => self.next_idx(self.r_head),
        };
        Ok(std::mem::take(&mut self.buffer[self.r_head]).unwrap())
    }

    pub fn clear(&mut self) {
        self.r_head = 0;
        self.w_head = 0;
        self.buffer.fill_with(|| None);
    }

    pub fn overwrite(&mut self, element: T) {
        if self.next_idx(self.w_head) == self.r_head {
            self.r_head = self.next_idx(self.r_head);
        }
        self.write(element).unwrap();
    }
}
