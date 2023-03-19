use std::{thread, time};
use crossterm::{
    cursor::{MoveLeft, MoveRight, MoveTo, MoveDown},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
    execute,
    Result,
};

const WIDTH: u16 = 60;
const HEIGHT: u16 = 20;
const PADDLE_SIZE: u16 = 5;

struct Ball {
    x: u16,
    y: u16,
    dx: i32,
    dy: i32,
}

struct Paddle {
    x: u16,
    y: u16,
}

fn main() -> Result<()> {
    // Initialize the terminal
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    execute!(std::io::stdout(), MoveTo(0, 0))?;

    // Initialize the ball and paddles
    let mut ball = Ball { x: WIDTH / 2, y: HEIGHT / 2, dx: 1, dy: 1 };
    let mut left_paddle = Paddle { x: 1, y: HEIGHT / 2 };
    let mut right_paddle = Paddle { x: WIDTH - 2, y: HEIGHT / 2 };

    enable_raw_mode().unwrap();
    
    loop {
        // Move the ball
        ball.x = (ball.x as i32 + ball.dx) as u16;
        ball.y = (ball.y as i32 + ball.dy) as u16;

        // Check for collisions with the walls and paddles
        if ball.y <= 0 || ball.y >= HEIGHT - 1 {
            ball.dy *= -1;
        }
        if ball.x <= left_paddle.x + 1 && ball.y >= left_paddle.y - PADDLE_SIZE / 2 && ball.y <= left_paddle.y + PADDLE_SIZE / 2 {
            ball.dx *= -1;
        }
        if ball.x >= right_paddle.x - 1 && ball.y >= right_paddle.y - PADDLE_SIZE / 2 && ball.y <= right_paddle.y + PADDLE_SIZE / 2 {
            ball.dx *= -1;
        }
        if ball.x <= 0 || ball.x >= WIDTH - 1 {
            break;
        }

        
        // Move the paddles
        if poll(time::Duration::from_millis(50))? {
                let event = read()?;
                match event {
                    Event::Key(KeyEvent { code: KeyCode::Char('w'), ..}) => {
                        if left_paddle.y > PADDLE_SIZE / 2 {
                            execute!(std::io::stdout(), crossterm::cursor::MoveUp(1))?;
                            left_paddle.y -= 1;
                        }
                    }
                    Event::Key(KeyEvent { code: KeyCode::Char('s'), modifiers: KeyModifiers::NONE, ..}) => {
                        if left_paddle.y < HEIGHT - PADDLE_SIZE / 2 {
                            execute!(std::io::stdout(), MoveDown(1))?;
                            left_paddle.y += 1;
                        }
                    }
                    Event::Key(KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE, ..}) => {
                        if right_paddle.y > PADDLE_SIZE / 2 {
                            execute!(std::io::stdout(), crossterm::cursor::MoveUp(1))?;
                            right_paddle.y -= 1;
                        }
                    }
                    Event::Key(KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::NONE, ..}) => {
                        if right_paddle.y < HEIGHT - PADDLE_SIZE / 2 {
                            execute!(std::io::stdout(), MoveDown(1))?;
                            right_paddle.y += 1
                        }
                    }
                    _ => {}
                }
        }
    // Clear the screen and redraw the ball and paddles
    execute!(std::io::stdout(), Clear(ClearType::All))?;
    execute!(std::io::stdout(), MoveTo(ball.x, ball.y))?;
    print!("O");
    for i in 0..PADDLE_SIZE {
        execute!(std::io::stdout(), MoveTo(left_paddle.x, left_paddle.y - PADDLE_SIZE / 2 + i))?;
        print!("|");
        execute!(std::io::stdout(), MoveTo(right_paddle.x, right_paddle.y - PADDLE_SIZE / 2 + i))?;
        print!("|");
    }

    // Sleep for a short amount of time
    thread::sleep(time::Duration::from_millis(25));
}

disable_raw_mode().unwrap();

// Game over
execute!(std::io::stdout(), Clear(ClearType::All))?;
execute!(std::io::stdout(), MoveTo(WIDTH / 2 - 4, HEIGHT / 2))?;
println!("GAME OVER");

Ok(())
}