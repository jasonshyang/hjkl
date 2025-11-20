use std::time::{Duration, Instant};

/// Spawner that determines when to spawn new enemies.
pub struct EnemySpawner {
    last_spawned: Instant,
    interval: Duration,
}

impl EnemySpawner {
    pub fn new(spawn_interval: Duration) -> Self {
        Self {
            last_spawned: Instant::now(),
            interval: spawn_interval,
        }
    }
}

impl EnemySpawner {
    pub fn should_spawn(&mut self) -> bool {
        if self.last_spawned.elapsed() >= self.interval {
            self.last_spawned = Instant::now();
            true
        } else {
            false
        }
    }
}
