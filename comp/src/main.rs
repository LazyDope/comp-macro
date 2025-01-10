use comp_macro::comp;

fn main() {
    let new_arr = [(1, 2), (2, 3), (3, 4)];
    let new_arr2 = [[1, 2], [3, 4], [5, 6]];
    println!(
        "{:?}",
        Vec::from_iter(comp![(x * 2, y - 1) for (x, y) in new_arr if x > 1 if y == 3])
    );
    for v in comp![x * 2 for x in arr if x > 1 for arr in new_arr2 if arr[1] < 6] {
        println!("{:?}", v);
    }
}
