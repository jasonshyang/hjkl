use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{domain::Position, tui::theme::*};

/// A visual effect in the TUI.
pub struct Effect {
    pub ty: EffectType,
    pub position: Position,
    pub timestamp: Instant,
}

/// Types of visual effects.
pub enum EffectType {
    Collision,
    Trailing,
}

impl Effect {
    pub fn collision(position: Position) -> Self {
        Self {
            ty: EffectType::Collision,
            position,
            timestamp: Instant::now(),
        }
    }

    pub fn trailing(position: Position, timestamp: Instant) -> Self {
        Self {
            ty: EffectType::Trailing,
            position,
            timestamp,
        }
    }

    pub fn percentage_elapsed(&self) -> f32 {
        let total_duration = self.duration().as_secs_f32();
        let elapsed = Instant::now().duration_since(self.timestamp).as_secs_f32();
        (elapsed / total_duration).clamp(0.0, 1.0)
    }

    pub fn duration(&self) -> Duration {
        match self.ty {
            EffectType::Collision => COLLISION_EFFECT_DURATION,
            EffectType::Trailing => TRAILING_EFFECT_DURATION,
        }
    }
}

/// Manages visual effects in the TUI.
#[derive(Default)]
pub struct Effects {
    map: HashMap<Position, Effect>,
}

impl Effects {
    pub fn get(&self, position: &Position) -> Option<&Effect> {
        self.map.get(position)
    }

    pub fn spawn_effect(&mut self, effect: Effect) {
        self.map.insert(effect.position, effect);
    }

    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.map
            .retain(|_, effect| now.duration_since(effect.timestamp) < effect.duration());
    }
}
