use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::Snap;

#[allow(dead_code)]
pub trait SnapAppState {
    fn post(&mut self, message: String) -> Result<Snap, SnapCreationError>;

    fn get(&self) -> Vec<Snap>;

    fn snap_count(&self) -> usize;
}

#[derive(Debug)]
pub enum SnapCreationError {
    IdCollisionError,
}

#[derive(Clone)]
pub struct MockSnapRepository {
    snaps_mtx: Arc<Mutex<HashMap<String, Snap>>>,
}

impl MockSnapRepository {
    pub fn new() -> MockSnapRepository {
        MockSnapRepository {
            snaps_mtx: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl SnapAppState for MockSnapRepository {
    fn post(&mut self, message: String) -> Result<Snap, SnapCreationError> {
        let mut snaps = self.snaps_mtx
            .lock()
            .unwrap();

        let snap = Snap::new(message);

        if snaps.contains_key(snap.id()) {
            return Err(SnapCreationError::IdCollisionError)
        }

        snaps.insert(snap.id().clone(), snap.clone());
        Ok(snap)
    }

    fn get(&self) -> Vec<Snap> {
        let mut vec = self.snaps_mtx
            .lock()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<Snap>>();

        vec.sort_by(|a, b| b.timestamp().cmp(a.timestamp()));
        vec
    }

    fn snap_count(&self) -> usize {
        self.snaps_mtx
            .lock()
            .unwrap()
            .len()
    }
}

#[cfg(test)]
mod mock_repo_test {
    use super::*;

    #[test]
    fn posting_snaps() {
        let mut repo = MockSnapRepository::new();
        assert_eq!(repo.snap_count(), 0);

        repo.post("A".to_string()).unwrap();
        assert_eq!(repo.snap_count(), 1);

        repo.post("B".to_string()).unwrap();
        repo.post("C".to_string()).unwrap();
        assert_eq!(repo.snap_count(), 3);
    }

    #[test]
    fn get_snaps_is_sorted() {
        let mut repo = MockSnapRepository::new();
        let snap_a = repo.post("A".to_string()).unwrap();
        let snap_b = repo.post("B".to_string()).unwrap();

        assert!(snap_a.timestamp() <= snap_b.timestamp());
        assert_ne!(snap_a.id(), snap_b.id());

        let snaps = repo.get();
        assert_eq!(snaps.len(), 2);
        // snap_b got posted last so it should be returned first
        assert_eq!(snaps[0].id(), snap_b.id());
        assert_eq!(snaps[1].id(), snap_a.id());
    }
}
