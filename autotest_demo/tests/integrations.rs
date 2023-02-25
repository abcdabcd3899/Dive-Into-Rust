use autotest_demo;

#[test]
fn test_add(){
    assert_eq!(autotest_demo::add(1, 2), 3);
}