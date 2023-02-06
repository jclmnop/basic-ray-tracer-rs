use crate::{LightColour, LightSource, PixelColour};

// Colours
pub const ZIMA_BLUE: PixelColour = PixelColour {
    x: 26,
    y: 179,
    z: 249,
};
pub const BURGUNDY: PixelColour = PixelColour {
    x: 128,
    y: 0,
    z: 32,
};
pub const BURNT_ORANGE: PixelColour = PixelColour {
    x: 204,
    y: 85,
    z: 0,
};

const DEFAULT_AMBIENT_COEFFICIENT: f64 = 0.3;
const DEFAULT_SPECULAR_COEFFECIENT: f64 = 1.0;

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

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient_k: ZIMA_BLUE.to_light_colour()
                * DEFAULT_AMBIENT_COEFFICIENT,
            diffuse_k: ZIMA_BLUE.to_light_colour(),
            specular_k: ZIMA_BLUE.to_light_colour()
                * DEFAULT_SPECULAR_COEFFECIENT,
            light_source: LightSource::default(),
        }
    }
}