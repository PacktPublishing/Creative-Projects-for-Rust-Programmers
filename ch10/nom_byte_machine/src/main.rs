mod emulator;
mod instructions;
mod parsing_interpreter;
mod translator;

fn main() {
    let prog = vec![
        187, 2, // 0: 699
        // Let the user input the digits of the limit number.
        1, 28, 1, // 2, 0: set digits
        6, 5, // 5, 0: input 5
        // Initialize digit pointer.
        1, 28, 1, // 7, 0: set digits
        3, 33, 1, // 10, 0: store pos
        // If the digit is less than 0, parsing is ended.
        // 13, 0: before_parsing_number
        22, 33, 1, // 13, 0: indirect_load_byte pos
        9, 37, 1, // 16, 0: subtract ascii_zero
        17, 73, 0, // 19, 0: jump_if_negative after_parsing_number
        // If the digit is greater than 9, parsing is ended.
        22, 33, 1, // 22, 0: indirect_load_byte pos
        9, 37, 1, // 25, 0: subtract ascii_zero
        9, 35, 1, // 28, 0: subtract number_base
        19, 73, 0, // 31, 0: jump_if_nonnegative after_parsing_number
        // Multiply by 10 the current limit.
        2, 22, 1, // 34, 0: load limit
        10, 35, 1, // 37, 0: multiply number_base
        3, 22, 1, // 40, 0: store limit
        // Add next digit to current limit.
        22, 33, 1, // 43, 0: indirect_load_byte pos
        9, 37, 1, // 46, 0: subtract ascii_zero
        8, 22, 1, // 49, 0: add limit
        3, 22, 1, // 52, 0: store limit
        // Increment digit pointer
        2, 33, 1, // 55, 0: load pos
        8, 39, 1, // 58, 0: add one
        3, 33, 1, // 61, 0: store pos
        // If pos points to itself, the digit buffer is ended.
        1, 33, 1, // 64, 0: set pos
        9, 33, 1, // 67, 0: subtract pos
        15, 13, 0, // 70, 0: jump_if_nonzero before_parsing_number
        // 73, 0: after_parsing_number
        2, 41, 1, // 73, 0: load two
        3, 24, 1, // 76, 0: store i
        // 79, 0: before_computing_primes
        2, 24, 1, // 79, 0: load i
        9, 22, 1, // 82, 0: subtract limit
        19, 157, 0, // 85, 0: jump_if_nonnegative after_computing_primes
        1, 43, 1, // 88, 0: set primes
        8, 24, 1, // 91, 0: add i
        3, 33, 1, // 94, 0: store pos
        22, 33, 1, // 97, 0: indirect_load_byte pos
        15, 145, 0, // 100, 0: jump_if_nonzero after_setting_multiples
        2, 24, 1, // 103, 0: load i
        8, 24, 1, // 106, 0: add i
        3, 26, 1, // 109, 0: store j
        // 112, 0: before_setting_multiples
        9, 22, 1, // 112, 0: subtract limit
        19, 145, 0, // 115, 0: jump_if_nonnegative after_setting_multiples
        1, 43, 1, // 118, 0: set primes
        8, 26, 1, // 121, 0: add j
        3, 33, 1, // 124, 0: store pos
        2, 39, 1, // 127, 0: load one
        23, 33, 1, // 130, 0: indirect_store_byte pos
        2, 26, 1, // 133, 0: load j
        8, 24, 1, // 136, 0: add i
        3, 26, 1, // 139, 0: store j
        13, 112, 0, // 142, 0: jump before_setting_multiples
        // 145, 0: after_setting_multiples
        2, 24, 1, // 145, 0: load i
        8, 39, 1, // 148, 0: add one
        3, 24, 1, // 151, 0: store i
        13, 79, 0, // 154, 0: jump before_computing_primes
        // 157, 0: after_computing_primes
        2, 41, 1, // 157, 0: load two
        3, 24, 1, // 160, 0: store i
        // 163, 0: before_printing_primes
        2, 24, 1, // 163, 0: load i
        9, 22, 1, // 166, 0: subtract limit
        19, 20, 1, // 169, 0: jump_if_nonnegative after_printing_all_primes
        1, 43, 1, // 172, 0: set primes
        8, 24, 1, // 175, 0: add i
        3, 33, 1, // 178, 0: store pos
        22, 33, 1, // 181, 0: indirect_load_byte pos
        15, 8, 1, // 184, 0: jump_if_nonzero after_printing_a_prime
        // Format a prime number
        2, 24, 1, // 187, 0: load i
        3, 26, 1, // 190, 0: store j
        1, 33, 1, // 193, 0: set pos
        3, 33, 1, // 196, 0: store pos
        // 199, 0: before_generating_digits
        2, 33, 1, // 199, 0: load pos
        9, 39, 1, // 202, 0: subtract one
        3, 33, 1, // 205, 0: store pos
        2, 26, 1, // 208, 0: load j
        12, 35, 1, // 211, 0: remainder number_base
        8, 37, 1, // 214, 0: add ascii_zero
        23, 33, 1, // 217, 0: indirect_store_byte pos
        2, 26, 1, // 220, 0: load j
        11, 35, 1, // 223, 0: divide number_base
        3, 26, 1, // 226, 0: store j
        15, 199, 0, // 229, 0: jump_if_nonzero before_generating_digits
        // Clear the initial spaces.
        // 232, 0: before_clearing_spaces
        1, 28, 1, // 232, 0: set digits
        9, 33, 1, // 235, 0: subtract pos
        14, 3, 1, // 238, 0: jump_if_zero after_clearing_spaces
        2, 33, 1, // 241, 0: load pos
        9, 39, 1, // 244, 0: subtract one
        3, 33, 1, // 247, 0: store pos
        1, 32, 0, // 250, 0: set 32 // blank
        23, 33, 1, // 253, 0: indirect_store_byte pos
        13, 232, 0, // 0, 1: jump before_clearing_spaces
        // 3, 1: after_clearing_spaces

        // Emit the prime number.
        1, 28, 1, // 3, 1: set digits
        7, 5, // 6, 1: output 5
        // 8, 1: after_printing_a_prime
        2, 24, 1, // 8, 1: load i
        8, 39, 1, // 11, 1: add one
        3, 24, 1, // 14, 1: store i
        13, 163, 0, // 17, 1: jump before_printing_primes
        // 20, 1: after_printing_all_primes
        0, 0, // 20, 1: terminate 0
        // data
        0, 0, // 22, 1: limit: word 0
        0, 0, // 24, 1: i: word 0
        0, 0, // 26, 1: j: word 0
        0, 0, 0, 0, 0, // 28, 1: digits: array 5
        0, 0, // 33, 1: pos: word 0
        10, 0, // 35, 1: number_base: word 10
        48, 0, // 37, 1: ascii_zero: word 48
        1, 0, // 39, 1: one: word 1
        2, 0, // 41, 1: two: word 2
           // 43, 1: primes: array 400
    ];

    let _ = translator::translate_program_to_c(&prog, "prog.c");

    let return_code = emulator::execute_program(&prog).unwrap();
    println!("\nReturn code: {}", return_code);

    let mut parsed_program = parsing_interpreter::parse_program(&prog).unwrap();
    //println!("\nparsed_program: {:?}", parsed_program);
    let return_code = parsing_interpreter::execute_parsed_program(&mut parsed_program);
    println!("\nReturn code: {}", return_code);
}
