pub fn draw_tree(levels: u32) {
    let max_width = (levels * 2 + 1) as usize; 

    (0..levels).for_each(|level| {
        (0..=level).for_each(|row| {
            let width = (row * 2 + 1) as usize; 
            let padding = (max_width - width) / 2; 

            println!("{}{}", " ".repeat(padding), "*".repeat(width));
        });
    });
}

fn main() {
    let levels = 6; 
    draw_tree(levels);
}
