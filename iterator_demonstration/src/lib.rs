#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    
    assert_eq!(v1_iter.next(), Some(&1));
}
