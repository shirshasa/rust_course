use crate::recorder::Recorder;
use crate::recorder::Dummy;
use crate::recorder::create_dummy;

use crate::two_way_map::TwoWayMap;

mod two_way_map;
mod recorder;


#[test]
fn test_dummy() {
    let (recorder, dummy) = create_dummy();
    assert_eq!(recorder.borrow().clones, 0);
    assert_eq!(recorder.borrow().dropped, false);

    let dummy_clone = dummy.clone();
    assert_eq!(recorder.borrow().clones, 1);
    assert_eq!(recorder.borrow().dropped, false);

    drop(dummy_clone);
    assert_eq!(recorder.borrow().clones, 1);
    assert_eq!(recorder.borrow().dropped, true);
}

#[test]
fn test_clone_counts(){
    let (recorder, dummy) = create_dummy();

    let mut map = TwoWayMap::new();
    map.insert(1, dummy);
    assert_eq!(recorder.borrow().clones, 0);
    assert_eq!(recorder.borrow().dropped, false);

    let map2 = map.clone();
    assert_eq!(recorder.borrow().clones, 1);
    assert_eq!(recorder.borrow().dropped, false);

    let map3 = map2.clone();
    assert_eq!(recorder.borrow().clones, 2);
    assert_eq!(recorder.borrow().dropped, false);
}

#[test]
fn test_moves(){
    let (recorder, dummy) = create_dummy();

    let mut map = TwoWayMap::new();
    map.insert(1, dummy);
    assert_eq!(recorder.borrow().clones, 0);
    assert_eq!(recorder.borrow().dropped, false);

    let map2 = map;
    assert_eq!(recorder.borrow().clones, 0);
    assert_eq!(recorder.borrow().dropped, false);
}

#[test]
fn test_into_iter_clone_counts(){
    let (recorder, dummy) = create_dummy();

    {
        let mut map = TwoWayMap::new();
        map.insert(1, dummy);
        assert_eq!(recorder.borrow().clones, 0);
        assert_eq!(recorder.borrow().dropped, false);
    
        let mut iter = map.into_iter();
        assert_eq!(recorder.borrow().clones, 0);
        assert_eq!(recorder.borrow().dropped, false);
    
        let (key, value) = iter.next().unwrap();
        println!("key: {}, value: {:?}", key, value);
    }
    assert_eq!(recorder.borrow().clones, 0);
    assert_eq!(recorder.borrow().dropped, true);

}

fn main() {
    
}