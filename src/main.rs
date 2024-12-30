use crossterm::{cursor, event::{self, Event, KeyCode}, terminal::{self, ClearType}, ExecutableCommand};
use std::io::{stdout, Error, Stdout};
use std::time::Duration;
use crossterm::style::Print;

fn main() -> Result<(), Error> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::Clear(ClearType::All))?;

    stdout.execute(cursor::Hide)?;
    draw_borders(&mut stdout)?;
    let (term_size_x, term_size_y) = terminal::size()?;

    let mut rectangle = Rectangle::new(1, 1, "███", "   ");
    loop {
        rectangle.draw_and_reset_position_if_needed(&mut stdout)?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => break,
                    KeyCode::Char('Q') => break,
                    KeyCode::Char('q') => break,
                    KeyCode::Up => rectangle.move_up(),
                    KeyCode::Down => rectangle.move_down(term_size_y),
                    KeyCode::Left => rectangle.move_left(),
                    KeyCode::Right => rectangle.move_right(term_size_x),
                    _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

fn draw_borders(stdout: &mut Stdout) -> Result<(), Error> {
    let (term_size_x, term_size_y) = terminal::size()?;

    for i in 0..term_size_x {
        stdout.execute(cursor::MoveTo(i, 0))?;
        print!("█");
        stdout.execute(cursor::MoveTo(i, term_size_y - 1))?;
        print!("█");
    }

    for i in 0..term_size_y {
        stdout.execute(cursor::MoveTo(0, i))?;
        print!("█");
        stdout.execute(cursor::MoveTo(term_size_x - 1, i))?;
        print!("█");
    }

    Ok(())
}

struct Rectangle {
    x: u16,
    y: u16,

    body: String,
    placeholder: String,

    previous_x: u16,
    previous_y: u16,
}

impl Rectangle {
    fn new(x: u16, y: u16, body: &str, placeholder: &str) -> Self {
        Self {
            x,
            y,
            body: body.to_string(),
            placeholder: placeholder.to_string(),
            previous_x: 0,
            previous_y: 0,
        }
    }

    fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn move_down(&mut self, term_size_y: u16) {
        if self.y < term_size_y - 1 {
            self.y += 1;
        }
    }

    fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn move_right(&mut self, term_size_x: u16) {
        if self.x < term_size_x - 1 {
            self.x += 1;
        }
    }

    fn draw_and_reset_position_if_needed(&mut self, stdout: &mut Stdout) -> Result<(), Error> {
        if self.x == self.previous_x && self.y == self.previous_y {
            return Ok(());
        }

        let placeholder: &str = self.placeholder.as_ref();
        let body: &str = self.body.as_ref();
        stdout
            .execute(cursor::MoveTo(self.previous_x, self.previous_y))?
            .execute(Print(placeholder))?;

        stdout
            .execute(cursor::MoveTo(self.x, self.y))?
            .execute(Print(body))?;

        self.previous_x = self.x;
        self.previous_y = self.y;

        Ok(())
    }
}
