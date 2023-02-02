fn get_products_for_direction(grid: &Vec<Vec<u64>>, direction: (i32, i32)) -> Vec<u64> {
    let mut products = Vec::new();

    let x_start: usize = (-direction.0 * 4).max(0) as usize;
    let x_end: usize = ((grid.len() as i32 - direction.0*4) as usize).min(grid.len());

    for x in x_start..x_end {

        let y_start: usize = (-direction.1 * 4).max(0) as usize;
        let y_end: usize = ((grid[x].len() as i32 - direction.1*4) as usize).min(grid[x].len());
        for y in y_start..y_end {
            let product = (0..4)
            .map(|n| grid[(x as i32 + direction.0*n) as usize][(y as i32+ direction.1*n) as usize])
            .reduce(|acc, ele| acc * ele)
            .unwrap();
            products.push(product);
        }
    }

    products
}

pub fn solve(grid: Vec<Vec<u64>>) -> u64 {
    let directions = vec![(1, 0), (1, 1), (0, 1), (-1, 1)];

    let all_products = directions
    .iter()
    .fold(Vec::new(), |acc, dir| {
        let mut vec = Vec::new();
        vec.extend(acc);
        vec.extend(get_products_for_direction(&grid, *dir));
        return vec;
    });

    *all_products
    .iter()
    .max()
    .unwrap()
}