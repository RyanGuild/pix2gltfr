use gltf::json::validation::Checked::Valid;
use gltf::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{ImageData, console};
use wasm_logger::{init, Config};

#[wasm_bindgen(start)]
pub fn startup() {
    init(Config::default());
    log::info!("startup");
}

type ColorT = [u8; 4];
const VOXEL_DIMENSION: f32 = 1.0;

#[wasm_bindgen]
pub fn image_to_gltf(image: ImageData) -> String {
  console::log_1(&"image_to_gltf starting".into());
  let x_offset: f32 = -(image.width() as f32) * VOXEL_DIMENSION * 0.5;
  let y_offset: f32 = (image.height() as f32) * VOXEL_DIMENSION - VOXEL_DIMENSION * 0.5;
  let z_offset: f32 = 0.0;

  let image_bytes = image.data();
  let mut material_indicies = HashMap::<ColorT, usize>::new();
  let mut materials = Vec::<json::Material>::new();
  let mut meshes = Vec::<json::Mesh>::new();
  let mut nodes = Vec::<json::Node>::new();

  console::log_3(&image.width().into(), &image.height().into(), &image_bytes.len().into());

  for y in 0..image.height() {
    for x in 0..image.width() {
      let data_index = (image.height() * 4 * y + 4 * x) as usize;
      //console::log_3(&x.into(), &y.into(), &data_index.into());
      if let [r, g, b, a] = image_bytes[data_index..(data_index + 3)] {
        console::log_1(&"image_to_gltf got color".into());
        if a != 0 {
          console::log_3(&x.into(), &y.into(), &a.into());
          let mat_ind: usize = match material_indicies.get(&[r, g, b, a]) {
            Some(ind) => *ind,
            None => {
              materials.push(json::Material {
                alpha_cutoff: None,
                alpha_mode: json::validation::Checked::Valid(json::material::AlphaMode::Opaque),
                double_sided: false,
                name: None,
                pbr_metallic_roughness: json::material::PbrMetallicRoughness {
                  base_color_factor: json::material::PbrBaseColorFactor([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0]),
                  base_color_texture: None,
                  metallic_factor: json::material::StrengthFactor(0.0),
                  roughness_factor: json::material::StrengthFactor(1.0),
                  metallic_roughness_texture: None,
                  extensions: Default::default(),
                  extras: Default::default(),
                },
                normal_texture: None,
                occlusion_texture: None,
                emissive_texture: None,
                emissive_factor: json::material::EmissiveFactor::default(),
                extensions: Default::default(),
                extras: Default::default()
              });
              let mat_ind = materials.len() - 1;

              meshes.push(json::Mesh {
                name: None,
                extras: Default::default(),
                extensions: Default::default(),
                primitives: vec![json::mesh::Primitive {
                  attributes: {
                    let mut map = HashMap::new();
                    map.insert(
                      Valid(Semantic::Positions),
                      json::Index::<json::Accessor>::new(0),
                    );
                    map
                  },
                  indices: None,
                  material: Some(json::Index::<json::Material>::new(mat_ind as u32)),
                  mode: Valid(json::mesh::Mode::Triangles),
                  targets: None,
                  extensions: Default::default(),
                  extras: Default::default(),
                }],
                weights: None,
              });
              material_indicies.insert([r, g, b, a], mat_ind);
              mat_ind 
            }
          };

          nodes.push(json::Node {
            camera: None,
            children: None,
            extensions: Default::default(),
            extras: Default::default(),
            matrix: None,
            mesh: Some(json::Index::<json::Mesh>::new(mat_ind as u32)),
            name: None,
            rotation: None,
            scale: None,
            skin: None,
            translation: Some([
              x_offset + (x as f32) * VOXEL_DIMENSION,
              y_offset - (y as f32) * VOXEL_DIMENSION,
              z_offset,
            ]),
            weights: None,
          });
        };
      };
    };
  };

  let result = json::Root {
    accessors: vec![],
    animations: vec![],
    asset: json::Asset {
      extensions: Default::default(),
      extras: Default::default(),
      generator: None,
      version: "2.0".to_string(),
      copyright: None,
      min_version: None,
    },
    buffers: vec![],
    buffer_views: vec![],
    cameras: vec![],
    extensions: Default::default(),
    extras: Default::default(),
    images: vec![],
    materials: materials,
    meshes: meshes,
    nodes: nodes.clone(),
    scene: Some(json::Index::<json::Scene>::new(0)),
    scenes: vec![
      json::Scene {
        name: Some("default".to_string()),
        extensions: Default::default(),
        extras: Default::default(),
        nodes: nodes.iter().enumerate().map(|(i, _)| json::Index::<json::Node>::new(i as u32)).collect(),
      }
    ],
    skins: vec![],
    textures: vec![],
    extensions_required: Default::default(),
    extensions_used: Default::default(),
    samplers: Default::default(),
  };

  json::serialize::to_string(&result).unwrap()
}
