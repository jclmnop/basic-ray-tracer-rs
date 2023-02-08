use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::prelude::*;
use gtk::{CheckButton, Image, Orientation};
use gtk::gdk::RGBA;
use gtk::glib::MainContext;
use image::{EncodableLayout, RgbaImage};
use ray_tracing::{render, Camera, PixelColour, Sphere, Vector3D, LightColour, IMG_HEIGHT, IMG_SIZE, IMG_WIDTH, Point};
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};
use relm4::factory::{Factory, FactoryPrototype, FactoryVec, FactoryView};
use relm4_macros::view;
use seq_macro::seq;
use tracker::track;

pub fn main() {
    // gtk::init().unwrap();
    let mut model = AppModel {
        shapes: vec![
            Sphere::default(),
            Sphere::default_with_pos(Point::new(100.0, 0.0, 50.0))
        ],
        camera: Camera::default(),
        canvas: RgbaImage::new(IMG_WIDTH, IMG_HEIGHT),
        image: Pixbuf::new(
            Colorspace::Rgb,
            true,
            8,
            IMG_SIZE as i32,
            IMG_SIZE as i32,
        )
        .unwrap(),
        current_index: 0,
        tracker: 0,
    };
    model.render();
    let app = RelmApp::new(model);
    app.run();
    let foo = gtk::Scale::default();
}

//TODO: add index to msg for multiple sphere manipulation
#[derive(Debug)]
enum AppMsg {
    AdjustPosition(Vector3D),
    AdjustRadius(f64),
    ChangeColour(PixelColour),
    SelectSphere(usize),
}

#[track]
struct AppModel {
    #[tracker::do_not_track]
    shapes: Vec<Sphere>,
    #[tracker::do_not_track]
    camera: Camera,
    #[tracker::do_not_track]
    canvas: RgbaImage,
    #[tracker::do_not_track]
    image: Pixbuf,
    current_index: usize,
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
        self.reset();
        match msg {
            AppMsg::AdjustPosition(new_position) => {
                let i = self.current_index;
                self.shapes[i].set_position(new_position);
                self.render();
            }
            AppMsg::AdjustRadius(delta) => {
                let i = self.current_index;
                self.shapes[i].adjust_radius(delta);
                self.render();
            }
            AppMsg::ChangeColour(new_colour) => {
                let i = self.current_index;
                println!("{new_colour}");
                self.shapes[i].set_colour(&new_colour);
                self.render();
            }
            AppMsg::SelectSphere(index) => {
                println!("{index}");
                self.set_current_index(index);
            }
        }
        true
    }
}
//
// macro_rules! radio_buttons {
//     ($i:literal) => {
//
//     };
// }

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
                    set_label: watch! {
                        &format!(
                            "Radius: {}",
                            model.shapes[model.current_index].radius
                        )
                    }
                },
                append = &gtk::Button {
                    set_label: "Smaller",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::AdjustRadius(-1.0));
                    },
                },
                append: red = &gtk::Scale {
                    set_range(255.0): 0.0,
                    set_value: track!(
                        model.changed(AppModel::current_index()),
                        model.shapes[model.current_index].material.diffuse_k.x * 255.0
                    ),
                    connect_value_changed[
                        sender: Sender<AppMsg> = sender.clone(),
                        diffuse_k = model.shapes[model.current_index].material.diffuse_k
                    ] => move |s| {
                        let v = s.value() as u8;
                        // let i = &model.current_index;
                        // let ambient_k = &model.shapes[*i].material.ambient_k;
                        let mut new_colour = PixelColour::from_light_colour(&diffuse_k);
                        new_colour.x = v;
                        send!(sender, AppMsg::ChangeColour(new_colour));
                    }
                },
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 10,
                    set_spacing: 10,

                    append: root_button = &gtk::CheckButton {
                        set_label: Some("Sphere 0"),
                        connect_toggled(sender) => move |_| {
                            send!(sender, AppMsg::SelectSphere(0));
                        }
                    },
                    append = &gtk::CheckButton {
                        set_label: Some("Sphere 1"),
                        set_group: Some(&root_button),
                        connect_toggled(sender) => move |_| {
                            send!(sender, AppMsg::SelectSphere(1));
                        }
                    },
                },
            }
        }
    }
}
