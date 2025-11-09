use minifb::{Window, WindowOptions};

fn rgb(red: u32, green: u32, blue: u32) -> u32 {
    (red << 16) | (green << 8) | blue
}

pub struct Visualizer {
    gui: GUI,
    pub tval: f32,
    pub lval: f32,
    pub bval: f32,
    pub rval: f32,
    pub hor_lock: bool,
    pub ver_lock: bool
}

struct GUI {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

struct Shape {
    shape: Vec<(i8, i8)>,
}

impl Visualizer {
    const KEY_SIZE: usize = 64;
    const GAP_SIZE: usize = 12;
    const WIDTH: usize = (Self::KEY_SIZE * 3) + (Self::GAP_SIZE * 4);
    const HEIGHT: usize = (Self::KEY_SIZE * 2) + (Self::GAP_SIZE * 3);
    const ROW_1: usize = Self::GAP_SIZE;
    const ROW_2: usize = Self::KEY_SIZE + (Self::GAP_SIZE * 2);
    const COL_1: usize = Self::GAP_SIZE;
    const COL_2: usize = Self::ROW_2;
    const COL_3: usize = Self::COL_2 + (Self::KEY_SIZE + Self::GAP_SIZE);

    pub fn new() -> Self {
        let gui = GUI::new(Self::WIDTH, Self::HEIGHT);

        Visualizer {
            gui,
            tval: 0.0,
            lval: 0.0,
            bval: 0.0,
            rval: 0.0,
            hor_lock: false,
            ver_lock: false
        }
    }

    pub fn draw(&mut self) {
        self.draw_container_keys();
        self.draw_values();
        self.draw_arrows();

        self.gui.update();
    }

    pub fn should_close(&mut self) -> bool {
        self.gui.should_close()
    }

    fn draw_values(&mut self) {
        let top_color = rgb(0, 215, 0);
        let bottom_color = rgb(215, 0, 0);
        let side_color = rgb(55, 155, 255);

        let top_mapped = Self::map_value(self.tval, Self::KEY_SIZE as f32, 0.0);
        let left_mapped = Self::map_value(self.lval, Self::KEY_SIZE as f32, 0.0);
        let bottom_mapped = Self::map_value(self.bval, 0.0, Self::KEY_SIZE as f32);
        let right_mapped = Self::map_value(self.rval, 0.0, Self::KEY_SIZE as f32);

        // Top Value
        self.gui.draw_rect(
            Self::COL_2,
            Self::ROW_1 + top_mapped as usize,
            Self::KEY_SIZE,
            Self::KEY_SIZE - top_mapped as usize,
            Some(top_color),
        );
        // Left Value
        self.gui.draw_rect(
            Self::COL_1 + left_mapped as usize,
            Self::ROW_2,
            Self::KEY_SIZE - left_mapped as usize,
            Self::KEY_SIZE,
            Some(side_color),
        );
        // Bottom Value
        self.gui.draw_rect(
            Self::COL_2,
            Self::ROW_2,
            Self::KEY_SIZE,
            bottom_mapped as usize,
            Some(bottom_color),
        );
        // Right Value
        self.gui.draw_rect(
            Self::COL_3,
            Self::ROW_2,
            right_mapped as usize,
            Self::KEY_SIZE,
            Some(side_color),
        );
    }

    fn map_value(value: f32, min: f32, max: f32) -> f32 {
        min + value * (max - min)
    }

    fn draw_container_keys(&mut self) {
        // Up Key
        self.draw_container_key(Self::COL_2, Self::ROW_1, self.ver_lock);
        // Left Key
        self.draw_container_key(Self::COL_1, Self::ROW_2, self.hor_lock);
        // Down Key
        self.draw_container_key(Self::COL_2, Self::ROW_2, self.ver_lock);
        // Right Key
        self.draw_container_key(Self::COL_3, Self::ROW_2, self.hor_lock);
    }

    fn draw_arrows(&mut self) {
        let half_size = Self::KEY_SIZE / 2;

        // Top Arrow
        self.draw_arrow(
            Self::COL_2 + half_size,
            Self::ROW_1 + half_size,
            Shape::arrow(Some(-90)),
        );
        // Left Arrow
        self.draw_arrow(
            Self::COL_1 + half_size,
            Self::ROW_2 + half_size,
            Shape::arrow(Some(180)),
        );
        // Down Arrow
        self.draw_arrow(
            Self::COL_2 + half_size,
            Self::ROW_2 + half_size,
            Shape::arrow(Some(90)),
        );
        // Right Arrow
        self.draw_arrow(
            Self::COL_3 + half_size,
            Self::ROW_2 + half_size,
            Shape::arrow(Some(0)),
        );
    }

    fn draw_container_key(&mut self, x: usize, y: usize, locked: bool) {
        let color = if locked {
            rgb(25, 25, 25)
        } else {
            rgb(55, 55, 55)
        };

        self.gui
            .draw_rect(x, y, Self::KEY_SIZE, Self::KEY_SIZE, Some(color));
    }

    fn draw_arrow(&mut self, x: usize, y: usize, shape: Shape) {
        self.gui.draw_shape(x, y, shape, Some(rgb(255, 255, 255)));
    }
}

impl GUI {
    fn new(width: usize, height: usize) -> Self {
        let buffer: Vec<u32> = vec![0; width * height];
        let mut window: Window = Window::new(
            "MouseSteering by RasyaJusticio",
            width,
            height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.topmost(true);
        window.set_target_fps(60);

        GUI {
            window,
            buffer,
            width,
            height,
        }
    }

    fn should_close(&mut self) -> bool {
        !self.window.is_open()
    }

    fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
        self.clear_buffer();
    }

    fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color_: Option<u32>) {
        let color: u32 = color_.unwrap_or(rgb(255, 255, 255));

        for dy in 0..height {
            for dx in 0..width {
                self.set_pixel(x + dx, y + dy, color);
            }
        }
    }

    fn draw_shape(&mut self, x: usize, y: usize, shape: Shape, color_: Option<u32>) {
        let color: u32 = color_.unwrap_or(rgb(255, 255, 255));

        for &(dx, dy) in &shape.shape {
            self.set_pixel(
                x.wrapping_add(dx as usize),
                y.wrapping_add(dy as usize),
                color,
            );
        }
    }

    fn clear_buffer(&mut self) {
        self.buffer.iter_mut().for_each(|p| *p = 0);
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        let index = self.to_index(x, y);
        self.buffer[index] = value;
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl Shape {
    pub fn arrow(angle: Option<i32>) -> Shape {
        let mut shape: Vec<(i8, i8)> = Vec::new();

        for dy in 0..8 {
            for dx in 0..3 {
                shape.push((dx - dy + 3, -dy));
            }
        }
        for dy in 1..8 {
            for dx in 0..3 {
                shape.push((dx - dy + 3, dy));
            }
        }

        if !angle.is_none() {
            shape = Shape::rotate(&shape, angle.unwrap());
        }

        Shape { shape }
    }

    fn rotate(shape: &Vec<(i8, i8)>, angle: i32) -> Vec<(i8, i8)> {
        let mut rotated_shape: Vec<(i8, i8)> = Vec::new();
        let angle_rad = angle as f64 * std::f64::consts::PI / 180.0;

        for &(x, y) in shape.iter() {
            let x_new = (x as f64 * angle_rad.cos() - y as f64 * angle_rad.sin()).round() as i8;
            let y_new = (x as f64 * angle_rad.sin() + y as f64 * angle_rad.cos()).round() as i8;
            rotated_shape.push((x_new, y_new));
        }

        rotated_shape
    }
}
