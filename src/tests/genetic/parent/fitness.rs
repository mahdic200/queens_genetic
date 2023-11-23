use crate::genetic::parent::Parent;

#[test]
pub fn fitness_on_best() {
    let locations = vec![4, 2, 7, 5, 1, 8, 6, 3];
    let instance = Parent::new(locations);
    assert_eq!(instance.fitness(), 1.0 / (0.0 + instance.epsilon()));
}

#[test]
pub fn fitness_on_bad() {
    let locations = vec![4, 5, 6, 1, 2, 3, 8, 7];
    let instance = Parent::new(locations);
    assert_eq!(instance.fitness(), 1.0 / (10.0 + instance.epsilon()));
}