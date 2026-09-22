use super::entity::IngestState;
use crate::domain::core::errors::{SandlandError, SandlandResult};

pub struct IngestStateMachine;

impl IngestStateMachine {
    pub fn can_transition(current: &IngestState, next: &IngestState) -> bool {
        match (current, next) {
            (IngestState::Pending, IngestState::Extracting) => true,
            (IngestState::Pending, IngestState::Failed) => true,
            (IngestState::Extracting, IngestState::Classifying) => true,
            (IngestState::Extracting, IngestState::Failed) => true,
            (IngestState::Classifying, IngestState::Classified) => true,
            (IngestState::Classifying, IngestState::NeedsManualReview) => true,
            (IngestState::Classifying, IngestState::Failed) => true,
            (IngestState::NeedsManualReview, IngestState::Classified) => true,
            (IngestState::Failed, IngestState::Pending) => true, // Retry
            _ => false,
        }
    }

    pub fn transition(current: &mut IngestState, next: IngestState) -> SandlandResult<()> {
        if Self::can_transition(current, &next) {
            *current = next;
            Ok(())
        } else {
            Err(SandlandError::IngestFailed {
                item_id: "unknown".to_string(),
                reason: format!("Transição de estado inválida: {:?} -> {:?}", current, next),
            })
        }
    }
}
