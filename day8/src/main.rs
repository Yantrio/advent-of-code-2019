use std::fs::read_to_string;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

type Screen = Vec<Layer>;
type Layer = Vec<u32>;

fn main() {
    let layers = parse_input_to_layers().expect("Failed to parse layers");
    let least_zero_layer = layer_with_least_zeroes(&layers);
    println!(
        "Solution Part 1: {}",
        count_in_layer(&least_zero_layer, 1) * count_in_layer(&least_zero_layer, 2)
    );

    println!("Solution Part 2:");
    render_layers(&layers, WIDTH);
}

fn layer_from_bytes(st: &[u8]) -> Layer {
    st.iter()
        .map(|&c| (c as char))
        .map(|c| c.to_digit(10).expect("Failed to parse char to number"))
        .collect()
}

fn parse_input_to_layers<'a>() -> Result<Screen, &'a str> {
    let input = read_to_string("input").expect("failed to read input file");

    match (input.len() % (WIDTH * HEIGHT)) == 0 {
        false => Err("image not the correct size, expected a multiple of width*height"),
        true => Ok(input
            .as_bytes()
            .chunks(WIDTH * HEIGHT)
            .map(layer_from_bytes)
            .collect()),
    }
}

fn layer_with_least_zeroes(layers: &Screen) -> &Layer {
    let idx = layers
        .iter()
        .map(|l| count_in_layer(l, 0))
        .enumerate()
        .min_by(|&(_, a), &(_, b)| a.cmp(&b))
        .map(|(idx, _)| idx)
        .expect("Can't find max number of zeroes in a layer");
    &layers[idx]
}

fn count_in_layer(v: &Layer, to_count: u32) -> usize {
    v.iter().filter(|&&i| i == to_count).count()
}

fn calculate_colours(s: &Screen) -> Layer {
    (0..s[0].len())
        .map(|idx| {
            s.iter()
                .map(|l| l[idx])
                .filter(|&p| p != 2)
                .nth(0)
                .expect("Failed to read pixel")
        })
        .collect()
}

fn render_layers(screen: &Screen, width: usize) -> () {
    calculate_colours(screen)
        .chunks(width)
        .map(|row| {
            row.iter()
                .map(|&p| if p == 0 { "⠀" } else { "█" })
                .collect::<Vec<&str>>()
                .join("")
        })
        .for_each(|l| println!("{}", l))
}
