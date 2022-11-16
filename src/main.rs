use std::collections::HashMap;

fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    println!("{code}");
    let mut pointer = 0i32;
    let mut instruct = 0;
    let code = code.chars().collect::<Vec<char>>();
    let mut input_byte = String::new();
    let mut tape: Vec<u8> = vec![0; 30000];
    let mut output: Vec<u8> = Vec::new();
    let mut output_byte = String::new();
    
    // Identify pairs of brackets
    let mut jumps: HashMap<usize,usize> = HashMap::new();
    let mut opens: Vec<usize> = Vec::new();
    for i in 0..code.len() {
        if code[i] == '[' {
            opens.push(i);
        } else if code[i] == ']' {
            let j = opens.pop().unwrap_or_default();
            jumps.insert(i, j);
            jumps.insert(j, i);
        }
    }
    
    let mut i = 0;
    while i < code.len() {
        match code[i] as char {
            '+' => tape[pointer as usize] ^= 1,
            ',' => {
                if instruct == input.len() && input_byte.is_empty() { 
                    tape[pointer as usize] = 0
                } else {
                    if input_byte.is_empty() {
                        input_byte = format!("{:08b}", input[instruct]);
                        instruct += 1;
                    }
                    tape[pointer as usize] = if input_byte.pop().unwrap() == '0' { 0 } else { 1 };
                }
            },
            ';' => output_byte.push_str(&format!("{}", tape[pointer as usize])),
            '<' => pointer -= 1,
            '>' => pointer += 1,
            '[' => if tape[pointer as usize] == 0 { i = *jumps.get(&i).unwrap() },
            ']' => if tape[pointer as usize] != 0 { i = *jumps.get(&i).unwrap() }
            _ => (),
        }
        if pointer < 0 {
            pointer += tape.len() as i32;
            let mut tmp_tape = vec![0; tape.len()];
            tmp_tape.extend(tape);
            tape = tmp_tape;
        } else if pointer > tape.len() as i32 {
            tape.resize(tape.len() * 2, 0);
        }
        if output_byte.len() == 8 {
            output.push(u8::from_str_radix(&output_byte.chars().rev().collect::<String>(), 2).unwrap());
            output_byte = String::new();
        }
        i += 1;
    }
    while output_byte.len() % 8 != 0 { output_byte.push('0'); }
    if !output_byte.is_empty() { output.push(u8::from_str_radix(&output_byte.chars().rev().collect::<String>(), 2).unwrap()); }
    output
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 2 {
        println!("Idiot, where's the files?");
        return;
    }
    let program_filename = args[1].clone();
    let input_filename = args[2].clone();
    let bf_program: String = std::fs::read_to_string(program_filename).expect("What's this garbage?");
    let bf_input : Vec<u8> = Vec::new();
    let bf_output = boolfuck(bf_program.as_str(), bf_input);
    println!("Here's your output: {bf_output:?}");
}
