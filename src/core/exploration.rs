use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::resources::prelude::Colors;

const BEAM_MAX_DISTANCE: f32 = 1_600.0;
const RESONANCE_MIN_DISTANCE: f32 = 160.0;

pub struct Exploration {
    pub beam: bool,
    pub resonance: bool,
}

#[derive(Component)]
pub struct BeamMarker;

#[derive(Component)]
pub struct ResonanceMarker;

#[derive(Component)]
pub struct SourceResonanceMarker;

#[derive(Component)]
pub struct TargetResonanceMarker;

pub fn handle_input(keyboard_input: &Input<KeyCode>, exploration: &mut Exploration) {
    exploration.beam = keyboard_input.just_pressed(KeyCode::Space);
}

#[must_use]
pub fn beam_reflected(source: Vec2, target: Vec2) -> bool {
    source.distance(target) <= BEAM_MAX_DISTANCE
}

pub fn beam(commands: &mut Commands, source: Vec2, target: Vec2) {
    let distance = source.distance(target);
    let beam = shapes::Line(source, target);
    let alpha = ((BEAM_MAX_DISTANCE - distance) / BEAM_MAX_DISTANCE).powf(2.0) - 0.008;
    let color = *Colors::PRIMARY.clone().set_a(alpha);

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &beam,
            DrawMode::Stroke(StrokeMode::new(color, 2.0)),
            Transform::from_xyz(0.0, 0.0, 4.0),
        ))
        .insert(BeamMarker);
}

#[must_use]
pub fn in_resonance(source: Vec2, target: Vec2) -> bool {
    source.distance(target) <= RESONANCE_MIN_DISTANCE
}

pub fn resonance(commands: &mut Commands, source: Vec2, target: Vec2) {
    let source_circle = shapes::Circle {
        radius: 160.0,
        center: Vec2::ZERO,
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &source_circle,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Colors::TRANSPARENT),
                outline_mode: StrokeMode::new(Colors::PRIMARY, 2.0),
            },
            Transform::from_xyz(source.x, source.y, 4.0),
        ))
        .insert(ResonanceMarker)
        .insert(SourceResonanceMarker);

    let target_circle = shapes::Circle {
        radius: 160.0,
        center: Vec2::ZERO,
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &target_circle,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Colors::TRANSPARENT),
                outline_mode: StrokeMode::new(Colors::LIGHT, 2.0),
            },
            Transform::from_xyz(target.x, target.y, 4.0),
        ))
        .insert(ResonanceMarker)
        .insert(TargetResonanceMarker);
}
