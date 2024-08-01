use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod framebuffer;
use framebuffer::Framebuffer;

fn count_neighbors(framebuffer: &Framebuffer, x: usize, y: usize) -> u8 {
    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), /*(0, 0),*/ (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    let mut count = 0;
    for (dx, dy) in directions.iter() {
        // Convert x and y to isize for negative index arithmetic
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        // Check if the new coordinates are within bounds
        if nx >= 0 && ny >= 0 && nx < framebuffer.width as isize && ny < framebuffer.height as isize {
            let nx = nx as usize;
            let ny = ny as usize;

            if framebuffer.buffer[ny * framebuffer.width + nx] == 0xFFFFFF {
                count += 1;
            }
        }
    }

    count
}

fn update_framebuffer(current: &Framebuffer, next: &mut Framebuffer) {
    for y in 0..current.height {
        for x in 0..current.width {
            let neighbors = count_neighbors(current, x, y);
            let index = y * current.width + x;

            if current.buffer[index] == 0xFFFFFF { // Celda viva
                if neighbors < 2 || neighbors > 3 {
                    next.buffer[index] = 0x000000; // Muere
                } else {
                    next.buffer[index] = 0xFFFFFF; // Sobrevive
                }
            } else { // Celda muerta
                if neighbors == 3 {
                    next.buffer[index] = 0xFFFFFF; // Nace
                } else {
                    next.buffer[index] = 0x000000; // Permanece muerta
                }
            }
        }
    }
}

use rand::{thread_rng, Rng}; 

fn set_initial_pattern(framebuffer: &mut Framebuffer) {
    let glider = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]; // Glider
    let lwss = vec![(1, 0), (4, 0), (0, 1), (0, 2), (4, 2), (0, 3), (1, 3), (2, 3), (3, 3)]; // Lightweight spaceship
    let pulsar = vec![
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2), (2, 3), (5, 3), (7, 3), (10, 3),
        // Agrega más puntos según sea necesario...
    ];
    let toad = vec![(0, 0), (1, 0), (2, 0), (1, 1), (2, 1), (3, 1)]; // Toad
    let beacon = vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)]; // Beacon
    let block = vec![(0, 0), (1, 0), (0, 1), (1, 1)]; // Block

    // Coloca estos patrones en posiciones aleatorias y ampliadas en el tablero
    let patterns = vec![&glider, &lwss, &pulsar, &toad, &beacon, &block];
    let mut rng = thread_rng();

    for pattern in patterns {
        // Genera coordenadas aleatorias para colocar patrones
        let x_offset = rng.gen_range(0..framebuffer.width - 20); // Asegura espacio para la expansión del patrón
        let y_offset = rng.gen_range(0..framebuffer.height - 20);

        // Escala los patrones para cubrir más espacio
        for &(x, y) in pattern {
            for dx in 0..3 {  // Multiplica cada celda por un factor para aumentar el tamaño del patrón
                for dy in 0..3 {
                    let x_pos = x * 3 + dx + x_offset;
                    let y_pos = y * 3 + dy + y_offset;
                    if x_pos < framebuffer.width && y_pos < framebuffer.height {
                        framebuffer.buffer[y_pos * framebuffer.width + x_pos] = 0xFFFFFF;
                    }
                }
            }
        }
    }
}




fn render(framebuffer: &mut Framebuffer) {
    // This is now managed in update_framebuffer
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let frame_delay = Duration::from_millis(100);

    let mut current_framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut next_framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    // Set the initial pattern
    set_initial_pattern(&mut current_framebuffer);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Update the framebuffer based on the game of life rules
        update_framebuffer(&current_framebuffer, &mut next_framebuffer);

        // Swap framebuffers
        std::mem::swap(&mut current_framebuffer, &mut next_framebuffer);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&current_framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
