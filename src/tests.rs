use super::*;

#[test]
fn test1() {
    let cache = Cache::load().unwrap();
    cache.save().unwrap();
}