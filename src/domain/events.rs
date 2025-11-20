use crate::domain::Position;
use std::time::Instant;

/// Events that can occur in the game world.
///
/// This is used to communicate between different systems in the game.
#[derive(Debug, Clone)]
pub enum GameEvent {
    /// An enemy has been destroyed at the given position.
    EnemyDestroyed { position: Position },
    /// The player's cursor has moved to a new position at the given timestamp.
    CursorMoved {
        position: Position,
        timestamp: Instant,
    },
}
