const MAX: usize = 65535 as usize;

struct Mem {
    input: String,
    arr: [u8; MAX],
    ptr: usize,
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
        self.arr[self.ptr] = match self.arr[self.ptr] {
            255 => 0,
            _ => self.arr[self.ptr] + 1,
        }
    }
    
    fn val_minus(&mut self) {
        self.arr[self.ptr] = match self.arr[self.ptr] {
            0 => 255,
            _ => self.arr[self.ptr] - 1,
        }
    }

    fn ptr_print(&mut self) {
        print!(
            "{}",
            std::char::from_u32(self.arr[self.ptr] as u32).unwrap()
        );
    }

    fn stdin_read(&mut self) {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        self.arr[self.ptr as usize] = buf.chars().nth(0).unwrap().to_digit(10).unwrap() as u8;
    }

    fn find_right(&mut self, mut idx: usize) -> usize {
        let mut left_num = 0usize;
        let mut right_num = 0usize;

        while {
            match (
                *&self.input[self.input.char_indices().nth(idx as usize).unwrap().0
                    ..self.input.char_indices().last().unwrap().0]
                    .chars()
                    .position(|p| p == '['),
                *&self.input[self.input.char_indices().nth(idx as usize).unwrap().0
                    ..self.input.char_indices().last().unwrap().0]
                    .chars()
                    .position(|p| p == ']'),
            ) {
                (Some(l_idx), Some(r_idx)) => {
                    if l_idx < r_idx {
                        left_num += 1;
                        idx += l_idx;
                    } else {
                        right_num += 1;
                        idx += r_idx;
                    }
                }
                (None, Some(r_idx)) => {
                    right_num += 1;
                    idx += r_idx;
                }
                (_, None) => panic!("カッコの数が対応しません"),
            }

            // let left_idx = idx + *&self.input[self.input.char_indices().nth(idx as usize).unwrap().0
            //     ..self.input.char_indices().last().unwrap().0]
            //     .chars()
            //     .position(|p| p == ']')
            //     .unwrap() as u16;

            // let right_idx =  idx + *&self.input[self.input.char_indices().nth(idx as usize).unwrap().0
            //     ..self.input.char_indices().last().unwrap().0]
            //     .chars()
            //     .position(|p| p == '[')
            //     .unwrap() as u16;

            left_num + 1 != right_num
        } {}

        idx
    }

    fn find_left(&mut self, mut idx: usize) -> usize {
        let mut left_num = 0usize;
        let mut right_num = 0usize;

        while {
            match (
                *&self.input[self.input.char_indices().nth(0).unwrap().0
                    ..self.input.char_indices().nth(idx).unwrap().0]
                    .chars()
                    .rev()
                    .position(|p| p == '['),
                *&self.input[self.input.char_indices().nth(0).unwrap().0
                    ..self.input.char_indices().nth(idx).unwrap().0]
                    .chars()
                    .rev()
                    .position(|p| p == ']'),
            ) {
                (Some(l_idx), Some(r_idx)) => {
                    if l_idx > r_idx {
                        right_num += 1;
                        idx -= r_idx;
                        idx -= 1;
                    } else {
                        left_num += 1;
                        idx -= l_idx;
                        idx -= 1;
                    }
                }
                (Some(l_idx), None) => {
                    left_num += 1;
                    idx -= l_idx;
                    idx -= 1;
                }
                (None, _) => panic!("カッコの数が対応しません"),
            }
            left_num != right_num + 1
        } {}
        // idx - *&self.input[self.input.char_indices().nth(0).unwrap().0
        //     ..self.input.char_indices().nth(*idx as usize).unwrap().0]
        //     .chars()
        //     .rev()
        //     .position(|p| p == '[')
        //     .unwrap()
        //     .clone() as u16 - 1;
        idx
    }

    fn run(&mut self) {
        let mut idx = 0usize;
        while idx != self.input.chars().enumerate().last().unwrap().0 + 1 {
            match self.input.chars().nth(idx as usize).unwrap() {
                '>' => self.ptr_plus(),
                '<' => self.ptr_minus(),
                '+' => self.val_plus(),
                '-' => self.val_minus(),
                '.' => self.ptr_print(),
                ',' => self.stdin_read(),
                '[' => {
                    if self.arr[self.ptr as usize] == 0 {
                        idx = self.find_right(idx);
                    }
                }
                ']' => {
                    if self.arr[self.ptr as usize] != 0 {
                        idx = self.find_left(idx);
                    }
                }
                _ => (),
            }
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

    mem = Mem::init(buf);

    mem.run();
    // }
}
