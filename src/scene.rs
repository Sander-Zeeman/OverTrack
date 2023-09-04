use crate::objects::Object;

pub struct Scene {
    objects: Vec<Box<dyn Object>>
}

impl Scene {
    pub fn default() -> Scene {
        Scene { objects: vec![] }
    }

    pub fn add_object(&mut self, obj: Box<dyn Object>) {
        self.objects.push(obj);
    }
}