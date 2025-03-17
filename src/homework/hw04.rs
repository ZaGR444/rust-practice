const HEIGHT: usize = 11; 
const WIDTH: usize = 11;  

fn main() {
    let mut rhombus = vec![vec![' '; WIDTH]; HEIGHT];

    let mid = WIDTH / 2; 

    for i in 0..=mid {
        for j in (mid - i)..=(mid + i) {
            rhombus[i][j] = '*';
        }
    }

    for i in (mid + 1)..HEIGHT {
        let mirror_i = HEIGHT - 1 - i;
        for j in (mid - mirror_i)..=(mid + mirror_i) {
            rhombus[i][j] = '*';
        }
    }

    print!("{}", rhombus.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
    println!();
}
