
use better_stopwatch::in_development;

#[test]
pub fn test_in_development() {
    assert_eq!(in_development(), "in development".to_string());
}