use bevy::prelude::{Plugin as BevyPlugin, *};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "amalgamate".to_string(),
            ..Default::default()
        });
    }
}
