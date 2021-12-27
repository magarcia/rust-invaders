use crossterm::terminal;

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let (cols, rows) = terminal::size().unwrap();
    let mut columns = Vec::with_capacity(cols as usize);
    for _ in 0..cols.into() {
        let mut row = Vec::with_capacity(rows as usize);
        for _ in 0..rows.into() {
            row.push(" ")
        }
        columns.push(row)
    }
    columns
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
