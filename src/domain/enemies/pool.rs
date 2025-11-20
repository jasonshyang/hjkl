use std::collections::HashMap;

use crate::domain::enemies::enemy::{Enemy, EnemyId};

/// Pool managing reusable enemy instances.
pub struct EnemyPool {
    pool: HashMap<EnemyId, Enemy>,
}

impl EnemyPool {
    pub fn new(pool_size: usize, move_interval: std::time::Duration, move_radius: usize) -> Self {
        let capacity = pool_size;
        let mut enemies = HashMap::with_capacity(capacity);
        for idx in 0..capacity {
            let enemy = Enemy::new(idx, move_interval, move_radius);
            enemies.insert(idx.into(), enemy);
        }
        Self { pool: enemies }
    }
}

impl EnemyPool {
    /// Takes an enemy from the pool, if available.
    pub fn take_enemy(&mut self) -> Option<Enemy> {
        let enemy_id = *self.pool.keys().next()?;
        self.pool.remove(&enemy_id)
    }

    /// Returns an enemy back to the pool.
    pub fn return_enemy(&mut self, enemy: Enemy) {
        self.pool.insert(enemy.id(), enemy);
    }
}
