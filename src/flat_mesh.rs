use bevy::{prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_asset::RenderAssetUsages}};

pub fn gen_flat_mesh(left: i32, top: i32, width: i32, height: i32) -> Mesh{

    let vertices_count = (width * height * 4) as usize;
    let indices_count = (width * height * 6) as usize;
    let mut positions = std::vec![Vec3::ZERO; vertices_count];
    let mut uvs = std::vec![Vec2::ZERO; vertices_count];
    let normals = std::vec![Vec3::Y; vertices_count];

    let mut indices: Vec<u32> = std::vec![0; indices_count];

    for h in 0..(height){
        for w in 0..(width){
            let position_first = ((h * width + w) * 4) as usize;
            let w: i32 = w as i32;
            let h: i32 = h as i32;
            
            positions[position_first] = Vec3 {x: (left + w) as f32, y: 0., z: (top + h) as f32};
            positions[position_first + 1] = Vec3 {x: (left + w + 1) as f32, y: 0., z: (top + h) as f32};
            positions[position_first + 2] = Vec3 {x: (left + w) as f32, y: 0., z: (top + h + 1) as f32};
            positions[position_first + 3] = Vec3 {x: (left + w + 1) as f32, y: 0., z: (top + h + 1) as f32};


            uvs[position_first] = Vec2 {x: 0.0, y: 0.0};
            uvs[position_first + 1] = Vec2 {x: 1.0, y: 0.0};
            uvs[position_first + 2] = Vec2 {x: 0.0, y: 1.0};
            uvs[position_first + 3] = Vec2 {x: 1.0, y: 1.0};


            let indice_first = ((h * (width as i32) + w) * 6) as usize;
            let position_first: u32 = position_first as u32;

            indices[indice_first] = position_first;
            indices[indice_first + 1] = position_first + 2;
            indices[indice_first + 2] = position_first + 1;

            indices[indice_first + 3] = position_first + 1;
            indices[indice_first + 4] = position_first + 2;
            indices[indice_first + 5] = position_first + 3;
        }
    }


    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        // Add 4 vertices, each with its own position attribute (coordinate in
        // 3D space), for each of the corners of the parallelogram.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            positions
        )
        // Assign a UV coordinate to each vertex.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            uvs
        )
        // Assign normals (everything points outwards)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals
        )
        // After defining all the vertices and their attributes, build each triangle using the
        // indices of the vertices that make it up in a counter-clockwise order.
        .with_inserted_indices(Indices::U32(indices))
}
