use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(Clone, Default, Debug)]
pub struct Brick {
    pub min: [f32; 3],
    pub max: [f32; 3],
    pub color_idx: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex([f32; 3]);

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(a, b)| a.to_bits() == b.to_bits())
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for &f in &self.0 {
            state.write_u32(f.to_bits());
        }
    }
}

fn get_vertex_index(
    vertex: [f32; 3],
    vertices: &mut Vec<[f32; 3]>,
    vertex_map: &mut HashMap<Vertex, usize>,
) -> usize {
    if let Some(&idx) = vertex_map.get(&Vertex(vertex)) {
        idx
    } else {
        let idx = vertices.len();
        vertices.push(vertex);
        vertex_map.insert(Vertex(vertex), idx);
        idx
    }
}

pub fn process_vertices_faces(
    vertices: &mut Vec<[f32; 3]>,
    vertex_map: &mut HashMap<Vertex, usize>,
    faces: &mut Vec<[usize; 4]>,
    min: [f32; 3],
    max: [f32; 3],
) {
    let new_vertices = [
        [min[0], min[1], min[2]],
        [max[0], min[1], min[2]],
        [max[0], max[1], min[2]],
        [min[0], max[1], min[2]],
        [min[0], min[1], max[2]],
        [max[0], min[1], max[2]],
        [max[0], max[1], max[2]],
        [min[0], max[1], max[2]],
    ];

    for v in new_vertices {
        if !vertex_map.contains_key(&Vertex(v)) {
            vertices.push(v);
        }
    }

    let indices: Vec<usize> = new_vertices
        .iter()
        .map(|&v| get_vertex_index(v, vertices, vertex_map))
        .collect();

    let new_faces = [
        [indices[0], indices[1], indices[2], indices[3]],
        [indices[4], indices[5], indices[6], indices[7]],
        [indices[0], indices[1], indices[5], indices[4]],
        [indices[3], indices[2], indices[6], indices[7]],
        [indices[1], indices[2], indices[6], indices[5]],
        [indices[0], indices[3], indices[7], indices[4]],
    ];

    faces.extend_from_slice(&new_faces);
}

pub fn index(x: usize, y: usize, z: usize, n: usize) -> usize {
    x + y * n + z * n * n
}

pub fn help() {
    println!("Usage: rust_obj-to-voxels-colorized [input_file_path] [size(for size x size x size voxel_grid)] [output_file_path] --max-merge-length(opt) [max_merge_length(opt)] --mtl-file-path(opt) [mtl_file_path(opt)]

    - input_file_path, output_file_path - input and output obj file paths
    - size - the size in voxels of the resulting grid dimensions (voxel grid will be size × size × size)
    - mtl_file_path - mtl file path for color aware conversion
    - max_merge_length - max amount (1 by default) of voxels to horizontal one-dimensional brick merging (bricks will be either 1 × 1 × <=max_merge_length or 1 × <=max_merge_length × 1)");
}
