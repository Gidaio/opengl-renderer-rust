use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

enum LineType {
    Vertex,
    Normal,
    Texture,
    Face,
    Comment,
    MaterialReference,
    UseMaterial,
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

pub struct Model {
    pub vertices: Vec<f32>,
    pub materials: Vec<Material>,
    pub material_indices: Vec<i32>
}


pub fn load_model(filename: &'static str) -> Model {
    let mut filepath = PathBuf::from(filename);
    println!("Reading file {:?}...", filepath);
    let file = File::open(&filepath).expect("Couldn't open the file!");

    let buffer = BufReader::new(file);
    let mut vertices: Vec<Vertex> = vec![];
    let mut textures: Vec<Texture> = vec![];
    let mut normals: Vec<Normal> = vec![];
    let mut faces: Vec<Face> = vec![];
    let mut file_materials: HashMap<String, Material> = HashMap::new();
    let mut material_indices: Vec<i32> = vec![];
    let mut model_materials: Vec<Material> = vec![];

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
                } else if token == "mtllib" {
                    LineType::MaterialReference
                } else if token == "usemtl" {
                    LineType::UseMaterial
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
            LineType::MaterialReference => {
                filepath.set_file_name(tokens.next().unwrap());
                file_materials = load_material_file(&filepath);
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

            LineType::UseMaterial => {
                let material_name = tokens.next().unwrap();

                println!("Using material {}", material_name);

                match file_materials.remove(material_name) {
                    Some(material) => {
                        model_materials.push(material);
                    }
                    None => {
                        panic!("Couldn't find material with name {}!", material_name);
                    }
                }

                if faces.len() > 0 {
                    material_indices.push((faces.len() * 3) as i32);
                }
            }

            LineType::Unknown => {
                println!("Unknown line type: {}", good_line);
            }
            LineType::Comment => ()
        }
    }

    material_indices.push((faces.len() * 3) as i32);

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

    Model {
        vertices: out,
        materials: model_materials,
        material_indices: material_indices
    }
}


enum MaterialLineType {
    NewMaterial,
    AmbientPercentage,
    DiffuseColor,
    SpecularColor,
    EmissiveColor,
    Shininess,
    Comment,
    Unknown
}

#[derive(Debug)]
pub struct Material {
    pub ambient_percentage: Vec<f32>,
    pub diffuse_color: Vec<f32>,
    pub specular_color: Vec<f32>,
    pub emissive_color: Vec<f32>,
    pub shininess: f32
}

fn load_material_file(filepath: &PathBuf) -> HashMap<String, Material> {
    println!("Reading file {:?}...", filepath);
    let material_file = File::open(filepath).expect("Couldn't open material file!");

    let buffer = BufReader::new(material_file);
    let mut current_material_name = String::from("");
    let mut current_material = Material {
        ambient_percentage: vec![],
        diffuse_color: vec![],
        specular_color: vec![],
        emissive_color: vec![],
        shininess: 0.0
    };

    let mut materials = HashMap::new();

    'lines: for line in buffer.lines() {
        let good_line = line.expect("Couldn't read the line!");
        let mut tokens = good_line.split_whitespace();

        let first_token = tokens.next();

        let line_type = match first_token {
            Some(token) => {
                if token == "newmtl" {
                    MaterialLineType::NewMaterial
                } else if token == "Ka" {
                    MaterialLineType::AmbientPercentage
                } else if token == "Kd" {
                    MaterialLineType::DiffuseColor
                } else if token == "Ks" {
                    MaterialLineType::SpecularColor
                } else if token == "Ke" {
                    MaterialLineType::EmissiveColor
                } else if token == "Ns" {
                    MaterialLineType::Shininess
                } else if token == "#" {
                    MaterialLineType::Comment
                } else {
                    MaterialLineType::Unknown
                }
            }
            None => { continue 'lines }
        };

        match line_type {
            MaterialLineType::NewMaterial => {
                println!("Found a new material!");

                if current_material_name != "" {
                    println!("Adding previous material to hashmap.");
                    materials.insert(current_material_name, current_material);
                }

                current_material_name = String::from(tokens.next().unwrap());
                println!("Material name is \"{}\"", current_material_name);
                current_material = Material {
                    ambient_percentage: vec![],
                    diffuse_color: vec![],
                    specular_color: vec![],
                    emissive_color: vec![],
                    shininess: 0.0
                };
            }
            MaterialLineType::AmbientPercentage => {
                println!("Found an ambient percentage!");
                current_material.ambient_percentage = parse_numbers(tokens);
                println!("Ambient percentage was {:?}", current_material.ambient_percentage);
            }
            MaterialLineType::DiffuseColor => {
                println!("Found a diffuse color!");
                current_material.diffuse_color = parse_numbers(tokens);
                println!("Diffuse collor was {:?}", current_material.diffuse_color);
            }
            MaterialLineType::SpecularColor => {
                println!("Found a specular color!");
                current_material.specular_color = parse_numbers(tokens);
                println!("Specular color was {:?}", current_material.specular_color);
            }
            MaterialLineType::EmissiveColor => {
                println!("Found an emissive color!");
                current_material.emissive_color = parse_numbers(tokens);
                println!("Emissive color was {:?}", current_material.emissive_color);
            }
            MaterialLineType::Shininess => {
                println!("Found a shininess!");
                current_material.shininess = parse_numbers(tokens)[0];
                println!("Shininess was {}", current_material.shininess);
            }

            MaterialLineType::Unknown => {
                println!("Unknown line type: {}", good_line);
            }
            MaterialLineType::Comment => ()
        }
    }

    materials.insert(current_material_name, current_material);

    println!("Materials: {:?}", materials);

    materials
}


fn parse_numbers(tokens: std::str::SplitWhitespace) -> Vec<f32> {
    let mut numbers: Vec<f32> = vec![];

    for token in tokens {
        let value: f32 = token.parse().unwrap();
        numbers.push(value);
    }

    numbers
}
