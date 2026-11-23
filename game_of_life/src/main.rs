use std::io;
use macroquad::prelude::*;
use std::{thread, time};


#[macroquad::main("Game of Life")]
async fn main() {
    const SIZE: usize = 256;

    println!("How many starting lifes do you want?");

    let mut start_lives = String::new();
    io::stdin()
        .read_line(&mut start_lives)
        .expect("msg1");

    let start:u32 = start_lives
        .trim()
        .parse()
        .expect("msg2");
    
    let mut game_board_1 = [[0; SIZE+2]; SIZE+2];
    let mut counter_alive = 0;

    while counter_alive < start {
        let rnd_x: usize = ::rand::random_range(1..=SIZE); 
        let rnd_y: usize = ::rand::random_range(1..=SIZE);

        if game_board_1 [rnd_x][rnd_y] == 0 {
            game_board_1 [rnd_x][rnd_y] = 1;
            counter_alive += 1;
        }
    }
    let mut game_board_2 = game_board_1;
    loop {
        clear_background(BLACK);

        for x in 1..=SIZE {
            for y in 1..=SIZE {
                let neighbour = 
                     game_board_1[x-1][y+1] 
                    +game_board_1[x-1][y]
                    +game_board_1[x-1][y-1]
                    +game_board_1[x+1][y]
                    +game_board_1[x][y-1]
                    +game_board_1[x+1][y+1]
                    +game_board_1[x][y+1]
                    +game_board_1[x+1][y-1];

                if game_board_1[x][y] == 0 && neighbour == 3{
                    game_board_2[x][y] = 1;
                } else if game_board_1[x][y] == 1 && (neighbour == 2 || neighbour == 3){
                    game_board_2[x][y] = 1;
                } else {
                    game_board_2[x][y] = 0;
                }
            }
        }

        let smaller = screen_width().min(screen_height());
        let cube_size = smaller/SIZE as f32;
        
        
        for x_pixle in 1..=SIZE{
            for y_pixle in 1..=SIZE{
                if game_board_1[x_pixle][y_pixle] == 1{
                    draw_rectangle((x_pixle-1) as f32 * cube_size, (y_pixle-1) as f32 * cube_size, cube_size, cube_size, WHITE)
                }
            }
        }

        game_board_1 = game_board_2;
        thread::sleep(time::Duration::from_millis(50));
        next_frame().await;

    }
}


