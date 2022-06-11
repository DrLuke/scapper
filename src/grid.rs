use bevy::prelude::*;

#[derive(Component)]
struct Grid;

pub fn spawn_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn()
        .insert(Grid {})
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("grid_1080p.png"),
            transform: Transform::from_xyz(0.,0.,0.),
            ..default()
        })
    ;
}

pub fn startup() -> SystemSet
{
    SystemSet::new()
        .with_system(spawn_grid)
}