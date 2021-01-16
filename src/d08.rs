use itertools::Itertools;

pub struct Image {
    layers: Vec<Vec<char>>,
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Image {
    let line = input.lines().next().unwrap();
    let layers = line
        .chars()
        .collect_vec()
        .chunks(WIDTH * HEIGHT)
        .map(|c| c.to_vec())
        .collect();
    Image { layers }
}

#[aoc(day8, part1)]
pub fn part1(image: &Image) -> usize {
    let mut min_zeros = usize::max_value();
    let mut result: usize = 0;
    for layer in &image.layers {
        let zero_ct = layer.iter().filter(|c| **c == '0').count();
        if zero_ct < min_zeros {
            let one_ct = layer.iter().filter(|c| **c == '1').count();
            let two_ct = layer.iter().filter(|c| **c == '2').count();
            min_zeros = zero_ct;
            result = one_ct * two_ct;
        }
    }
    result
}

#[aoc(day8, part2)]
pub fn part2(image: &Image) -> String {
    let mut final_image: Vec<char> = image.layers[0].clone();

    for layer in &image.layers {
        for (i, c) in layer.iter().enumerate() {
            if final_image[i] == '2' {
                final_image[i] = *c;
            }
        }
    }

    assert_eq!(0, final_image.iter().filter(|c| **c == '2').count());

    let display_image: Vec<char> = final_image
        .iter()
        .map(|c| match c {
            '0' => ' ',
            '1' => '#',
            _ => panic!("invalid"),
        })
        .collect();

    format!(
        "\n{}",
        display_image
            .chunks(WIDTH)
            .map(|chunk| chunk.iter().collect::<String>())
            .join("\n")
    )
}
