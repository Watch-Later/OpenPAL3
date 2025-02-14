use crate::directors::{
    sce_vm::{SceCommand, SceState},
    SceneManagerExtensions,
};
use imgui::Ui;
use radiance::{
    math::Vec3,
    scene::{Entity, SceneManager},
};

#[derive(Debug, Clone)]
pub struct SceCommandHyFly {
    position_x: f32,
    position_y: f32,
    position_z: f32,
}

impl SceCommand for SceCommandHyFly {
    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        let entity = scene_manager.get_resolved_role_mut(state, 5).unwrap();
        entity.transform_mut().set_position(&Vec3::new(
            self.position_x,
            self.position_y,
            self.position_z,
        ));
        entity.idle();
        true
    }
}

impl SceCommandHyFly {
    pub fn new(position_x: f32, position_y: f32, position_z: f32) -> Self {
        Self {
            position_x,
            position_y,
            position_z,
        }
    }
}
