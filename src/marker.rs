use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::prelude::shape::Icosphere;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::{PickableBundle, PickingEvent};
use bevy_prototype_lyon::prelude::*;

/// Markers are used for aligning the reference image with the 3D scan
#[derive(Component)]
pub struct Marker;

pub fn spawn_marker(
    mut commands: Commands,
)
{
    let shape = RegularPolygon {
        sides: 6,
        feature: RegularPolygonFeature::Radius(20.0),
        ..default()
    };

    commands.spawn()
        .insert(Marker {})
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::RED),
                outline_mode: StrokeMode::new(Color::WHITE, 5.0),
            },
            Transform::from_translation(Vec3::new(0., 0., 1.)),
        ))
        .insert_bundle(PickableBundle::default())
    ;
}

/// Entity is currently being dragged
#[derive(Component)]
pub struct Dragging;

pub fn add_dragging_system(
    mut commands: Commands,
    mut picking_events: EventReader<PickingEvent>,
    mut marker_query: Query<&Marker>,
) {
    for ev in picking_events.iter() {
        if let PickingEvent::Clicked(e) = ev {
            if let Ok(_) = marker_query.get(*e) {
                // Add dragging component if entity with marker component is clicked
                commands.entity(*e).insert(Dragging);
            }
        }
    }
}

pub fn remove_dragging_system(
    mut commands: Commands,
    mut dragging_query: Query<(Entity, &Dragging)>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_released(MouseButton::Left) {
        for (entity, _) in dragging_query.iter_mut() {
            commands.entity(entity).remove::<Dragging>();
        }
    }
}

pub fn marker_dragging_system(
    mut mouse_events: EventReader<MouseMotion>,
    mut dragging_query: Query<(&mut Transform, &Dragging)>,
)
{
    for ev in mouse_events.iter() {
        for (mut transform, _) in dragging_query.iter_mut() {
            transform.translation.x += ev.delta.x;
            transform.translation.y -= ev.delta.y;
        }
    }
}

pub fn marker_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(add_dragging_system)
        .with_system(remove_dragging_system)
        .with_system(marker_dragging_system)
}