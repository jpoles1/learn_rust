fn main() {
    let range = 1..7;
    let new_range = range.fold(1, |prod, n:i64| n.pow(3)*prod);
    println!("{:?}", new_range);
}
