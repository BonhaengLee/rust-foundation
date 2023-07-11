pub fn chain_function() {
    let v: Vec<i32> = (1..4).chain([20, 30, 40]).rev().collect();

    assert_eq!(v, [40, 30, 20, 3, 2, 1]);
}
