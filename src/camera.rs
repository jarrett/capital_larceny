use cgmath::{Vector2, Matrix4, ortho};

pub struct Camera {
    pub translation: Vector2<f32>,
    pub zoom: f32,
    pub model_view: Matrix4<f32>,
    pub projection: Matrix4<f32>,
    pub width: u16,
    pub height: u16
}

impl Camera {
    pub fn new(width: u16, height: u16, zoom: f32) -> Camera {
        let mut cam = Camera {
            translation: Vector2 {x: 0f32, y: 0f32}, zoom: zoom,
            model_view: Matrix4::identity(), projection: Matrix4::identity(),
            width: width, height: height
        };
        cam.rebuild_model_view();
        cam.rebuild_projection();
        cam
    }
    
    pub fn rebuild_model_view(&mut self) {
        // Translate model.
        self.model_view = Matrix4::from_translation(&self.translation.extend(0f32));
    }
    
    pub fn rebuild_projection(&mut self) {
        self.projection = ortho(
            self.width  as f32 / (-1f32 * self.zoom),    // Left.
            self.width  as f32 /          self.zoom ,    // Right.
            self.height as f32 /          self.zoom ,    // Bottom.
            self.height as f32 / (-1f32 * self.zoom),    // Top.
            1f32, -1f32                                  // Near, far.
        );
    }
    
    pub fn resize(&mut self, width: u16, height: u16) {
        if width != self.width || height != self.height {
            self.width = width;
            self.height = height;
            self.rebuild_projection();
        }
    }
    
    pub fn translate(&mut self, amount: Vector2<f32>) {
        self.translation = self.translation + amount;
        self.rebuild_model_view();
    }
    
    pub fn zoom(&self) -> f32 {
        self.zoom
    }
    
    pub fn zoom_by(&mut self, multiplier: f32) {
        self.zoom = self.zoom * multiplier;
        self.rebuild_projection();
    }
}