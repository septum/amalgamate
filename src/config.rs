use bevy::{
    prelude::{Plugin as BevyPlugin, *},
    window::WindowMode,
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "amalgamate".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        });
    }
}
