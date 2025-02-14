use super::{Director, Scene};
use imgui::Ui;
use std::{cell::RefCell, rc::Rc};

pub trait SceneManager {
    fn update(&mut self, ui: &Ui, delta_sec: f32);
    fn scene(&self) -> Option<&dyn Scene>;
    fn scene_mut(&mut self) -> Option<&mut dyn Scene>;
    fn scenes(&self) -> &[Box<dyn Scene>];
    fn scenes_mut(&mut self) -> Vec<&mut dyn Scene>;

    fn set_view_extent(&mut self, extent: (u32, u32));
    fn director(&self) -> Option<Rc<RefCell<dyn Director>>>;
    fn set_director(&mut self, director: Rc<RefCell<dyn Director>>);
    fn push_scene(&mut self, scene: Box<dyn Scene>);
    fn pop_scene(&mut self) -> Option<Box<dyn Scene>>;
    fn unload_all_scenes(&mut self);
    fn unset_director(&mut self);
}

pub struct DefaultSceneManager {
    director: Option<Rc<RefCell<dyn Director>>>,
    scenes: Vec<Box<dyn Scene>>,
    view_extent: (u32, u32),
}

impl DefaultSceneManager {
    pub fn new() -> Self {
        DefaultSceneManager {
            director: None,
            scenes: vec![],
            view_extent: (1024, 768),
        }
    }
}

macro_rules! scene_mut {
    ($self: ident) => {
        $self.scenes.last_mut().and_then(|x| Some(&mut **x))
    };
}

impl SceneManager for DefaultSceneManager {
    fn update(&mut self, ui: &Ui, delta_sec: f32) {
        if let Some(d) = self.director.as_ref() {
            let director = d.clone();
            let new_director = director.borrow_mut().update(self, ui, delta_sec);
            if let Some(d) = new_director {
                d.borrow_mut().activate(self);
                self.director = Some(d);
            }
        }

        if let Some(s) = scene_mut!(self) {
            s.update(delta_sec);
        }
    }

    fn scene(&self) -> Option<&dyn Scene> {
        self.scenes.last().and_then(|x| Some(&**x))
    }

    fn scene_mut(&mut self) -> Option<&mut dyn Scene> {
        scene_mut!(self)
    }

    fn scenes(&self) -> &[Box<dyn Scene>] {
        &self.scenes
    }

    fn scenes_mut(&mut self) -> Vec<&mut dyn Scene> {
        self.scenes.iter_mut().map(|s| s.as_mut()).collect()
    }

    fn set_view_extent(&mut self, extent: (u32, u32)) {
        self.view_extent = extent;
    }

    fn director(&self) -> Option<Rc<RefCell<dyn Director>>> {
        Some(self.director.as_ref()?.clone())
    }

    fn set_director(&mut self, director: Rc<RefCell<dyn Director>>) {
        director.borrow_mut().activate(self);
        self.director = Some(director);
    }

    fn push_scene(&mut self, scene: Box<dyn Scene>) {
        self.scenes.push(scene);

        let scene_mut = self.scene_mut().unwrap();
        scene_mut.load();
    }

    fn pop_scene(&mut self) -> Option<Box<dyn Scene>> {
        let mut scene = self.scenes.pop();
        if let Some(s) = scene.as_mut() {
            s.unload();
        }

        scene
    }

    fn unload_all_scenes(&mut self) {
        while self.pop_scene().is_some() {}
    }

    fn unset_director(&mut self) {
        self.director = None;
    }
}

impl Drop for DefaultSceneManager {
    fn drop(&mut self) {
        self.unload_all_scenes();
        self.unset_director();
    }
}
