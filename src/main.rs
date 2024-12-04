use raylib::prelude::*;

const FRAC_1_3: f32 = 1.0 / 3.0;

pub enum TileState {
    X,
    O,
}

pub struct Board {
    pub turn: u8,
    pub bounds: Rectangle,
    /// [row][col]
    pub states: [[Option<TileState>; 3]; 3],
}

impl Board {
    pub fn new(bounds: Rectangle) -> Self {
        Self {
            turn: 0,
            bounds,
            states: Default::default(),
        }
    }

    fn mark(&mut self, row: u8, col: u8) {
        self.states[row as usize][col as usize] = Some(if self.turn & 1 == 0 { TileState::X } else { TileState::O });
        self.turn += 1;
    }

    fn bounds_iter(&self) -> impl Iterator<Item = ((u8, u8), Rectangle)> {
        let (start_x, start_y, sub_width, sub_height) = (
            self.bounds.x,
            self.bounds.y,
            self.bounds.width  * FRAC_1_3,
            self.bounds.height * FRAC_1_3
        );
        (0..9).map(move |i: u8| {
            let (row, col) = (i / 3, i % 3);
            (
                (row, col),
                Rectangle::new(
                    start_x + col as f32 * sub_width,
                    start_y + row as f32 * sub_height,
                    sub_width,
                    sub_height,
                ),
            )
        })
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = ((u8, u8), Rectangle, &'a Option<TileState>)> {
        self.bounds_iter().map(|((row, col), bounds)| (
            (row, col),
            bounds,
            &self.states[row as usize][col as usize],
        ))
    }
}

fn main() {
    let (width, height) = (720, 720);
    let (mut rl, thread) = init()
        .title("tic-tac-toe")
        .size(width, height)
        .build();

    rl.set_target_fps(60);

    let mut board = Board::new(rrect(0, 0, width, height));

    while !rl.window_should_close() {
        let mouse_pos = rl.get_mouse_position();

        let hovered_cell = board.iter()
            .find_map(|(cell, rec, state)|
                (rec.check_collision_point_rec(mouse_pos) && state.is_none())
                    .then_some(cell)
            );

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some((row, col)) = hovered_cell.as_ref().copied() {
                board.mark(row, col);
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for (_, rec, state) in board.iter() {
            d.draw_rectangle_rec(rec, match state {
                Some(TileState::X) => Color::RED,
                Some(TileState::O) => Color::BLUE,
                None => Color::GRAY,
            });
            d.draw_rectangle_lines_ex(rec, 2.0, Color::BLACK);
        }
    }
}
