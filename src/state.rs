use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::Snap;

/// Trait for the application state.
/// Used with the axum router function 'with_state()'.
#[allow(dead_code)]
pub trait SnapAppState {
    /// Create a snap with a message.
    /// Snap timestamp will be the time of creation.
    /// ID of the snap will be random.
    /// Returns a copy of the snap on success,
    /// or an error if it can't create it.
    fn post(&mut self, message: String) -> Result<Snap, SnapCreationError>;

    /// Return a vector with the copy of all snaps
    /// at the time, ordered from the most recent to the oldest.
    fn get(&self) -> Vec<Snap>;

    /// Return the ammount of snaps currently.
    fn snap_count(&self) -> usize;
}

#[derive(Debug)]
pub enum SnapCreationError {
    IdCollisionError,
}

/// Simple repository for snaps in mememory.
#[derive(Clone)]
pub struct MockSnapRepository {
    snaps_mtx: Arc<Mutex<HashMap<String, Snap>>>,
}

impl MockSnapRepository {
    /// Create a new empty repository.
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
