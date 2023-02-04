use crate::{LightColour, LightSource, PixelColour};

// TODO: change all to private
#[derive(Copy, Clone)]
pub struct Material {
    ambient_k: LightColour,
    diffuse_k: LightColour,
    specular_k: LightColour,
    light_source: LightSource,
}

impl Material {
    pub fn new(
        ambient: LightColour,
        diffuse: LightColour,
        specular: LightColour,
        light_source: LightSource,
    ) -> Self {
        Self {
            ambient_k: ambient,
            diffuse_k: diffuse,
            specular_k: specular,
            light_source,
        }
    }

    pub fn ambient_k(&self) -> LightColour {
        self.ambient_k
    }

    pub fn diffuse_k(&self) -> LightColour {
        self.diffuse_k
    }

    pub fn specular_k(&self) -> LightColour {
        self.specular_k
    }

    pub fn light_source(&self) -> LightSource {
        self.light_source
    }

    pub fn set_ambient(&mut self, new_colour: &PixelColour) {
        self.ambient_k = new_colour.to_light_colour();
    }

    pub fn set_diffuse(&mut self, new_colour: &PixelColour) {
        self.diffuse_k = new_colour.to_light_colour();
    }

    pub fn set_specular(&mut self, new_colour: &PixelColour) {
        self.specular_k = new_colour.to_light_colour();
    }
}
