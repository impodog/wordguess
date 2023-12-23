use crate::data::*;
use crate::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub y: usize,
    pub x: usize,
}

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
    pub text: Text2dBundle,
}

impl TileBundle {
    fn calc_pos(flip: bool, y: usize, x: usize, yl: usize, xl: usize, size: usize) -> Vec2 {
        if flip {
            let matrix_width = size * yl;
            let matrix_height = size * xl;
            Vec2::new(
                -(matrix_width as f32 / 2.0) + (size * y) as f32 - (size / 2) as f32,
                matrix_height as f32 / 2.0 - (size * x) as f32 - (size / 2) as f32,
            )
        } else {
            let matrix_width = size * xl;
            let matrix_height = size * yl;
            Vec2::new(
                -(matrix_width as f32 / 2.0) + (size * x) as f32 - (size / 2) as f32,
                matrix_height as f32 / 2.0 - (size * y) as f32 - (size / 2) as f32,
            )
        }
    }
    pub fn new(
        flip: bool,
        y: usize,
        x: usize,
        yl: usize,
        xl: usize,
        size: usize,
        cascadia: &Res<Cascadia>,
    ) -> TileBundle {
        let pos = Self::calc_pos(flip, y, x, yl, xl, size);
        debug_println!("pos: {:?}", pos);
        let transform = Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0));
        TileBundle {
            tile: Tile { y, x },
            text: Text2dBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: cascadia.font.clone(),
                        font_size: 50.0,
                        color: Color::BLACK,
                    },
                ),
                transform,
                ..Default::default()
            },
        }
    }
}
