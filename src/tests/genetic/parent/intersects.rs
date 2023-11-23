use crate::genetic::parent::Parent;

#[test]
pub fn intersects_on_best() {
    let locations = vec![4, 2, 7, 5, 1, 8, 6, 3];
    let instance = Parent::new(locations);
    assert_eq!(instance.intersects(), 0.0);
}


#[test]
pub fn intersects_on_bad() {
    let locations = vec![4, 5, 6, 1, 2, 3, 8, 7];
    let instance = Parent::new(locations);
    assert_eq!(instance.intersects(), 10.0);
}
