use crate::y2015::io::get_contents;

fn parse_dimensions(dimensions: &str) -> Option<[u32; 3]> {
    let mut parts = dimensions.split('x');

    let length = parts.next()?.parse().ok()?;
    let width = parts.next()?.parse().ok()?;
    let height = parts.next()?.parse().ok()?;

    Some([length, width, height])
}

pub fn one_star() {
    let contents = get_contents("src/y2015/inputs/d02.txt");

    let dimensions_list = contents.lines();

    let mut total_wrapping_paper: u32 = 0;

    for dimensions in dimensions_list {
        let Some([length, width, height]) = parse_dimensions(dimensions) else {
            continue;
        };

        let surface_areas = [length * width, width * height, height * length];

        let total_area: u32 = surface_areas.iter().sum();
        let minimum_area: u32 = surface_areas.into_iter().min().unwrap();

        total_wrapping_paper += 2 * total_area + minimum_area;
    }

    println!("Total amount of wrapping paper needed: {total_wrapping_paper}");
}

pub fn two_star() {
    let contents = get_contents("src/y2015/inputs/d02.txt");

    let dimensions_list = contents.lines();

    let mut total_ribbon_length: u32 = 0;

    for dimensions in dimensions_list {
        let Some([length, width, height]) = parse_dimensions(dimensions) else {
            continue;
        };

        let mut sides = [length, width, height];

        // Sorting the array allows us to grab the smallest two sides using indices
        sides.sort();

        let volume = length * width * height;
        let smallest_perimeter = 2 * sides[0] + 2 * sides[1];

        total_ribbon_length += volume + smallest_perimeter;
    }

    println!("Total length of ribbon needed: {total_ribbon_length}");
}
