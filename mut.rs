fn main() {
    let mut i = 0;
    update(&mut i);
    println!("i = {}", i);
}

// immutable なのでコンパイルエラー
// fn cannot_update(i: u64) {
//     i = 1
// }

fn update(i: &mut u64) {
    *i = 1
}