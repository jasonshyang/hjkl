use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

use crate::domain::{
    Buffer, EnemyConfig, Position,
    enemies::{pool::EnemyPool, spawner::EnemySpawner},
};

/// Unique identifier for an enemy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnemyId(usize);

impl From<usize> for EnemyId {
    fn from(id: usize) -> Self {
        EnemyId(id)
    }
}

impl EnemyId {
    pub fn id(&self) -> usize {
        self.0
    }
}

/// The collection of enemies in the game world,
/// including active enemies, enemy pool, and spawner.
pub struct Enemies {
    /// Currently active enemies mapped by their IDs.
    active: HashMap<EnemyId, Enemy>,
    /// Enemy pool for managing reusable enemy instances.
    enemy_pool: EnemyPool,
    /// Spawner for determining when to spawn new enemies.
    spawner: EnemySpawner,
}

impl Enemies {
    pub fn new(config: &EnemyConfig) -> Self {
        Self {
            active: HashMap::new(),
            enemy_pool: EnemyPool::new(config.pool_size, config.move_interval, config.move_radius),
            spawner: EnemySpawner::new(config.spawn_interval),
        }
    }

    /// Gets a reference to an enemy by its ID.
    pub fn get(&self, id: &EnemyId) -> Option<&Enemy> {
        self.active.get(id)
    }

    /// Gets all enemy positions.
    pub fn positions(&self) -> Vec<Position> {
        self.active.values().map(|e| e.pos()).collect()
    }

    /// Gets a set of all enemy positions.
    pub fn position_set(&self) -> HashSet<Position> {
        self.active.values().map(|e| e.pos()).collect()
    }

    /// Returns an iterator over all active enemies.
    pub fn iter(&self) -> impl Iterator<Item = &Enemy> {
        self.active.values()
    }

    /// Advances the state of all enemies and spawns new ones as needed.
    ///
    /// Only spawn if there are available enemies in the pool.
    pub fn tick(&mut self, buffer: &Buffer) {
        if self.spawner.should_spawn()
            && let Some(mut enemy) = self.enemy_pool.take_enemy()
        {
            // spawn at a random position
            let start_pos = buffer.random_position(false).unwrap_or_default();
            enemy.move_to(start_pos);
            self.active.insert(enemy.id(), enemy);
        }

        for enemy in self.active.values_mut() {
            enemy.tick(buffer);
        }
    }

    /// Destroys an enemy by its ID, returning it to the pool.
    pub fn destroy(&mut self, id: &EnemyId) {
        if let Some(enemy) = self.active.remove(id) {
            self.enemy_pool.return_enemy(enemy);
        }
    }
}

/// An enemy in the game world.
pub struct Enemy {
    id: EnemyId,
    position: Position,
    last_moved: Instant,
    move_interval: Duration,
    move_radius: usize,
}

impl Enemy {
    pub fn new(id: impl Into<EnemyId>, move_interval: Duration, move_radius: usize) -> Self {
        Self {
            id: id.into(),
            position: Position::default(),
            last_moved: Instant::now(),
            move_interval,
            move_radius,
        }
    }

    pub fn id(&self) -> EnemyId {
        self.id
    }

    pub fn pos(&self) -> Position {
        self.position
    }

    pub fn reset(&mut self) {
        self.position = Position::default();
        self.last_moved = Instant::now();
    }

    /// Advances the enemy's state, moving it if enough time has passed.
    pub fn tick(&mut self, buffer: &Buffer) -> bool {
        if self.last_moved.elapsed() >= self.move_interval {
            self.move_random(buffer);
            self.last_moved = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn move_to(&mut self, position: Position) {
        self.position = position;
    }

    pub fn move_random(&mut self, buffer: &Buffer) {
        let new_position = buffer
            .random_position_from(self.position, self.move_radius, false)
            .unwrap_or(self.position);
        self.position = new_position;
        self.last_moved = Instant::now();
    }
}
