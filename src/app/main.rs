use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::Image;
use ray_tracing::{Camera, IMG_HEIGHT, IMG_SIZE, IMG_WIDTH, PixelColour, render, Sphere, Vector3D};
use relm4::{AppUpdate, Model, RelmApp, Widgets, WidgetPlus, Sender, send};
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use image::{EncodableLayout, RgbaImage};

pub fn main() {
    let mut model = AppModel{
        shapes: vec![Sphere::default()],
        camera: Camera::default(),
        canvas: RgbaImage::new(IMG_WIDTH, IMG_HEIGHT),
        image: Pixbuf::new(
            Colorspace::Rgb,
            true,
            8,
            IMG_SIZE as i32,
            IMG_SIZE as i32,
        ).unwrap()
    };
    model.render();
    let app = RelmApp::new(model);
    app.run();
}

//TODO: add index to msg for multiple sphere manipulation
#[derive(Debug)]
enum AppMsg {
    AdjustPosition(Vector3D),
    AdjustRadius(f64),
    ChangeColour(PixelColour),
}

struct AppModel {
    shapes: Vec<Sphere>,
    camera: Camera,
    canvas: RgbaImage,
    image: Pixbuf,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppModel {
    pub fn render(&mut self) {
        render(&mut self.canvas, &self.camera, &self.shapes);
        self.image = Pixbuf::from_bytes(
            &self.canvas.as_bytes().into(),
            Colorspace::Rgb,
            true,
            8,
            IMG_SIZE as i32,
            IMG_SIZE as i32,
            (IMG_SIZE * 4) as i32,
        );
    }
}

impl AppUpdate for AppModel {
    fn update(
        &mut self,
        msg: Self::Msg,
        _components: &Self::Components,
        _sender: Sender<Self::Msg>,
    ) -> bool {
        match msg {
            AppMsg::AdjustPosition(new_position) => {
                let i = 0;
                self.shapes[i].set_position(new_position);
                self.render();
            }
            AppMsg::AdjustRadius(delta) => {
                let i = 0;
                self.shapes[i].adjust_radius(delta);
                self.render();
            }
            AppMsg::ChangeColour(new_colour) => {
                let i = 0;
                self.shapes[i].set_colour(&new_colour);
                self.render();
            }
        }
        true
    }
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        gtk::ApplicationWindow {
            set_title: Some("Ray Tracer"),
            set_default_width: 1000,
            set_default_height: 1000,
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                append = &gtk::Picture {
                    set_pixbuf: watch! {Some(&model.image)}
                },
                append = &gtk::Button {
                    set_label: "Bigger",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::AdjustRadius(1.0));
                    },
                },
                append = &gtk::Label {
                    set_margin_all: 5,
                    set_label: watch! { &format!("Radius: {}", model.shapes[0].radius)}
                },
                append = &gtk::Button {
                    set_label: "Smaller",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::AdjustRadius(-1.0));
                    },
                },
            },
        }
    }
}
