use std::cmp::max;
use std::collections::HashMap;


fn main() {

    let small_triangle: String = String::from(
        "\
        3;\
        7, 4;\
        2, 4, 6;\
        8, 5, 9, 3\
        "
    );
    let mut small_triangle_map= HashMap::new();
    let mut small_triangle_memo = HashMap::new();

    let triangle: String = String::from(
        "\
        75;\
        95, 64;\
        17, 47, 82;\
        18, 35, 87, 10;\
        20, 04, 82, 47, 65;\
        19, 01, 23, 75, 03, 34;\
        88, 02, 77, 73, 07, 63, 67;\
        99, 65, 04, 28, 06, 16, 70, 92;\
        41, 41, 26, 56, 83, 40, 80, 70, 33;\
        41, 48, 72, 33, 47, 32, 37, 16, 94, 29;\
        53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14;\
        70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57;\
        91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48;\
        63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31;\
        04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23\
        "
    );
    let mut triangle_map= HashMap::new();
    let mut triangle_memo = HashMap::new();
    let mut triangle_size = 0;


    for (i, layer) in small_triangle.split(';').rev().enumerate() {
        for (j, num) in layer.trim().split(',').enumerate() {
            if i == 0 {
                small_triangle_memo.insert((i, j), num.trim().parse::<u16>().unwrap());
            } else {
                let max_below = max(small_triangle_memo.get(&(i - 1, j)).unwrap(), small_triangle_memo.get(&(i - 1, j + 1)).unwrap());
                small_triangle_memo.insert((i, j), num.trim().parse::<u16>().unwrap() + max_below);
            }
            small_triangle_map.insert((i, j), num.trim().parse::<u16>().unwrap());
        }
    }

    for (i, layer) in triangle.split(';').rev().enumerate() {
        for (j, num) in layer.trim().split(',').enumerate() {
            if i == 0 {
                triangle_memo.insert((i, j), num.trim().parse::<u16>().unwrap());
            } else {
                let max_below = max(triangle_memo.get(&(i - 1, j)).unwrap(), triangle_memo.get(&(i - 1, j + 1)).unwrap());
                triangle_memo.insert((i, j), num.trim().parse::<u16>().unwrap() + max_below);
            }
            triangle_map.insert((i, j), num.trim().parse::<u16>().unwrap());
        }
        triangle_size = i;
    }
    println!("The biggest achievable sum in the small triangle is {}.", small_triangle_memo.get(&(3usize, 0usize)).unwrap());
    println!("The biggest achievable sum in the big triangle is {}.", triangle_memo.get(&(triangle_size as usize, 0usize)).unwrap());
}
