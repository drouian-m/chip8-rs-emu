const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4096;

pub struct Display {
    pub ram: [u8; 2048],
}

impl Display {
    pub fn new() -> Display {
        Display {
            ram: [0; RAM_SIZE]
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, active: bool) {
        self.ram[x + y + SCREEN_WIDTH] = active;
    }
}