const INPUT_FOR_VERIFICATION: &str= "SabqponmabcryxxlaccszExkacctuvwjabdefghi";

// Values for mark start and end positions
const START_VALUE: u8 = 30;
const END_VALUE: u8 = 40;

fn read_input(input: &str) -> Vec<u8> {
    let bytes = input.bytes();
    let input_len = bytes.len();
    const BASE_SYMBOL_BYTE  :u8 = 'a' as u8;
    const START_SYMBOL_BYTE :u8 = 'S' as u8;
    const END_SYMBOL_BYTE   :u8 = 'E' as u8;

    println!("base symbol is {}", BASE_SYMBOL_BYTE);
    
    let mut result = vec![0; input_len];

    let mut cnt = 0;
    for (idx, x) in bytes.enumerate() {

        if ( idx != 0 && idx % 8 == 0) {
            println!();
        }

        result[idx] = 
        match x {
            START_SYMBOL_BYTE => START_VALUE,
            END_SYMBOL_BYTE => END_VALUE,
            _ => x - BASE_SYMBOL_BYTE };

        println!("[{}] {} -> {}", idx, x, result[idx]);
    }
    println!();
    cnt = 0;
    for x in &result {
        print!("{:3} ", x);
        if ( {cnt+=1; cnt} % 8 == 0) {
            println!();
        }
  
    }
    // Return result
    result
}

fn main() {
    read_input(INPUT_FOR_VERIFICATION);
}
