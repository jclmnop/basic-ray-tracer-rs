use crate::{ColourChannel, LightColour, PixelColour};

// TODO: specular lighting (optional)

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
    specular_coefficient: f64,
    pub colour: LightColour,
}

impl Material {
    pub fn new(
        ambient_coefficient: f64,
        colour: LightColour,
        specular_coefficient: f64,
    ) -> Self {
        Self {
            ambient_coefficient,
            specular_coefficient,
            colour,
        }
    }

    pub fn default_with_colour(colour: PixelColour) -> Self {
        let mut material = Self::default();
        material.colour = colour.to_light_colour();
        material
    }

    pub fn ambient_k(&self) -> LightColour {
        self.colour * self.ambient_coefficient
    }

    pub fn colour(&self) -> LightColour {
        self.colour
    }

    pub fn specular_k(&self) -> LightColour {
        self.colour * self.specular_coefficient
    }

    pub fn set_ambient_coefficient(&mut self, new_coefficient: f64) {
        self.ambient_coefficient = new_coefficient;
    }

    pub fn set_colour(&mut self, new_colour: &PixelColour) {
        self.colour = new_colour.to_light_colour();
    }

    pub fn set_specular_coefficient(&mut self, new_coefficient: f64) {
        self.specular_coefficient = new_coefficient;
    }

    pub fn set_colour_channel(&mut self, channel: &ColourChannel, value: u8) {
        let value: f64 = value as f64 / 255.0;
        match channel {
            ColourChannel::Red => {
                self.colour.x = value;
            }
            ColourChannel::Green => {
                self.colour.y = value;
            }
            ColourChannel::Blue => {
                self.colour.z = value;
            }
        };
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            specular_coefficient: DEFAULT_SPECULAR_COEFFICIENT,
            ambient_coefficient: DEFAULT_AMBIENT_COEFFICIENT,
            colour: BURGUNDY.to_light_colour(),
        }
    }
}
