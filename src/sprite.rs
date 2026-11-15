use crate::{instance::{Instance, QueuedInstance}, texture::TextureId};

#[derive(Debug)]
pub struct Sprite {
    pub texture: TextureId,
    pub src: [f32; 4], // 1.0 to 0.0
    pub size: [f32; 2], //size in world units
    pub tint: [f32; 4], 
    pub layer: i32, 
}

impl Sprite {
    pub fn new(
        texture: TextureId,
        texture_size: [f32; 2],
        src: [f32; 4], //in pixels
        size: [f32; 2],
        tint: [f32; 4], //1.0 is no tint
        layer: i32,
    ) -> Self {
        Self { 
            texture,
            src: [
                src[0] / texture_size[0],
                src[1] / texture_size[1],
                src[2] / texture_size[0],
                src[3] / texture_size[1],
            ],
            size,
            tint,
            layer, 
        }
    }

    pub fn to_queued_instance(&self, position: [f32; 2], rotation: f32) -> QueuedInstance {
        let instance = Instance {
            position,
            rotation,
            size: self.size,
            uv_rect: self.src,
            color: self.tint,
        };

        QueuedInstance {
            layer: self.layer,
            texture: self.texture.index,
            instance,
        }
    }
}
