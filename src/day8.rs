fn get_layers(input: &[u8], width: usize, height: usize) -> Vec<Vec<u8>> {
    let size = width * height;
    let mut layers = Vec::new();
    for layer in input.chunks(size) {
        layers.push(layer.to_owned());
    }
    layers
}
fn print_layers(layers: &[Vec<u8>], width: usize, height: usize) {
    let mut image = vec![2u8; width * height];
    let mut head = 0;
    while head < image.len() {
        for layer in layers {
            if layer[head] != 2 {
                image[head] = layer[head];
                break;
            }
        }
        head += 1;
    }
    for row in image.chunks(width) {
        println!("{:?}", row);
    }
}
#[cfg(test)]
mod tests {
    use super::get_layers;
    use super::print_layers;
    use std::fs;
    #[test]
    fn example_day8_part2() {
        let input = vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0];
        let layers = get_layers(&input, 2, 2);
        print_layers(&layers, 2, 2);
    }
    #[test]
    fn day_8_part1_and_2() {
        let input: Vec<u8> = fs::read_to_string("input/day8")
            .unwrap()
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let layers = get_layers(&input, 25, 6);
        let min_zeroes = layers
            .iter()
            .min_by_key(|layer| {
                layer
                    .iter()
                    .map(|&n| if n == 0 { 1u32 } else { 0u32 })
                    .sum::<u32>()
            })
            .unwrap();
        use std::collections::HashMap;
        let mut count: HashMap<u8, u32> = HashMap::new();
        for &digit in min_zeroes {
            *count.entry(digit).or_default() += 1;
        }
        assert_eq!(count[&1], 16);
        assert_eq!(count[&2], 128);

        print_layers(&layers, 25, 6);
        // HFYAK

    }
}
