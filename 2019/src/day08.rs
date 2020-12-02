use std::fs;
use std::char;

#[derive(Debug, PartialEq, Eq)]
struct Pixel {
    color: u8
}

impl Pixel {
    fn parse(to_parse: char) -> Pixel {
        Pixel {
            color: to_parse.to_digit(10).map(|x| x as u8).unwrap()
        }
    }

    fn merge(&self, other: &Pixel) -> Pixel {
        if self.color == 2 {
            return Pixel { color: other.color }
        } else {
            return Pixel { color: self.color }
        }
    }

    fn to_char(&self) -> char {
        if self.color == 0 {
            return ' ';
        }

        char::from_digit(self.color as u32, 10).unwrap()
    }
}

#[derive(Debug)]
struct Row {
    pixels: Vec<Pixel>,
    width: usize
}

impl Row {
    fn parse(to_parse: &str, width: usize) -> Row {
        Row {
            pixels: to_parse.chars().map(|c| Pixel::parse(c)).collect(),
            width: width
        }
    }

    fn count_pixel(&self, to_check: &Pixel) -> u32 {
        self.pixels.iter().fold(0, |count, pixel| {
            if pixel == to_check {
                return count + 1;
            } else {
                return count
            }
        })
    }

    fn merge(&self, other: &Row) -> Row {
        Row {
            pixels: self.pixels.iter().zip(&other.pixels).map(|(p1, p2)| p1.merge(&p2)).collect(),
            width: self.width
        }
    }

    fn to_string(&self) -> String {
        self.pixels.iter().map(|p| p.to_char()).collect()
    }
}

#[derive(Debug)]
struct Layer {
    rows: Vec<Row>,
    dimension: (usize, usize)
}

impl Layer {
    fn parse(to_parse: &str, (width, height): (usize, usize)) -> Layer {
        let mut res = vec![];
        for i in 0..height {
            res.push(Row::parse(&to_parse[i*width..(i+1)*width], width));
        }

        Layer {
            rows: res,
            dimension: (width, height)
        }
    }

    fn count_pixel(&self, to_check: &Pixel) -> u32 {
        self.rows.iter().map(|row| row.count_pixel(to_check)).sum()
    }

    fn merge(&self, other: &Layer) -> Layer {
        Layer {
            rows: self.rows.iter().zip(&other.rows).map(|(r1, r2)| r1.merge(&r2)).collect(),
            dimension: self.dimension
        }
    }

    fn to_string(&self) -> String {
        let strings: Vec<String> = self.rows.iter().map(|row| row.to_string()).collect();
        return strings.join("\n");
    }
}

#[derive(Debug)]
struct Image {
    layers: Vec<Layer>,
    dimension: (usize, usize)
}

impl Image {
    fn parse(to_parse: &str, (width, height): (usize, usize)) -> Image {
        let total_size = width * height;
        let mut res = vec![];
        for i in 0..(to_parse.len() / total_size) {
            res.push(Layer::parse(&to_parse[i*total_size..(i+1)*total_size], (width, height)))
        }

        Image {
            layers: res,
            dimension: (width, height)
        }
    }

    fn merge(&self) -> MergedImage {
        let mut merged = self.layers[0].merge(&self.layers[1]);
        for layer in &self.layers[2..] {
            merged = merged.merge(layer);
        };

        MergedImage {
            layer: merged,
            dimension: self.dimension
        }
    }
}

#[derive(Debug)]
struct MergedImage {
    layer: Layer,
    dimension: (usize, usize)
}

impl MergedImage {
    fn to_string(&self) -> String {
        self.layer.to_string()
    }
}

pub fn main() {
    let file_contents = fs::read_to_string("puzzles/08.txt").unwrap();
    let image = Image::parse(&file_contents, (25, 6));

    let mut max_value = u32::max_value();
    let mut max_layer: Option<usize> = None;
    for (i, layer) in image.layers.iter().enumerate() {
        let count = layer.count_pixel(&Pixel {color: 0});
        if count < max_value {
            max_value = count;
            max_layer = Some(i);
        }
    }

    let max_layer = &image.layers[max_layer.unwrap()];
    println!("Solution to first exercise: {}", max_layer.count_pixel(&Pixel {color: 2}) * max_layer.count_pixel(&Pixel {color: 1}));

    println!("Solution to the second exercise: \n{}", image.merge().to_string());
}