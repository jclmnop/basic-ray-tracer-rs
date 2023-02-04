#![allow(dead_code, unused_variables)]
use crate::shapes::Shape;
use crate::{ColourChannel, LightColour, PixelColour, Point, Vector3D};

#[derive(Copy, Clone)]
pub struct LightSource {
    pub position: Point,
    pub colour: LightColour,
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
    point: Point,
    object: &'a dyn Shape,
    ray: &'a Ray,
    light_source: LightSource,
}

impl<'a> Intersection<'a> {
    pub fn new(
        point: Point,
        object: &'a impl Shape,
        ray: &'a Ray,
        light_source: LightSource,
    ) -> Self {
        Self {
            point,
            object,
            ray,
            light_source,
        }
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

    pub fn phong(&self, pixel_point: &Point) -> PixelColour {
        self.phong_diffuse(pixel_point) + self.phong_ambient(pixel_point)
    }

    fn phong_ambient(&self, pixel_point: &Point) -> PixelColour {
        PixelColour::new(
            self.phong_ambient_colour_channel(ColourChannel::Red),
            self.phong_ambient_colour_channel(ColourChannel::Green),
            self.phong_ambient_colour_channel(ColourChannel::Blue),
        )
    }

    fn phong_ambient_colour_channel(&self, channel: ColourChannel) -> u8 {
        let colour_k = self.object.material().ambient_k().colour(&channel);
        let colour_l = self.light_source.colour.colour(&channel);
        (colour_k * colour_l * 255.0) as u8
    }

    fn phong_diffuse(&self, pixel_point: &Point) -> PixelColour {
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
        let diffuse_k = self.object().material().diffuse_k();
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
