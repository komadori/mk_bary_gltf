use std::collections::HashMap;

use ::gltf as glb;
use gltf_json as gltf;
use gltf_json::validation::Checked::Valid;
use serde_json::json;

trait IndexedVec {
    type Item;
    fn push_index(&mut self, item: Self::Item) -> gltf::Index<Self::Item>;
}

impl<T> IndexedVec for Vec<T> {
    type Item = T;
    fn push_index(&mut self, item: T) -> gltf::Index<T> {
        let index = gltf::Index::new(self.len() as u32);
        self.push(item);
        index
    }
}

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    s: f32,
    t: f32,
    u: f32,
}

fn main() -> Result<(), anyhow::Error> {
    let mut buffer = Vec::<u8>::new();
    for v in [
        Vertex {
            x: -1.0,
            y: -1.0,
            z: 0.0,
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
            s: 1.0,
            t: 0.0,
            u: 0.0,
        },
        Vertex {
            x: 1.0,
            y: -1.0,
            z: 0.0,
            r: 0.5,
            g: 0.5,
            b: 0.0,
            a: 1.0,
            s: 0.0,
            t: 1.0,
            u: 0.0,
        },
        Vertex {
            x: -1.0,
            y: 1.0,
            z: 0.0,
            r: 0.5,
            g: 0.5,
            b: 0.0,
            a: 1.0,
            s: 0.0,
            t: 0.0,
            u: 1.0,
        },
        Vertex {
            x: 1.0,
            y: 1.0,
            z: 0.0,
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
            s: 1.0,
            t: 0.0,
            u: 0.0,
        },
    ] {
        buffer.extend_from_slice(&v.x.to_le_bytes());
        buffer.extend_from_slice(&v.y.to_le_bytes());
        buffer.extend_from_slice(&v.z.to_le_bytes());
        buffer.extend_from_slice(&v.r.to_le_bytes());
        buffer.extend_from_slice(&v.g.to_le_bytes());
        buffer.extend_from_slice(&v.b.to_le_bytes());
        buffer.extend_from_slice(&v.a.to_le_bytes());
        buffer.extend_from_slice(&v.s.to_le_bytes());
        buffer.extend_from_slice(&v.t.to_le_bytes());
        buffer.extend_from_slice(&v.u.to_le_bytes());
    }
    // Indices
    let ind_off = buffer.len() as u32;
    for i in [0u16, 1, 2, 2, 1, 3] {
        buffer.extend_from_slice(&i.to_le_bytes());
    }
    // GLTF
    let mut root = gltf::Root::default();
    let attr_view_idx = root.buffer_views.push_index(gltf::buffer::View {
        buffer: gltf::Index::new(0),
        byte_length: ind_off,
        byte_offset: Some(0),
        byte_stride: Some(ind_off / 4),
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(gltf::buffer::Target::ArrayBuffer)),
    });
    let ind_view_idx = root.buffer_views.push_index(gltf::buffer::View {
        buffer: gltf::Index::new(0),
        byte_length: buffer.len() as u32 - ind_off,
        byte_offset: Some(ind_off),
        byte_stride: None,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(gltf::buffer::Target::ArrayBuffer)),
    });
    let mut attr_map = HashMap::new();
    let mut attr_byte_offset = 0;
    let position_acc_idx = root.accessors.push_index(gltf::Accessor {
        buffer_view: Some(attr_view_idx),
        byte_offset: attr_byte_offset,
        count: 4,
        component_type: Valid(gltf::accessor::GenericComponentType(
            gltf::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(gltf::accessor::Type::Vec3),
        min: Some(json!([-1.0, -1.0, 0.0])),
        max: Some(json!([1.0, 1.0, 0.0])),
        name: None,
        normalized: false,
        sparse: None,
    });
    attr_map.insert(Valid(gltf::mesh::Semantic::Positions), position_acc_idx);
    attr_byte_offset += 12;
    let colour_acc_idx = root.accessors.push_index(gltf::Accessor {
        buffer_view: Some(attr_view_idx),
        byte_offset: attr_byte_offset,
        count: 4,
        component_type: Valid(gltf::accessor::GenericComponentType(
            gltf::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(gltf::accessor::Type::Vec4),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });
    attr_map.insert(Valid(gltf::mesh::Semantic::Colors(0)), colour_acc_idx);
    attr_byte_offset += 16;
    let bary_acc_idx = root.accessors.push_index(gltf::Accessor {
        buffer_view: Some(attr_view_idx),
        byte_offset: attr_byte_offset,
        count: 4,
        component_type: Valid(gltf::accessor::GenericComponentType(
            gltf::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(gltf::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });
    attr_map.insert(
        Valid(gltf::mesh::Semantic::Extras("_BARYCENTRIC".to_string())),
        bary_acc_idx,
    );
    attr_byte_offset += 12;
    let ind_acc_idx = root.accessors.push_index(gltf::Accessor {
        buffer_view: Some(ind_view_idx),
        byte_offset: 0,
        count: 6,
        component_type: Valid(gltf::accessor::GenericComponentType(
            gltf::accessor::ComponentType::U16,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(gltf::accessor::Type::Scalar),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    });
    let prim = gltf::mesh::Primitive {
        attributes: attr_map,
        extensions: Default::default(),
        extras: Default::default(),
        indices: Some(ind_acc_idx),
        material: None,
        mode: Valid(gltf::mesh::Mode::Triangles),
        targets: None,
    };
    root.meshes.push_index(gltf::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        primitives: vec![prim],
        weights: None,
    });
    root.buffers.push(gltf::Buffer {
        byte_length: buffer.len() as u32,
        name: None,
        uri: None,
        extensions: Default::default(),
        extras: Default::default(),
    });
    let json_string = gltf::serialize::to_string(&root)?;
    let json_bytes = json_string.into_bytes();
    let json_offset = json_bytes.len() as u32;
    let glb = glb::binary::Glb {
        header: glb::binary::Header {
            magic: b"glTF".clone(),
            version: 2,
            length: json_offset + buffer.len() as u32,
        },
        bin: Some(std::borrow::Cow::Owned(buffer)),
        json: std::borrow::Cow::Owned(json_bytes),
    };
    let writer = std::fs::File::create("barycentric.glb")?;
    glb.to_writer(writer)?;
    Ok(())
}
