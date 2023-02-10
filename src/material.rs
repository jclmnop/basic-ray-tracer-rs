use crate::{ColourChannel, LightColour, PixelColour};

// TODO: refactor ambient_k fields etc:
//          - single colour field
//          - ambient + specular coefficients
//          - getter methods for ambient/specular colour which calculate

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
const DEFAULT_SPECULAR_COEFFICIENT: f64 = 1.0;

#[derive(Copy, Clone)]
pub struct Material {
    ambient_coefficient: f64,
    ambient_k: LightColour,
    pub diffuse_k: LightColour,
    specular_k: LightColour,
}

impl Material {
    pub fn new(
        ambient_coefficient: f64,
        diffuse: LightColour,
        specular: LightColour,
    ) -> Self {
        Self {
            ambient_coefficient,
            ambient_k: diffuse * ambient_coefficient,
            diffuse_k: diffuse,
            specular_k: specular,
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

    pub fn set_ambient(&mut self, new_colour: &PixelColour) {
        self.ambient_k = new_colour.to_light_colour();
    }

    pub fn set_diffuse(&mut self, new_colour: &PixelColour) {
        self.diffuse_k = new_colour.to_light_colour();
    }

    pub fn set_specular(&mut self, new_colour: &PixelColour) {
        self.specular_k = new_colour.to_light_colour();
    }

    pub fn set_colour(&mut self, new_colour: &PixelColour) {
        self.diffuse_k = new_colour.to_light_colour();
        self.ambient_k =
            new_colour.to_light_colour() * self.ambient_coefficient;
        //TODO: specular coeff etc
    }

    pub fn set_colour_channel(&mut self, channel: &ColourChannel, value: u8) {
        let value: f64 = value as f64 / 255.0;
        match channel {
            ColourChannel::Red => {
                self.diffuse_k.x = value;
                self.ambient_k.x = value * self.ambient_coefficient;
            },
            ColourChannel::Green => {
                self.diffuse_k.y = value;
                self.ambient_k.y = value * self.ambient_coefficient;
            },
            ColourChannel::Blue => {
                self.diffuse_k.z = value;
                self.ambient_k.z = value * self.ambient_coefficient;
            },
        };
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient_coefficient: DEFAULT_AMBIENT_COEFFICIENT,
            ambient_k: BURGUNDY.to_light_colour()
                * DEFAULT_AMBIENT_COEFFICIENT,
            diffuse_k: BURGUNDY.to_light_colour(),
            specular_k: BURGUNDY.to_light_colour()
                * DEFAULT_SPECULAR_COEFFICIENT,
        }
    }
}
