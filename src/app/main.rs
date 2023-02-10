use std::path::Path;
use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::prelude::*;
use gtk::{Image, Orientation};
use image::{EncodableLayout, RgbaImage};
use ray_tracing::{render, Camera, Sphere, IMG_HEIGHT, IMG_SIZE, IMG_WIDTH, Point, ColourChannel};
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets, set_global_css_from_file};
use tracker::track;

pub fn main() {
    // gtk::init().unwrap();
    let mut model = AppModel {
        shapes: vec![
            Sphere::default(),
            Sphere::default_with_pos(Point::new(100.0, 100.0, 200.0)),
            Sphere::default_with_pos(Point::new(200.0, 200.0, 400.0)),
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
    set_global_css_from_file(Path::new("./src/aoo/resources/style.css"));
    app.run();
}

#[derive(Debug)]
enum AppMsg {
    ChangePosition(Axis, f64),
    AdjustRadius(f64),
    ChangeColour(ColourChannel, f64),
    SelectSphere(usize),
    MoveX(f64),
    MoveY(f64),
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
    Z
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
            AppMsg::ChangePosition(axis, v) => {
                let i = self.current_index;
                match axis {
                    Axis::X => {self.shapes[i].set_x(v)}
                    Axis::Y => {self.shapes[i].set_y(v)}
                    Axis::Z => {self.shapes[i].set_z(v)}
                }
                self.render();
            }
            AppMsg::AdjustRadius(delta) => {
                let i = self.current_index;
                self.shapes[i].adjust_radius(delta);
                self.render();
            }
            AppMsg::ChangeColour(channel, new_colour) => {
                let i = self.current_index;
                println!("{new_colour}");
                self.shapes[i].set_colour_channel(&channel, new_colour as u8);
                self.render();
            }
            AppMsg::SelectSphere(index) => {
                println!("{index}");
                self.set_current_index(index);
            }
            AppMsg::MoveX(x) => {
                self.camera.move_x(x);
                self.render();
            }
            AppMsg::MoveY(y) => {
                println!("{y}");
                self.camera.move_y(y);
                self.render();
            }
        }
        true
    }
}

const UPPER_BOUND_POS: f64 = (IMG_SIZE as f64) / 2.0;
const LOWER_BOUND_POS: f64 = -((IMG_SIZE as f64) / 2.0);

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        gtk::ApplicationWindow {
            set_title: Some("Ray Tracer"),
            set_default_width: 1000,
            set_default_height: 1200,
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                append = &gtk::Picture {
                    set_can_shrink: true,

                    set_pixbuf: watch! {Some(&model.image)},
                },
                append = &gtk::Separator::new(gtk::Orientation::Horizontal) {},
                append: settings = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_homogeneous: true,
                    append: radio_buttons = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_all: 10,
                        set_spacing: 10,

                        append: root_button = &gtk::CheckButton {
                            set_label: Some("Sphere 1"),
                            set_active: true,
                            connect_toggled(sender) => move |_| {
                                send!(sender, AppMsg::SelectSphere(0));
                            }
                        },
                        append = &gtk::CheckButton {
                            set_label: Some("Sphere 2"),
                            set_group: Some(&root_button),
                            connect_toggled(sender) => move |_| {
                                send!(sender, AppMsg::SelectSphere(1));
                            }
                        },
                        append = &gtk::CheckButton {
                            set_label: Some("Sphere 3"),
                            set_group: Some(&root_button),
                            connect_toggled(sender) => move |_| {
                                send!(sender, AppMsg::SelectSphere(2));
                            }
                        },
                    },
                    append = &gtk::Separator::new(gtk::Orientation::Vertical) {
                        set_halign: gtk::Align::Center,
                    },
                    append: colour_sliders = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,
                        set_valign: gtk::Align::Fill,

                        // append: red_box = &gtk::Box {
                        //     set_orientation: gtk::Orientation::Vertical,
                            append: red = &gtk::Scale {
                                set_orientation: gtk::Orientation::Vertical,
                                set_inverted: true,
                                set_valign: gtk::Align::Fill,
                                set_range: args!(0.0, 255.0),
                                set_value: track!(
                                    model.changed(AppModel::current_index()),
                                    model.shapes[model.current_index].material.diffuse_k.x * 255.0
                                ),
                                connect_value_changed[
                                    sender: Sender<AppMsg> = sender.clone(),
                                ] => move |s| {
                                    let v = s.value();
                                    send!(sender, AppMsg::ChangeColour(ColourChannel::Red, v));
                                }
                            },
                            // append: red_label = &gtk::Label {
                            //     set_valign: gtk::Align::End,
                            //     set_label: "R"
                            // },
                        // },
                        // append: green_box = &gtk::Box {
                        //     set_orientation: gtk::Orientation::Vertical,
                        //     set_valign: gtk::Align::Fill,

                            append: green = &gtk::Scale {
                                set_orientation: gtk::Orientation::Vertical,
                                set_inverted: true,
                                set_valign: gtk::Align::Fill,
                                set_range: args!(0.0, 255.0),
                                set_value: track!(
                                    model.changed(AppModel::current_index()),
                                    model.shapes[model.current_index].material.diffuse_k.y * 255.0
                                ),
                                connect_value_changed[
                                    sender: Sender<AppMsg> = sender.clone(),
                                ] => move |s| {
                                    let v = s.value();
                                    send!(sender, AppMsg::ChangeColour(ColourChannel::Green, v));
                                }
                            },

                            // append: green_label = &gtk::Label {
                            //     set_valign: gtk::Align::End,
                            //     set_label: "G"
                            // },
                        // },
                        // append: blue_box = &gtk::Box {
                        //     set_orientation: gtk::Orientation::Vertical,
                        //     set_valign: gtk::Align::Fill,

                            append: blue = &gtk::Scale {
                                set_orientation: gtk::Orientation::Vertical,
                                set_inverted: true,
                                set_valign: gtk::Align::Fill,
                                set_range: args!(0.0, 255.0),
                                set_value: track!(
                                    model.changed(AppModel::current_index()),
                                    model.shapes[model.current_index].material.diffuse_k.z * 255.0
                                ),
                                connect_value_changed[
                                    sender: Sender<AppMsg> = sender.clone(),
                                ] => move |s| {
                                    let v = s.value();
                                    send!(sender, AppMsg::ChangeColour(ColourChannel::Blue, v));
                                }
                            },

                            // append: blue_label = &gtk::Label {
                            //     set_valign: gtk::Align::End,
                            //     set_label: "B"
                            // },
                        // },
                    },
                    append = &gtk::Separator::new(gtk::Orientation::Vertical) {
                        set_halign: gtk::Align::Center,
                    },
                    append: radius_controls = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        append = &gtk::Button {
                            set_label: "Bigger",
                            connect_clicked(sender) => move |_| {
                                send!(sender, AppMsg::AdjustRadius(10.0));
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
                                send!(sender, AppMsg::AdjustRadius(-10.0));
                            },
                        },
                    },


                },
                append = &gtk::Separator::new(gtk::Orientation::Horizontal) {},

                append: position_controls = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_halign: gtk::Align::Fill,
                    append: x_box = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_halign: gtk::Align::Fill,
                        append: x_pos = &gtk::Scale {
                            set_halign: gtk::Align::Fill,
                            set_orientation: gtk::Orientation::Horizontal,
                            set_draw_value: true,
                            set_range: args!(LOWER_BOUND_POS, UPPER_BOUND_POS),
                            set_value: track!(
                                model.changed(AppModel::current_index()),
                                model.shapes[model.current_index].center.x
                            ),
                            connect_value_changed[
                                sender: Sender<AppMsg> = sender.clone(),
                            ] => move |s| {
                                let v = s.value();
                                send!(sender, AppMsg::ChangePosition(Axis::X, v));
                            }
                        },

                        append: x_label = &gtk::Label {
                            set_halign: gtk::Align::Center,
                            set_label: "x"
                        },
                    },
                    append: y_box = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        append: y_pos = &gtk::Scale {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_draw_value: true,
                            set_range: args!(LOWER_BOUND_POS, UPPER_BOUND_POS),
                            set_value: track!(
                                model.changed(AppModel::current_index()),
                                model.shapes[model.current_index].center.y
                            ),
                            connect_value_changed[
                                sender: Sender<AppMsg> = sender.clone(),
                            ] => move |s| {
                                let v = s.value();
                                send!(sender, AppMsg::ChangePosition(Axis::Y, v));
                            }
                        },

                        append: y_label = &gtk::Label {
                            set_halign: gtk::Align::Center,
                            set_label: "y"
                        },
                    },
                    append: z_box = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        append: z_pos = &gtk::Scale {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_draw_value: true,
                            set_range: args!(LOWER_BOUND_POS, UPPER_BOUND_POS),
                            set_value: track!(
                                model.changed(AppModel::current_index()),
                                model.shapes[model.current_index].center.z
                            ),
                            connect_value_changed[
                                sender: Sender<AppMsg> = sender.clone(),
                            ] => move |s| {
                                let v = s.value();
                                send!(sender, AppMsg::ChangePosition(Axis::Z, v));
                            }
                        },

                        append: z_label = &gtk::Label {
                            set_halign: gtk::Align::Center,
                            set_label: "z"
                        },
                    },
                },

                append = &gtk::Separator::new(gtk::Orientation::Horizontal) {},
                append: camera_controls = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_halign: gtk::Align::Center,
                    append: vertical_controls = &gtk::Scale {
                        set_orientation: gtk::Orientation::Vertical,
                        set_range: args!(-100.0, 100.0),
                        set_increments: args!(1.0, 1.0),
                        set_slider_size_fixed: true,
                        set_inverted: true,
                        set_size_request: args!(-1, 100),
                        connect_value_changed(sender) => move |s| {
                            let v = s.value();
                            if v != 0.0 {
                                send!(sender, AppMsg::MoveY(v));
                                s.set_value(0.0);
                            }
                        },
                    },
                    append: horizontal_controls = &gtk::Scale {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_range: args!(-100.0, 100.0),
                        set_increments: args!(1.0, 1.0),
                        set_slider_size_fixed: true,
                        set_size_request: args!(100, -1),
                        connect_value_changed(sender) => move |s| {
                            let v = s.value();
                            if v != 0.0 {
                                send!(sender, AppMsg::MoveX(v));
                                s.set_value(0.0);
                            }
                        },
                    },
                },
                append = &gtk::Separator::new(gtk::Orientation::Horizontal) {},
            }
        }
    }
}
