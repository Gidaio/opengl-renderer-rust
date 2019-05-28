use std::fs::File;
use std::io::{BufRead, BufReader};

enum LineType {
    Vertex,
    Normal,
    Texture,
    Face,
    Comment,
    Unknown
}

#[derive(Debug)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl Vertex {
    fn from_numbers(numbers: Vec<f32>) -> Vertex {
        if numbers.len() < 3 {
            panic!("Not enough numbers to make a vertex!")
        }

        Vertex {
            x: numbers[0],
            y: numbers[1],
            z: numbers[2],
            w: if numbers.len() > 3 { numbers[3] } else { 1.0 }
        }
    }
}

#[derive(Debug)]
struct Normal {
    x: f32,
    y: f32,
    z: f32
}

impl Normal {
    fn from_numbers(numbers: Vec<f32>) -> Normal {
        if numbers.len() < 3 {
            panic!("Not enough numbers to make a normal!")
        }

        Normal {
            x: numbers[0],
            y: numbers[1],
            z: numbers[2]
        }
    }
}

#[derive(Debug)]
struct Texture {
    x: f32,
    y: f32
}

impl Texture {
    fn from_numbers(numbers: Vec<f32>) -> Texture {
        if numbers.len() < 2 {
            panic!("Not enough numbers to make a texture coordinate!")
        }

        Texture {
            x: numbers[0],
            y: numbers[1]
        }
    }
}

#[derive(Debug)]
struct FacePoint {
    vertex_index: usize,
    texture_index: usize,
    normal_index: usize
}

#[derive(Debug)]
struct Face {
    points: Vec<FacePoint>
}


pub fn load_model(filename: &'static str) -> Vec<f32> {
    println!("Reading file {}...", filename);
    let file = File::open(filename).expect("Couldn't open the file!");

    let buffer = BufReader::new(file);
    let mut vertices: Vec<Vertex> = vec![];
    let mut textures: Vec<Texture> = vec![];
    let mut normals: Vec<Normal> = vec![];
    let mut faces: Vec<Face> = vec![];

    'lines: for line in buffer.lines() {
        let good_line = line.expect("Couldn't read the line!");
        let mut tokens = good_line.split_whitespace();

        let first_token = tokens.next();

        let line_type = match first_token {
            Some(token) => {
                if token == "v" {
                    LineType::Vertex
                } else if token == "vt" {
                    LineType::Texture
                } else if token == "vn" {
                    LineType::Normal
                } else if token == "f" {
                    LineType::Face
                } else if token == "#" {
                    LineType::Comment
                } else {
                    LineType::Unknown
                }
            }
            None => { continue 'lines }
        };

        match line_type {
            LineType::Vertex => {
                vertices.push(Vertex::from_numbers(parse_numbers(tokens)));
            }
            LineType::Normal => {
                normals.push(Normal::from_numbers(parse_numbers(tokens)));
            }
            LineType::Texture => {
                textures.push(Texture::from_numbers(parse_numbers(tokens)));
            }

            LineType::Face => {
                let mut face_points: Vec<FacePoint> = vec![];

                for token in tokens {
                    let indices: Vec<usize> = token.split("/").map(|index| {
                        if index == "" {
                            0
                        } else {
                            index.parse::<usize>().unwrap()
                        }
                    }).collect();

                    if indices.len() < 1 {
                        panic!("Malformed face tag!");
                    }

                    face_points.push(FacePoint {
                        vertex_index: indices[0],
                        texture_index: if indices.len() > 1 { indices[1] } else { 0 },
                        normal_index: if indices.len() > 2 { indices[2] } else { 0 }
                    });
                }

                faces.push(Face { points: face_points });
            }

            LineType::Unknown => {
                println!("Unknown line type: {}", good_line);
            }
            LineType::Comment => ()
        }
    }

    println!("Parsing file...");
    let mut out: Vec<f32> = vec![];

    for face in faces {
        for point in face.points {
            if vertices.len() < point.vertex_index {
                panic!("Out of bounds vertex index specified!");
            }

            out.push(vertices[point.vertex_index - 1].x);
            out.push(vertices[point.vertex_index - 1].y);
            out.push(vertices[point.vertex_index - 1].z);

            if point.normal_index > 0 {
                if normals.len() < point.normal_index {
                    panic!("Out of bounds normal index specified!");
                }

                out.push(normals[point.normal_index - 1].x);
                out.push(normals[point.normal_index - 1].y);
                out.push(normals[point.normal_index - 1].z);
            }

            if point.texture_index > 0 {
                if textures.len() < point.texture_index {
                    panic!("Out of bounds texture index specified!");
                }

                out.push(textures[point.texture_index - 1].x);
                out.push(textures[point.texture_index - 1].y);
            } else {
              out.push(0.0);
              out.push(0.0);
            }
        }
    }

    println!("Done!");

    out
}

fn parse_numbers(tokens: std::str::SplitWhitespace) -> Vec<f32> {
    let mut numbers: Vec<f32> = vec![];

    for token in tokens {
        let value: f32 = token.parse().unwrap();
        numbers.push(value);
    }

    numbers
}
