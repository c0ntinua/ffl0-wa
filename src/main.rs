mod curses; use curses::*;
mod filter; use filter::*;
mod state; use state::*;
use std::env;    

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut default_settings: Vec<usize> =vec![50,100,7,5,20,3000];
    for i in 0..default_settings.len() {
        if i < args.len() - 1 {
            let parsed = match args[i+1].parse::<usize>() {
                Ok(u) => u,
                Err(_) => 0,
            };
            if parsed > 0 && i < default_settings.len() {
                default_settings[i] = parsed;
            }
        }
    }
    let rows = default_settings[0];
    let cols = default_settings[1];
    let filters = default_settings[2];
    let span = default_settings[3] as i32;
    let flux = default_settings[4];
    let updates = default_settings[5];
    let mut reps = 3000;
    clear_screen();
    hide_cursor();
    let mut filter_system = simple_random_filters(filters,span,span,2.0);
    let mut state = random_state(rows, cols);
    cursor_to(1,1);
    print!("fflo {} {} {} {} {} {}", rows, cols, filters, span, flux, updates);
    loop {
        for _ in 0..reps {
            let row =  rand::random::<usize>() % rows;
            let col =  rand::random::<usize>() % cols;
            for f in &filter_system {
                filter_state_mutate_cell(&f, &mut state, row, col, rows, cols);
            }

        }

 
        display(&state, rows, cols);
        if rand::random::<usize>()%1000 < flux {
            filter_system = simple_random_filters(filters,span,span,0.2 + rand::random::<f64>());
        }
        // if rand::random::<usize>()%2000 < flux {
        //     state = random_state(rows, cols);
        // }
    }
}
