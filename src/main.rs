const MAX: usize = u16::MAX as usize;
#[allow(dead_code)]
const MIN: usize = u16::MIN as usize;

struct Mem {
    input: String,
    arr: [u8; MAX],
    ptr: u16,
}

impl Mem {
    fn init(input: String) -> Self {
        Mem {
            input,
            arr: [0; MAX],
            ptr: 0,
        }
    }

    fn ptr_plus(&mut self) {
        self.ptr += 1;
    }

    fn ptr_minus(&mut self) {
        self.ptr -= 1;
    }

    fn val_plus(&mut self) {
        self.arr[self.ptr as usize] += 1;
    }

    fn val_minus(&mut self) {
        self.arr[self.ptr as usize] -= 1;
    }

    fn ptr_print(&mut self) {
        print!(
            "{}",
            std::char::from_u32(self.arr[self.ptr as usize] as u32).unwrap()
        );
    }

    fn stdin_read(&mut self) {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        self.arr[self.ptr as usize] = buf.chars().nth(0).unwrap().to_digit(10).unwrap() as u8;
    }

    
    fn find_right(&mut self, idx: &u16) -> u16 {
        idx + *&self.input[self.input.char_indices().nth(*idx as usize).unwrap().0
            ..self.input.char_indices().last().unwrap().0]
            .chars()
            .position(|p| p == ']')
            .unwrap()
            .clone() as u16
    }


    fn find_left(&mut self, idx: &u16) -> u16 {
        idx - *&self.input[self.input.char_indices().nth(0).unwrap().0
            ..self.input.char_indices().nth(*idx as usize).unwrap().0]
            .chars()
            .rev()
            .position(|p| p == '[')
            .unwrap()
            .clone() as u16 - 1
    }

    fn run(&mut self) {
        let mut idx = 0u16;
        while idx != self.input.chars().enumerate().last().unwrap().0 as u16 + 1{
            match self.input.chars().nth(idx as usize).unwrap() {
                '>' => self.ptr_plus(),
                '<' => self.ptr_minus(),
                '+' => self.val_plus(),
                '-' => self.val_minus(),
                '.' => self.ptr_print(),
                ',' => self.stdin_read(),
                '[' => {
                    if self.arr[self.ptr as usize] == 0 {
                        idx = self.find_right(&idx);
                    }
                }
                ']' => {
                    if self.arr[self.ptr as usize] != 0 {
                        idx = self.find_left(&idx)
                    }
                }
                _ => (),
            }
            // println!("{}", idx);
            idx += 1;
        }
        print!("\n");
    }
}

fn main() {
    let mut mem: Mem;
    // loop {
        println!("BF>");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        // if buf.trim().to_string() == String::from("exit") {
        //     break;
        // }

        mem = Mem::init(buf);

        mem.run();
    // }
}
