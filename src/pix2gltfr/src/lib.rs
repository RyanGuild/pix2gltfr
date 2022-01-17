use gltf::json::validation::Checked::Valid;
use gltf::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{ImageData, console};

const template_gltf: &[u8] = include_bytes!("template.gltf");


type ColorT = [u8; 4];
const VOXEL_DIMENSION: f32 = 1.0;

#[wasm_bindgen]
pub fn image_to_gltf(image: ImageData) -> String {
  console::log_1(&"image_to_gltf starting".into());

  let (template_document, template_buffers, template_images) = gltf::import_slice(template_gltf).unwrap();
  let template_document_json = template_document.into_json();
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
      let data_index = ((image.width() * 4 * y) + (4 * x)) as usize;
      if let [r, g, b, a] = image_bytes[data_index..(data_index + 4)] {
        if a != 0 {
          console::log_4(&image_bytes[data_index].into(),&image_bytes[data_index+1].into(),&image_bytes[data_index+2].into(),&image_bytes[data_index+3].into());
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
                    let mut map = HashMap::<json::validation::Checked<json::mesh::Semantic>, json::Index<json::Accessor>>::new();
                    
                    map.insert(
                      json::validation::Checked::Valid(
                        json::mesh::Semantic::Positions
                      ), 
                      json::Index::new(0)
                    );
                    
                    map
                  },
                  indices: Some(json::Index::new(1)),
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
    accessors: template_document_json.accessors.clone(),
    animations: template_document_json.animations.clone(),
    asset: template_document_json.asset.clone(),
    buffers: template_document_json.buffers.clone(),
    buffer_views: template_document_json.buffer_views.clone(),
    cameras: template_document_json.cameras.clone(),
    extensions: template_document_json.extensions.clone(),
    extras: template_document_json.extras.clone(),
    images: template_document_json.images.clone(),
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
