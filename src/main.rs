//
// Find shortest path with 
// difference in elevation growth eq 1
// Also good if elevation of destination 
// square is less then current elevation.
//
const INPUT_FOR_VERIFICATION: &str= "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

// Values for mark start and end positions
const START_VALUE: u8 = 30;
const END_VALUE: u8 = 40;

struct MapDesc {
    rows: u16,
    columns: u16,
    data: Vec<u8>,
}

fn read_input(input: &str) -> MapDesc {
    let bytes = input.bytes();
    let input_len = bytes.len();
    const BASE_SYMBOL_BYTE  :u8 = 'a' as u8;
    const START_SYMBOL_BYTE :u8 = 'S' as u8;
    const END_SYMBOL_BYTE   :u8 = 'E' as u8;
    const NEW_LINE_SYMBOL   :u8 = '\n' as u8;

    println!("base symbol is {}", BASE_SYMBOL_BYTE);
    
    let mut data = vec![0; input_len];

    let mut columns: Option<u16> = None;
    let mut rows: u16 = 0;
    let mut cnt: usize = 0;
    for x in bytes {

        if NEW_LINE_SYMBOL == x {
            println!();
            rows += 1;
            columns.get_or_insert(cnt.try_into().unwrap());
            continue;
        }

        data[cnt] = 
        match x {
            START_SYMBOL_BYTE => START_VALUE,
            END_SYMBOL_BYTE => END_VALUE,
            _ => x - BASE_SYMBOL_BYTE };

        println!("[{}] {} -> {}", cnt, x, data[cnt]);
        cnt += 1;
    }

    let columns: u16 = columns.unwrap();
    // Remove elements allocated for newlines
    data.truncate((columns * rows).into());

    let result = MapDesc {
        rows        : rows,
        columns     : columns,
        data        : data,
    };

    println!();
    cnt = 0;
    for x in &result.data{
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
