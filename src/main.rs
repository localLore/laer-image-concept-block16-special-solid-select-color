use laer_image_concept::image::Image;
use laer_image_concept::color::Color;
use laer_image_concept_cube141::CubeImage141;
use std::collections::HashMap;

fn pos(x: usize, y: usize, z: usize) -> (usize, usize) {
    (90 - 6 * x + 6 * y + 0 * z, 120 + 2 * x + 2 * y - 8 * z)
}

pub struct Runtime {
    pub assets: HashMap<String, CubeImage141>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            assets: HashMap::new()
        }
    }

    pub fn load(&mut self, path: &str) {
        let target = CubeImage141::open(path).unwrap();
        self.assets.insert(path.to_string(), target);
    }

    pub fn show(&self, path: &str) {
        let _ = self.assets.get(path).unwrap().show();
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let to_path = |name: &str| format!("assets/{}.png", name);
    runtime.load(&to_path("red"));
    runtime.load(&to_path("green"));
    runtime.load(&to_path("blue"));

    let mut target = Image::new(191, 191, Color::default());

    let mut frame: [[[Option<String>; 16]; 16]; 16] = Default::default();
    frame[1][0][0] = Some(to_path("red"));
    frame[0][2][0] = Some(to_path("green"));
    frame[0][0][3] = Some(to_path("blue"));

    for z in 0..16 {
        for y in 0..16 {
            for x in 0..16 {
                if let Some(path) = &frame[z][y][x] {
                    if let Some(atom) = runtime.assets.get(path.as_str()) {
                        let (pos_x, pos_y) = pos(x, y, z);
                        let _ = target.mix(&atom.to_image(), pos_x, pos_y);
                    }
                }
            }
        }
    }

    let _ = target.save("output.png");
    let _ = target.show();
}
