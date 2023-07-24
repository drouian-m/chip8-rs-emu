const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;

pub struct Ram {
    ram: [u8; RAM_SIZE],
    sp: u16,
    stack: [u16; STACK_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            sp: 0,
            stack: [0; STACK_SIZE],
        }
    }

    pub fn reset(&mut self) {
        self.ram = [0; RAM_SIZE];
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
    }

    pub fn read(&self, index: usize) -> u8 {
        self.ram[index]
    }

    pub fn write(&mut self, index: usize, value: u8) {
        self.ram[index] = value
    }

    pub fn push_arr(&mut self, items: &[u8]) {
        self.ram[..items.len()].copy_from_slice(items);
    }

    pub fn push_at(&mut self, items: &[u8], index: usize) {
        let start = index;
        let end = index + items.len();
        self.ram[start..end].copy_from_slice(items);
    }

    pub fn stack_push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    pub fn stack_pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::ram::Ram;

    #[test]
    fn test_put_value_in_stack() {
        let mut ram = Ram::new();
        ram.stack_push(5);
        assert_eq!(ram.sp, 1, "Stack is incremented by one");
    }

    #[test]
    fn test_pop_value_from_stack() {
        let mut ram = Ram::new();
        ram.stack_push(5);
        ram.stack_push(4);
        let val = ram.stack_pop();
        assert_eq!(val, 4, "Last value pushed in stack is {}", 4);
        assert_eq!(ram.sp, 1, "Stack is decremented by one");
    }

    #[test]
    fn read_empty_ram_value() {
        let ram = Ram::new();
        let val = ram.read(5);
        assert_eq!(val, 0, "Read value is equal to zero because it has not be set");
    }

    #[test]
    fn read_filled_ram_value() {
        let mut ram = Ram::new();
        ram.write(5, 4);
        let val = ram.read(5);
        assert_eq!(val, 4, "Value at index {} is {}", 5, 4);
    }

    #[test]
    fn push_array() {
        let mut ram = Ram::new();
        ram.push_arr(&[2, 4, 6, 8]);
        let first = ram.read(0);
        let last = ram.read(3);
        assert_eq!(first, 2, "First array value is at index {} is {}", 0, 2);
        assert_eq!(last, 8, "First array value is at index {} is {}", 3, 8);
    }

    #[test]
    fn push_array_at_index() {
        let mut ram = Ram::new();
        ram.push_at(&[2, 4, 6, 8], 10);
        let first = ram.read(10);
        let last = ram.read(13);
        assert_eq!(first, 2, "First array value is at index {} is {}", 10, 2);
        assert_eq!(last, 8, "First array value is at index {} is {}", 13, 8);
    }

    #[test]
    fn reset_ram() {
        let mut ram = Ram::new();
        ram.stack_push(5);
        ram.push_arr(&[2, 4, 6, 8]);
        ram.reset();
        assert_eq!(ram.sp, 0, "Stack is empty");
        assert_eq!(ram.read(2), 0, "Ram is empty");
    }
}
