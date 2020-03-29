use clock::{extract_remainder, normal_time};

#[test]
fn normal_0() {
    assert_eq!(normal_time(0, 0), (0, 0));
}

#[test]
fn normal_2() {
    assert_eq!(normal_time(0, 11), (0, 11));
}

#[test]
fn normal_3() {
    assert_eq!(normal_time(0, -1), (23, 59));
}

#[test]
fn normal_4() {
    assert_eq!(normal_time(-1, -1), (22, 59));
}

#[test]
fn extract_remainder_0() {
    assert_eq!(extract_remainder(0, 12), (0, 0))
}

#[test]
fn extract_remainder_1() {
    assert_eq!(extract_remainder(13, 12), (1, 1))
}

#[test]
fn extract_remainder_2() {
    assert_eq!(extract_remainder(-1, 12), (-1, 11))
}
