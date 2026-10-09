use bevy::{color::palettes::tailwind::*, mesh::VertexAttributeValues, prelude::*};
use noise::{BasicMulti, NoiseFn, Perlin};

pub fn gen_mesh(a: f32, d: u32) -> Mesh {
    let mut terrain = Plane3d::default().mesh().size(a, a).subdivisions(d).build();
    
    let noise = BasicMulti::<Perlin>::new(1);
    let terrain_height = 90.0;

    if let Some(VertexAttributeValues::Float32x3(verts)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION){
        for v in verts.iter_mut() {
            let val = noise.get([v[0] as f64 / 300.0, v[2] as f64 / 300.]) as f32;


            v[1] = val * terrain_height;
        }

        let colors: Vec<[f32; 4]> = verts.iter().map(|[r, g, b]| {
            let g = *g / terrain_height * 2.0;
            if g > 0.8 {
                (Color::LinearRgba(LinearRgba::new(20., 20., 20.,1.0,))).to_linear().to_f32_array()
            } else if g > 0.3 {
                Color::from(AMBER_800).to_linear().to_f32_array()
            } else if g < -0.8{
                Color::BLACK.to_linear().to_f32_array()
            } else {
                let val = (noise.get([*r as f64 / 30.0, *b as f64 / 30.0]) + 1.0) / 4.0 + 0.5;
                (Color::from(GREEN_400).to_linear() * val as f32).to_f32_array()
            }
        }).collect();

        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    };

    terrain.compute_normals();
    terrain
}