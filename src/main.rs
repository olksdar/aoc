//
// Find shortest path with 
// difference in elevation growth eq 1
// Also good if elevation of destination 
// square is less then current elevation.
//

// Values for mark start and end positions
const START_VALUE: u8 = 30;
const END_VALUE: u8 = 40;

struct MapDesc {
    rows: usize,
    columns: usize,
    data: Vec<u8>,
    start: usize,
    end: usize,
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

    let mut columns: Option<usize> = None;
    // At least one row is available
    let mut rows: usize= 1;
    let mut cnt: usize = 0;
    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;

    for x in bytes {

        if NEW_LINE_SYMBOL == x {
            println!();
            rows += 1;
            columns.get_or_insert(cnt);
            continue;
        }

        data[cnt] = 
        match x {
            START_SYMBOL_BYTE => {start.get_or_insert(cnt); START_VALUE},
            END_SYMBOL_BYTE => {end.get_or_insert(cnt); END_VALUE},
            _ => x - BASE_SYMBOL_BYTE };

        println!("[{}] {} -> {}", cnt, x, data[cnt]);
        cnt += 1;
    }

    let columns: usize = columns.unwrap();
    // Remove elements allocated for newlines
    data.truncate(columns * rows);

    let result = MapDesc {
        rows        : rows,
        columns     : columns,
        data        : data,
        start       : start.unwrap(),
        end         : end.unwrap(),
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

fn find_path(input_map: &MapDesc) -> u16 {

    0
}

fn main() {
//    let map_presentation = read_input(INPUT_FOR_VERIFICATION);
//    let steps = get_steps(map_presentation);
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT_FOR_VERIFICATION: &str= "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";


    #[test]
    fn input() {
        let map_repr = read_input(INPUT_FOR_VERIFICATION);
        assert_eq!(map_repr.rows, 5);
        assert_eq!(map_repr.columns, 8);
        assert_eq!(map_repr.data.len(), 40);
        assert_eq!(map_repr.start, 0);
        assert_eq!(map_repr.end, 21);
    }

    #[test]
    fn pathfind() {
        let map_repr = read_input(INPUT_FOR_VERIFICATION);
        let path_len = find_path(&map_repr);
        assert_eq!(path_len, 31);
    }

}
