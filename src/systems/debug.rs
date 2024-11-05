use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};
use crate::components::{Actor, FpsText};

pub fn fps_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

pub fn print_player_position(
    player_query: Query<&Transform, With<Actor>>,
) {
    
    for transform in player_query.iter() {
        println!(
            "Player position: x={:.2}, y={:.2}", 
            transform.translation.x, 
            transform.translation.y
        );
    }
}
