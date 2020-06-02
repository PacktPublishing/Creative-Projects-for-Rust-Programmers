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
        //println!("ip: {:3} instr: {:2} {:5} acc: {:5} {:?}",
        //    ip, opcode, operand, acc, process[187..216].to_vec());
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
    let prog = vec![
        600, // 0:
        // Let the user input the digits of the limit number.
        1, 190, // 1: set digits
        6, 5, // 3: input 5
        // Initialize digit pointer.
        1, 190, // 5: set digits
        3, 195, // 7: store pos
        // If the digit is less than 0, parsing is ended.
        // 9: before_parsing_number
        4, 195, // 9: indirect_load pos
        9, 197, // 11: subtract ascii_zero
        17, 49, // 13: jump_if_negative after_parsing_number
        // If the digit is greater than 9, parsing is ended.
        4, 195, // 15: indirect_load pos
        9, 197, // 17: subtract ascii_zero
        9, 196, // 19: subtract number_base
        19, 49, // 21: jump_if_nonnegative after_parsing_number
        // Multiply by 10 the current limit.
        2, 187, // 23: load limit
        10, 196, // 25: multiply number_base
        3, 187, // 27: store limit
        // Add next digit to current limit.
        4, 195, // 29: indirect_load pos
        9, 197, // 31: subtract ascii_zero
        8, 187, // 33: add limit
        3, 187, // 35: store limit
        // Increment digit pointer
        2, 195, // 37: load pos
        8, 198, // 39: add one
        3, 195, // 41: store pos
        // If pos points to itself, the digit buffer is ended.
        1, 195, // 43: set pos
        9, 195, // 45: subtract pos
        15, 9, // 47: jump_if_nonzero before_parsing_number
        // 49: after_parsing_number
        2, 199, // 49: load two
        3, 188, // 51: store i
        // 53: before_computing_primes
        2, 188, // 53: load i
        9, 187, // 55: subtract limit
        19, 105, // 57: jump_if_nonnegative after_computing_primes
        1, 200, // 59: set primes
        8, 188, // 61: add i
        3, 195, // 63: store pos
        4, 195, // 65: indirect_load pos
        15, 97, // 67: jump_if_nonzero after_setting_multiples
        2, 188, // 69: load i
        8, 188, // 71: add i
        3, 189, // 73: store j
        // 75: before_setting_multiples
        9, 187, // 75: subtract limit
        19, 97, // 77: jump_if_nonnegative after_setting_multiples
        1, 200, // 79: set primes
        8, 189, // 81: add j
        3, 195, // 83: store pos
        2, 198, // 85: load one
        5, 195, // 87: indirect_store pos
        2, 189, // 89: load j
        8, 188, // 91: add i
        3, 189, // 93: store j
        13, 75, // 95: jump before_setting_multiples
        // 97: after_setting_multiples
        2, 188, // 97: load i
        8, 198, // 99: add one
        3, 188, // 101: store i
        13, 53, // 103: jump before_computing_primes
        // 105: after_computing_primes
        2, 199, // 105: load two
        3, 188, // 107: store i
        // 109: before_printing_primes
        2, 188, // 109: load i
        9, 187, // 111: subtract limit
        19, 185, // 113: jump_if_nonnegative after_printing_all_primes
        1, 200, // 115: set primes
        8, 188, // 117: add i
        3, 195, // 119: store pos
        4, 195, // 121: indirect_load pos
        15, 177, // 123: jump_if_nonzero after_printing_a_prime
        // Format a prime number
        2, 188, // 125: load i
        3, 189, // 127: store j
        1, 195, // 129: set pos
        3, 195, // 131: store pos
        // 133: before_generating_digits
        2, 195, // 133: load pos
        9, 198, // 135: subtract one
        3, 195, // 137: store pos
        2, 189, // 139: load j
        12, 196, // 141: remainder number_base
        8, 197, // 143: add ascii_zero
        5, 195, // 145: indirect_store pos
        2, 189, // 147: load j
        11, 196, // 149: divide number_base
        3, 189, // 151: store j
        15, 133, // 153: jump_if_nonzero before_generating_digits
        // Clear the initial spaces.
        // 155: before_clearing_spaces
        1, 190, // 155: set digits
        9, 195, // 157: subtract pos
        14, 173, // 159: jump_if_zero after_clearing_spaces
        2, 195, // 161: load pos
        9, 198, // 163: subtract one
        3, 195, // 165: store pos
        1, 32, // 167: set 32 // blank
        5, 195, // 169: indirect_store pos
        13, 155, // 171: jump before_clearing_spaces
        // 173: after_clearing_spaces

        // Emit the prime number.
        1, 190, // 173: set digits
        7, 5, // 175: output 5
        // 177: after_printing_a_prime
        2, 188, // 177: load i
        8, 198, // 179: add one
        3, 188, // 181: store i
        13, 109, // 183: jump before_printing_primes
        // 185: after_printing_all_primes
        0, 0, // 185: terminate 0
        // data
        0, // 187: limit: word 280
        0, // 188: i: word 0
        0, // 189: j: word 0
        0, 0, 0, 0, 0,  // 190: digits: array 5
        0,  // 195: pos: word 0
        10, // 196: number_base: word 10
        48, // 197: ascii_zero: word 48
        1,  // 198: one: word 1
        2,  // 199: two: word 2
            // 200: primes: array 400
    ];
    execute(&prog);
}
