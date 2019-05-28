use opencv::core::{CV_32S, CV_32SC3, CV_64F, CV_64FC1, CV_8U, CV_8UC2, MAKETYPE, Moments};

#[test]
fn make_type() {
    assert_eq!(MAKETYPE(CV_8U, 2), CV_8UC2);
    assert_eq!(MAKETYPE(CV_32S, 3), CV_32SC3);
    assert_eq!(MAKETYPE(CV_64F, 1), CV_64FC1);
}

#[test]
fn moments() {
    let moments = Moments::default().unwrap();
    assert_eq!(0., moments.m00);
    assert_eq!(0., moments.m12);
    assert_eq!(0., moments.mu30);
}
