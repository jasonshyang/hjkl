use crate::domain::config::GameConfig;
use crate::domain::enemies::Enemies;
use crate::domain::events::GameEvent;
use crate::domain::mechanics::{CollisionEvent, check_collisions};
use crate::domain::motions::Motion;
use crate::domain::{Buffer, Cursor, generate_random_rust_code_buffer, load_buffer_from_file};
use std::time::Instant;

/// The game world, containing the buffer, cursor, enemies, and game state.
pub struct World {
    /// The text buffer
    buffer: Buffer,
    /// The player's cursor
    cursor: Cursor,
    /// Currently active enemies
    enemies: Enemies,
    /// Events generated on this tick
    events: Vec<GameEvent>,
    /// Current score
    score: usize,
    /// Config
    config: GameConfig,
}

impl Default for World {
    fn default() -> Self {
        Self::new(GameConfig::default())
    }
}

impl World {
    /// Creates a new World with the given game configuration.
    ///
    /// Buffer is loaded from file if path provided; otherwise, a random Rust code buffer is generated.
    pub fn new(config: GameConfig) -> Self {
        let buffer = if let Some(ref path) = config.file_path {
            load_buffer_from_file(path).unwrap_or_else(|_| generate_random_rust_code_buffer())
        } else {
            generate_random_rust_code_buffer()
        };

        Self {
            buffer,
            cursor: Cursor::default(),
            enemies: Enemies::new(&config.enemy),
            events: Vec::new(),
            config,
            score: 0,
        }
    }

    pub fn reset(&mut self) {
        self.cursor.reset();
        self.buffer = if let Some(ref path) = self.config.file_path {
            load_buffer_from_file(path).unwrap_or_else(|_| generate_random_rust_code_buffer())
        } else {
            generate_random_rust_code_buffer()
        };
        self.enemies = Enemies::new(&self.config.enemy);
        self.events.clear();
        self.score = 0;
    }

    /// Returns a reference to the current text buffer.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Returns the number of lines in the buffer.
    pub fn buffer_lines(&self) -> usize {
        self.buffer.rows()
    }

    /// Returns a reference to the player's cursor.
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Returns a reference to the currently active enemies.
    pub fn enemies(&self) -> &Enemies {
        &self.enemies
    }

    /// Returns the current score.
    pub fn score(&self) -> usize {
        self.score
    }

    /// Pull (consume) all events generated since last pull
    pub fn pull_events(&mut self) -> Vec<GameEvent> {
        std::mem::take(&mut self.events)
    }

    /// Apply motion to cursor and handle resulting events
    pub fn apply_motion(&mut self, motion: Motion, count: Option<usize>) {
        let old_pos = self.cursor.pos();
        self.cursor.apply_motion(&self.buffer, motion, count);
        let new_pos = self.cursor.pos();

        // Generate cursor moved event if position changed
        if old_pos != new_pos {
            self.events.push(GameEvent::CursorMoved {
                position: new_pos,
                timestamp: Instant::now(),
            });
        }

        // Check for collisions
        let collision_events = check_collisions(new_pos, &self.enemies);
        for event in collision_events {
            match event {
                CollisionEvent::PlayerHitEnemy { position, enemy_id } => {
                    self.enemies.destroy(&enemy_id.into());
                    self.score += 1;
                    self.events.push(GameEvent::EnemyDestroyed { position });
                }
            }
        }
    }

    /// Advance the game state by one tick
    ///
    /// Currently only enemies move each tick.
    pub fn tick(&mut self) {
        self.enemies.tick(&self.buffer);
    }
}
