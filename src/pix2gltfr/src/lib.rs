use gltf::json::validation::Checked::Valid;
use gltf::*;
use itertools::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{console, ImageData};

const TEMPLATE_GLTF: &[u8] = include_bytes!("template.gltf");

type ColorT = [u8; 4];
const VOXEL_DIMENSION: f32 = 1.0;

fn color_to_material(&[r, g, b, a]: &ColorT) -> json::Material {
  json::Material {
    alpha_cutoff: None,
    alpha_mode: json::validation::Checked::Valid(json::material::AlphaMode::Opaque),
    double_sided: false,
    name: None,
    pbr_metallic_roughness: json::material::PbrMetallicRoughness {
      base_color_factor: json::material::PbrBaseColorFactor([
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
      ]),
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
    emissive_factor: Default::default(),
    extensions: Default::default(),
    extras: Default::default(),
  }
}

#[wasm_bindgen]
pub fn image_to_gltf(image: ImageData) -> String {
  console::log_1(&"image_to_gltf starting".into());

  let (template_document, _template_buffers, _template_images) =
    gltf::import_slice(TEMPLATE_GLTF).unwrap();
  let template_document_json = template_document.into_json();
  let x_offset: f32 = -(image.width() as f32) * VOXEL_DIMENSION * 0.5;
  let y_offset: f32 = (image.height() as f32) * VOXEL_DIMENSION - VOXEL_DIMENSION * 0.5;
  let z_offset: f32 = 0.0;

  let image_bytes = image.data();

  let pix_iter = iproduct!(0..image.width(), 0..image.height())
    .map(|(x, y)| ((y * image.width() * 4 + x * 4) as usize, x, y))
    .map(|(i, x, y)| {
      (
        [
          image_bytes[i],
          image_bytes[i + 1],
          image_bytes[i + 2],
          image_bytes[i + 3],
        ] as ColorT,
        x,
        y,
      )
    })
    .filter(|(color, _x, _y)| color[3] != 0);

  let image_colors = pix_iter
    .clone()
    .map(|(color, _x, _y)| color)
    .filter(|color| color[3] != 0)
    .unique();

  let materials: Vec<json::Material> = image_colors
    .clone()
    .map(|color| color_to_material(&color))
    .collect();

  let material_indicies: HashMap<ColorT, usize> = image_colors
    .clone()
    .enumerate()
    .map(|(i, color)| (color, i))
    .collect();

  let meshes: Vec<json::Mesh> = image_colors
    .clone()
    .map(|color| json::Mesh {
      name: None,
      primitives: vec![json::mesh::Primitive {
        attributes: {
          let mut map = HashMap::new();
          map.insert(
            Valid(json::mesh::Semantic::Positions), 
            json::Index::<json::Accessor>::new(0)
          );
          map
        },
        // geometry from template
        indices: Some(json::Index::<json::Accessor>::new(1)),
        // material of same color
        material: Some(json::Index::<json::Material>::new(
          *material_indicies.get(&color).unwrap() as u32,
        )),
        mode: Valid(json::mesh::Mode::Triangles),
        extensions: Default::default(),
        extras: Default::default(),
        targets: None,
      }],
      weights: Some(vec![]),
      extensions: Default::default(),
      extras: Default::default(),
    })
    .collect();

  let nodes: Vec<json::Node> = pix_iter
    .clone()
    .map(|(color, x, y)| json::Node {
      mesh: Some(json::Index::<json::Mesh>::new(
        *material_indicies.get(&color).unwrap() as u32,
      )),
      name: None,
      rotation: None,
      scale: None,
      translation: Some([
        x_offset + x as f32 * VOXEL_DIMENSION,
        y_offset - y as f32 * VOXEL_DIMENSION,
        z_offset,
      ]),
      camera: None,
      children: Some(vec![]),
      matrix: None,
      skin: None,
      weights: Some(vec![]),
      extensions: Default::default(),
      extras: Default::default(),
    })
    .collect();

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
    scenes: vec![json::Scene {
      name: Some("default".to_string()),
      extensions: Default::default(),
      extras: Default::default(),
      nodes: nodes
        .iter()
        .enumerate()
        .map(|(i, _)| json::Index::<json::Node>::new(i as u32))
        .collect(),
    }],
    skins: vec![],
    textures: vec![],
    ..Default::default()
  };

  json::serialize::to_string(&result).unwrap()
}
