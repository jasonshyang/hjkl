use std::time::Duration;

/// Top level configuration for the game domain layer
#[derive(Clone, Debug, Default)]
pub struct GameConfig {
    /// Enemy configuration
    pub enemy: EnemyConfig,
    /// File to load at start, if not provided, a random buffer is generated
    pub file_path: Option<String>,
}

/// Configuration for enemy behavior
#[derive(Clone, Debug)]
pub struct EnemyConfig {
    pub pool_size: usize,
    pub move_interval: Duration,
    pub move_radius: usize,
    pub spawn_interval: Duration,
}

impl Default for EnemyConfig {
    fn default() -> Self {
        Self {
            pool_size: 32,
            move_interval: Duration::from_millis(2500),
            move_radius: 3,
            spawn_interval: Duration::from_secs(2),
        }
    }
}
