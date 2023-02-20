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

const DEFAULT_SPECULAR_COEFFICIENT: f64 = 10.0;

// TODO: move ambient coefficient to camera? or somewhere else
#[derive(Copy, Clone)]
pub struct Material {
    specular_coefficient: f64,
    specular_k: LightColour,
    pub colour: LightColour,
}

impl Material {
    pub fn new(
        colour: LightColour,
        specular_coefficient: f64,
    ) -> Self {
        Self {
            specular_k: LightColour::new(1.0, 1.0, 1.0),
            specular_coefficient,
            colour,
        }
    }

    pub fn default_with_colour(colour: PixelColour) -> Self {
        let mut material = Self::default();
        material.colour = colour.to_light_colour();
        material
    }

    pub fn specular_coefficient(&self) -> f64 {
        self.specular_coefficient
    }

    pub fn specular_k(&self) -> LightColour {
        // self.specular_k
        self.colour
    }

    pub fn ambient_k(&self, ambient_coefficient: f64) -> LightColour {
        self.colour * ambient_coefficient
    }

    pub fn colour(&self) -> LightColour {
        self.colour
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
            specular_k: LightColour::new(1.0, 1.0, 1.0),
            specular_coefficient: DEFAULT_SPECULAR_COEFFICIENT,
            colour: BURGUNDY.to_light_colour(),
        }
    }
}
