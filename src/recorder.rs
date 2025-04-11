use std::{cell::RefCell, rc::Rc};


#[derive(Default, Debug)]
pub struct Recorder {
    pub clones: usize,
    pub dropped: bool,
}

#[derive(Default, Debug)]
pub struct Dummy {
    pub recorder: Rc<RefCell<Recorder>>,
}

impl PartialEq for Dummy {
    fn eq(&self, other: &Self) -> bool {
        self.recorder.borrow().clones == other.recorder.borrow().clones
    }
}

impl Eq for Dummy {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for Dummy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Dummy {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.recorder.borrow().clones.cmp(&other.recorder.borrow().clones)
    }
}

impl Clone for Dummy {
    fn clone(&self) -> Self {
        let mut recorder = self.recorder.borrow_mut();
        recorder.clones += 1;
        Dummy {
            recorder: self.recorder.clone(),
        }
    }
}

impl Drop for Dummy {
    fn drop(&mut self) {
        let mut recorder = self.recorder.borrow_mut();
        recorder.dropped = true;
    }
}

pub fn create_dummy() -> (Rc<RefCell<Recorder>>, Dummy) {
    let recorder = Rc::new(RefCell::new(Recorder::default()));
    let dummy = Dummy {
        recorder: recorder.clone(),
    };
    (recorder, dummy)
}

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
