use env_logger::Builder;
use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::prelude::*;
use ray_tracing::{
    render, timeit, Camera, ColourChannel, Point, Sphere, BURNT_ORANGE,
    IMG_SIZE, ZIMA_BLUE,
};
use relm4::{
    send, set_global_css_from_file, AppUpdate, Model, RelmApp, Sender,
    WidgetPlus, Widgets,
};
use std::path::Path;
use tracker::track;

// Render time higher than 40ms means a framerate less than 25fps
const RENDER_WARN_MS: u128 = 40;
const CAMERA_WARN_MS: u128 = 1;

pub fn main() {
    setup_logging();
    let mut model = AppModel {
        shapes: vec![
            Sphere::default(),
            Sphere::default_with_pos(Point::new(100.0, 100.0, 200.0)),
            Sphere::default_with_pos(Point::new(200.0, 200.0, 400.0)),
            Sphere::new_with_colour(
                Point::new(-150.0, -50.0, 200.0),
                50.0,
                ZIMA_BLUE,
            ),
            Sphere::new_with_colour(
                Point::new(34.0, 100.0, -150.0),
                50.0,
                BURNT_ORANGE,
            ),
        ],
        camera: Camera::default(),
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
        is_light_selected: false,
    };
    model.render();
    let app = RelmApp::new(model);
    set_global_css_from_file(Path::new("./src/app/resources/style.css"));
    app.run();
}

fn setup_logging() {
    let mut log_builder = Builder::new();
    log_builder.filter_level(log::LevelFilter::Warn);
    log_builder.parse_env("LOG");
    log_builder.init();
}

#[derive(Debug)]
enum AppMsg {
    ChangePosition(Axis, f64),
    AdjustRadius(f64),
    ChangeColour(ColourChannel, f64),
    SelectSphere(usize),
    MoveX(f64),
    MoveY(f64),
    ResetCamera(RotationAxis),
    SetAmbient(f64),
    SelectLight,
}

#[derive(Debug)]
enum RotationAxis {
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
    Z,
}

#[track]
struct AppModel {
    #[tracker::do_not_track]
    shapes: Vec<Sphere>,
    #[tracker::do_not_track]
    camera: Camera,
    #[tracker::do_not_track]
    image: Pixbuf,
    current_index: usize,
    is_light_selected: bool,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppModel {
    pub fn render(&mut self) {
        let render_time = timeit!({
            render(&mut self.image, &self.camera, &self.shapes);
        })
        .as_millis();
        if render_time > RENDER_WARN_MS {
            log::warn!("Render time: {render_time}ms");
        } else {
            log::info!("Render time: {render_time}ms");
        }
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
                if !self.is_light_selected {
                    match axis {
                        Axis::X => self.shapes[i].set_x(v),
                        Axis::Y => self.shapes[i].set_y(v),
                        Axis::Z => self.shapes[i].set_z(v),
                    }
                } else {
                    match axis {
                        Axis::X => self.camera.light_source.set_x(v),
                        Axis::Y => self.camera.light_source.set_y(v),
                        Axis::Z => self.camera.light_source.set_z(v),
                    }
                }
                self.render();
            }
            AppMsg::AdjustRadius(delta) => {
                let i = self.current_index;
                if !self.is_light_selected {
                    self.shapes[i].adjust_radius(delta);
                }
                self.render();
            }
            AppMsg::ChangeColour(channel, new_colour) => {
                let i = self.current_index;
                // println!("{new_colour}");
                if !self.is_light_selected {
                    self.shapes[i].set_colour_channel(&channel, new_colour as u8);
                } else {
                    self.camera.light_source.set_colour_channel(&channel, new_colour as u8);
                }
                self.render();
            }
            AppMsg::SelectSphere(index) => {
                // println!("{index}");
                self.is_light_selected = false;
                if self.current_index != index {
                    self.set_current_index(index);
                } else {
                    self.tracker += 1;
                }
            }
            AppMsg::SelectLight => {
                println!("Light selected");
                self.is_light_selected = true;
                self.tracker += 1;
            }
            AppMsg::MoveX(x) => {
                // println!("MoveX: {x}");
                let camera_setup_time = timeit!({
                    self.camera.move_x(x);
                })
                .as_millis();
                if camera_setup_time > CAMERA_WARN_MS {
                    log::warn!("Camera Setup Time: {camera_setup_time}ms");
                } else {
                    log::info!("Camera Setup Time: {camera_setup_time}ms");
                }
                self.render();
            }
            AppMsg::MoveY(y) => {
                // println!("MoveY: {y}");
                let camera_setup_time = timeit!({
                    self.camera.move_y(y);
                })
                .as_millis();
                if camera_setup_time > CAMERA_WARN_MS {
                    log::warn!("Camera Setup Time: {camera_setup_time}ms");
                } else {
                    log::info!("Camera Setup Time: {camera_setup_time}ms");
                }
                self.render();
            }
            AppMsg::ResetCamera(axis) => {
                match axis {
                    RotationAxis::Horizontal => {
                        self.camera.reset_x();
                    }
                    RotationAxis::Vertical => {
                        self.camera.reset_y();
                    }
                    RotationAxis::Both => {
                        self.camera.reset_vrp();
                    }
                }
                self.render();
            }
            AppMsg::SetAmbient(v) => {
                self.camera.set_ambient_coefficient(v);
                self.render()
            }
        }
        true
    }
}

const UPPER_BOUND_POS: f64 = (IMG_SIZE as f64) / 2.0;
const LOWER_BOUND_POS: f64 = -((IMG_SIZE as f64) / 2.0);

// This code is disgusting and should never be seen
#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        gtk::ApplicationWindow {
            set_title: Some("Ray Tracer"),
            set_default_width: 800,
            set_default_height: 1000,
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
                        append = &gtk::CheckButton {
                            set_label: Some("Sphere 4"),
                            set_group: Some(&root_button),
                            connect_toggled(sender) => move |_| {
                                send!(sender, AppMsg::SelectSphere(3));
                            }
                        },
                        append = &gtk::CheckButton {
                            set_label: Some("Sphere 5"),
                            set_group: Some(&root_button),
                            connect_toggled(sender) => move |_| {
                                send!(sender, AppMsg::SelectSphere(4));
                            }
                        },
                        append = &gtk::CheckButton {
                            set_label: Some("Light Source"),
                            set_group: Some(&root_button),
                            connect_toggled(sender) => move |_| {
                                send!(sender, AppMsg::SelectLight);
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

                            append: red = &gtk::Scale {
                                set_orientation: gtk::Orientation::Vertical,
                                set_inverted: true,
                                set_valign: gtk::Align::Fill,
                                set_range: args!(0.0, 255.0),
                                set_value: track!(
                                    model.changed(AppModel::current_index()),
                                    if !model.is_light_selected {
                                        model.shapes[model.current_index].material.colour.x * 255.0
                                    } else {
                                        model.camera.light_source.colour.x * 255.0
                                    }
                                ),
                                connect_value_changed[
                                    sender: Sender<AppMsg> = sender.clone(),
                                ] => move |s| {
                                    let v = s.value();
                                    send!(sender, AppMsg::ChangeColour(ColourChannel::Red, v));
                                }
                            },

                            append: green = &gtk::Scale {
                                set_orientation: gtk::Orientation::Vertical,
                                set_inverted: true,
                                set_valign: gtk::Align::Fill,
                                set_range: args!(0.0, 255.0),
                                set_value: track!(
                                    model.changed(AppModel::current_index()),
                                    if !model.is_light_selected {
                                        model.shapes[model.current_index].material.colour.y * 255.0
                                    } else {
                                        model.camera.light_source.colour.y * 255.0
                                    }
                                ),
                                connect_value_changed[
                                    sender: Sender<AppMsg> = sender.clone(),
                                ] => move |s| {
                                    let v = s.value();
                                    send!(sender, AppMsg::ChangeColour(ColourChannel::Green, v));
                                }
                            },

                            append: blue = &gtk::Scale {
                                set_orientation: gtk::Orientation::Vertical,
                                set_inverted: true,
                                set_valign: gtk::Align::Fill,
                                set_range: args!(0.0, 255.0),
                                set_value: track!(
                                    model.changed(AppModel::current_index()),
                                    if !model.is_light_selected {
                                        model.shapes[model.current_index].material.colour.z * 255.0
                                    } else {
                                        model.camera.light_source.colour.z * 255.0
                                    }
                                ),
                                connect_value_changed[
                                    sender: Sender<AppMsg> = sender.clone(),
                                ] => move |s| {
                                    let v = s.value();
                                    send!(sender, AppMsg::ChangeColour(ColourChannel::Blue, v));
                                }
                            },
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

                        append: x_label = &gtk::Label {
                            set_halign: gtk::Align::Center,
                            set_label: "X"
                        },

                        append: x_pos = &gtk::Scale {
                            set_halign: gtk::Align::Fill,
                            set_orientation: gtk::Orientation::Horizontal,
                            set_draw_value: true,
                            set_range: args!(LOWER_BOUND_POS, UPPER_BOUND_POS),
                            set_value: track!(
                                model.changed(AppModel::current_index()),
                                if !model.is_light_selected {
                                    model.shapes[model.current_index].center.x
                                } else {
                                    model.camera.light_source.position.x
                                }
                                // model.shapes[model.current_index].center.x
                            ),
                            connect_value_changed[
                                sender: Sender<AppMsg> = sender.clone(),
                            ] => move |s| {
                                let v = s.value();
                                send!(sender, AppMsg::ChangePosition(Axis::X, v));
                            }
                        },
                    },
                    append: y_box = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        append: y_label = &gtk::Label {
                            set_halign: gtk::Align::Center,
                            set_label: "Y"
                        },
                        append: y_pos = &gtk::Scale {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_draw_value: true,
                            set_range: args!(LOWER_BOUND_POS, UPPER_BOUND_POS),
                            set_value: track!(
                                model.changed(AppModel::current_index()),
                                if !model.is_light_selected {
                                    model.shapes[model.current_index].center.y
                                } else {
                                    model.camera.light_source.position.y
                                }
                                // model.shapes[model.current_index].center.y
                            ),
                            connect_value_changed[
                                sender: Sender<AppMsg> = sender.clone(),
                            ] => move |s| {
                                let v = s.value();
                                send!(sender, AppMsg::ChangePosition(Axis::Y, v));
                            }
                        },
                    },
                    append: z_box = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        append: z_label = &gtk::Label {
                            set_halign: gtk::Align::Center,
                            set_label: "Z"
                        },
                        append: z_pos = &gtk::Scale {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_draw_value: true,
                            set_range: args!(LOWER_BOUND_POS, UPPER_BOUND_POS),
                            set_value: track!(
                                model.changed(AppModel::current_index()),
                                if !model.is_light_selected {
                                    model.shapes[model.current_index].center.z
                                } else {
                                    model.camera.light_source.position.z
                                }
                                // model.shapes[model.current_index].center.z
                            ),
                            connect_value_changed[
                                sender: Sender<AppMsg> = sender.clone(),
                            ] => move |s| {
                                let v = s.value();
                                send!(sender, AppMsg::ChangePosition(Axis::Z, v));
                            }
                        },
                    },
                },

                append = &gtk::Separator::new(gtk::Orientation::Horizontal) {},
                append: camera_controls = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_halign: gtk::Align::Center,
                    append: vertical_controls = &gtk::Scale {
                        set_orientation: gtk::Orientation::Vertical,
                        set_range: args!(-10.0, 10.0),
                        set_increments: args!(1.0, 1.0),
                        // set_slider_size_fixed: true,
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
                        set_range: args!(-10.0, 10.0),
                        set_increments: args!(1.0, 1.0),
                        // set_slider_size_fixed: true,
                        set_size_request: args!(100, -1),
                        set_inverted: true,
                        connect_value_changed(sender) => move |s| {
                            let v = s.value();
                            if v != 0.0 {
                                send!(sender, AppMsg::MoveX(v));
                                s.set_value(0.0);
                            }
                        },
                    },
                },

                append = &gtk::Label {
                    set_margin_all: 5,
                    set_label: watch! {
                        &format!(
                            "Camera Coords ({:.0}, {:.0}, {:.0})\nRotation: ({:.0}, {:.0})",
                            model.camera.vrp().x,
                            model.camera.vrp().y,
                            model.camera.vrp().z,
                            model.camera.h_rotation(),
                            model.camera.v_rotation(),
                        )
                    }
                },
                append: reset_buttons = &gtk::Box {
                    set_halign: gtk::Align::Center,
                    set_orientation: gtk::Orientation::Horizontal,
                    append = &gtk::Button {
                        set_label: "Reset Vertical",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::ResetCamera(RotationAxis::Vertical));
                        },
                    },
                    append = &gtk::Button {
                        set_label: "Reset Horizontal",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::ResetCamera(RotationAxis::Horizontal));
                        },
                    },
                    append = &gtk::Button {
                        set_label: "Reset Both",
                        connect_clicked(sender) => move |_| {
                            send!(sender, AppMsg::ResetCamera(RotationAxis::Both));
                        },
                    },
                },
                append = &gtk::Separator::new(gtk::Orientation::Horizontal) {},
                append = &gtk::Label {
                    set_margin_all: 5,
                    set_halign: gtk::Align::Center,
                    set_label: "Ambient Light",
                },

                append = &gtk::Separator::new(gtk::Orientation::Horizontal) {},
                append: ambient_controls = &gtk::Scale {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_range: args!(0.0, 1.0),
                    set_increments: args!(0.01, 0.01),
                    // set_slider_size_fixed: true,
                    set_value: model.camera.ambient_coefficient(),
                    // set_size_request: args!(100, -1),
                    connect_value_changed(sender) => move |s| {
                        let v = s.value();
                        send!(sender, AppMsg::SetAmbient(v));
                    },
                },
            }
        }
    }
}
