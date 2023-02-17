use crate::shapes::Shape;
use crate::{ColourChannel, LightColour, PixelColour, Point, Vector3D};

#[derive(Copy, Clone)]
pub struct LightSource {
    pub position: Point,
    pub colour: LightColour,
}

impl Default for LightSource {
    fn default() -> Self {
        Self {
            position: Point::new(-500.0, -350.0, -350.0),
            colour: LightColour::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3D,
}

impl Ray {
    pub fn point(&self, t: f64) -> Point {
        self.origin + (self.direction * t)
    }
}

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    t: f64,
    point: Point,
    object: &'a dyn Shape,
    ray: &'a Ray,
    light_source: LightSource,
    is_inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn new(
        t: f64,
        point: Point,
        object: &'a impl Shape,
        ray: &'a Ray,
        light_source: LightSource,
        is_inside: bool,
    ) -> Self {
        Self {
            t,
            point,
            object,
            ray,
            light_source,
            is_inside,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn object(&self) -> &'a dyn Shape {
        self.object
    }

    pub fn ray(&self) -> &'a Ray {
        self.ray
    }

    pub fn light_source(&self) -> LightSource {
        self.light_source
    }

    pub fn phong(&self, _pixel_point: &Point, ambient_coefficient: f64) -> PixelColour {
        self.phong_diffuse() + self.phong_ambient(ambient_coefficient)
    }

    fn phong_ambient(&self, ambient_coefficient: f64) -> PixelColour {
        if self.is_inside {
            PixelColour::new(
                self.phong_ambient_colour_channel(ColourChannel::Red, ambient_coefficient),
                self.phong_ambient_colour_channel(ColourChannel::Green, ambient_coefficient),
                self.phong_ambient_colour_channel(ColourChannel::Blue, ambient_coefficient),
            ) / 2
        } else {
            PixelColour::new(
                self.phong_ambient_colour_channel(ColourChannel::Red, ambient_coefficient),
                self.phong_ambient_colour_channel(ColourChannel::Green, ambient_coefficient),
                self.phong_ambient_colour_channel(ColourChannel::Blue, ambient_coefficient),
            )
        }
    }

    fn phong_ambient_colour_channel(&self, channel: ColourChannel, ambient_coefficient: f64) -> u8 {
        let colour_k = self.object.material().ambient_k(ambient_coefficient).colour(&channel);
        let colour_l = self.light_source.colour.colour(&channel);
        (colour_k * colour_l * 255.0) as u8
    }

    fn phong_diffuse(&self) -> PixelColour {
        let mut direction_l = self.light_source.position - self.point;
        direction_l.normalise();
        let direction_n = self.object.surface_normal(&self.point);
        let n_l_dot = direction_l.dot(&direction_n);
        PixelColour::new(
            self.phong_diffuse_colour_channel(ColourChannel::Red, n_l_dot),
            self.phong_diffuse_colour_channel(ColourChannel::Green, n_l_dot),
            self.phong_diffuse_colour_channel(ColourChannel::Blue, n_l_dot),
        )
    }

    fn phong_diffuse_colour_channel(
        &self,
        channel: ColourChannel,
        // direction_l: &Vector3D,
        // direction_n: &Vector3D,
        n_l_dot: f64,
    ) -> u8 {
        let diffuse_k = self.object().material().colour();
        let colour_k = diffuse_k.colour(&channel);
        let colour_l = self.light_source.colour.colour(&channel);

        let n_l_dot = if n_l_dot < 0.0 {
            0.0
        } else if n_l_dot > 1.0 {
            1.0
        } else {
            n_l_dot
        };

        (colour_l * colour_k * n_l_dot * 255.0) as u8
    }
}
