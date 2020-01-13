fn input_line(buffer: &mut [u16]) {
    let mut text = String::new();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");
    let text_size = text.len().min(buffer.len());
    for (i, word) in buffer.iter_mut().enumerate().take(text_size) {
        *word = text.as_bytes()[i].into();
    }
    for word in buffer.iter_mut().skip(text.len()) {
        *word = 0;
    }
}

fn execute(program: &[u16]) -> u16 {
    let mut acc: u16 = 0;
    let mut process = vec![0u16; program[0] as usize];
    process[..program.len()].copy_from_slice(program);
    let mut ip = 1;
    loop {
        let opcode = process[ip];
        let operand = process[ip + 1];
        //println!("ip: {} opcode: {} operand: {} acc: {}",
        //ip, opcode, operand, acc);
        ip += 2;
        match opcode {
            0 =>
            // terminate
            {
                return operand
            }
            1 =>
            // set
            {
                acc = operand
            }
            2 =>
            // load
            {
                acc = process[operand as usize]
            }
            3 =>
            // store
            {
                process[operand as usize] = acc
            }
            4 => {
                // indirect_load
                let address = process[operand as usize] as usize;
                acc = process[address];
            }
            5 => {
                // indirect_store
                let address = process[operand as usize] as usize;
                process[address] = acc;
            }
            6 => {
                // input
                let address = acc as usize;
                input_line(&mut process[address..address + operand as usize]);
            }
            7 => {
                // output
                let address = acc as usize;
                for &word in &process[address..address + operand as usize] {
                    print!("{}", if word == 0 { ' ' } else { word as u8 as char });
                }
            }
            8 =>
            // add
            {
                acc = acc.wrapping_add(process[operand as usize])
            }
            9 =>
            // subtract
            {
                acc = acc.wrapping_sub(process[operand as usize])
            }
            10 =>
            // multiply
            {
                acc = acc.wrapping_mul(process[operand as usize])
            }
            11 =>
            // divide
            {
                acc = acc.wrapping_div(process[operand as usize])
            }
            12 =>
            // remainder
            {
                acc = acc.wrapping_rem(process[operand as usize])
            }
            13 =>
            // jump
            {
                ip = operand as usize
            }
            14 =>
            // jump_if_zero
            {
                if acc == 0 {
                    ip = operand as usize
                }
            }
            15 =>
            // jump_if_nonzero
            {
                if acc != 0 {
                    ip = operand as usize
                }
            }
            16 =>
            // jump_if_positive
            {
                if (acc as i16) > 0 {
                    ip = operand as usize
                }
            }
            17 =>
            // jump_if_negative
            {
                if (acc as i16) < 0 {
                    ip = operand as usize
                }
            }
            18 =>
            // jump_if_nonpositive
            {
                if (acc as i16) <= 0 {
                    ip = operand as usize
                }
            }
            19 =>
            // jump_if_nonnegative
            {
                if (acc as i16) >= 0 {
                    ip = operand as usize
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let prog: Vec<u16> = vec![
        43, 1, 39, 3, 39, 2, 39, 9, 42, 3, 39, 2, 33, 12, 40, 8, 41, 5, 39, 2, 33, 11, 40, 3, 33,
        15, 5, 1, 34, 7, 5, 0, 0, 6710, 0, 0, 0, 0, 0, 0, 10, 48, 1,
    ];
    execute(&prog);
}
