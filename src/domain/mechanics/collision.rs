use crate::domain::{Position, enemies::Enemies};

/// Events related to collisions in the game world.
#[derive(Debug, Clone)]
pub enum CollisionEvent {
    /// Event when the player hits an enemy.
    PlayerHitEnemy { position: Position, enemy_id: usize },
}

pub fn check_collisions(cursor_pos: Position, enemies: &Enemies) -> Vec<CollisionEvent> {
    enemies
        .iter()
        .filter(|enemy| enemy.pos() == cursor_pos)
        .map(|enemy| CollisionEvent::PlayerHitEnemy {
            position: enemy.pos(),
            enemy_id: enemy.id().id(),
        })
        .collect()
}
