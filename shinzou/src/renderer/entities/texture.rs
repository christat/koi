use std::path::PathBuf;
//----------------------------------------------------------------------------------------------------------------------

#[allow(unused_imports)]
use image::GenericImageView;
//----------------------------------------------------------------------------------------------------------------------

pub enum MapType {
    Diffuse,
    Normal,
}
//----------------------------------------------------------------------------------------------------------------------

pub struct Texture {
    pub name: String,
    pub file_path: PathBuf,
    pub map_type: MapType,
}
//----------------------------------------------------------------------------------------------------------------------

pub struct TextureRaw {
    pub buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub bit_depth: u32,
    pub size: u32,
}
//----------------------------------------------------------------------------------------------------------------------

impl Texture {
    pub fn new(name: &String, file_path: PathBuf, map_type: Option<MapType>) -> Self {
        Self {
            name: name.to_owned(),
            file_path,
            map_type: map_type.unwrap_or(MapType::Diffuse),
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn load_raw_from_file(&self) -> TextureRaw {
        let img = image::open(&self.file_path).expect(&format!(
            "Failed to open texture file {:?}",
            &self.file_path
        ));

        let width = img.width();
        let height = img.height();
        let bit_depth = 4; // TODO derive
        let size = width * height * bit_depth;

        TextureRaw {
            buffer: img.into_bytes(),
            width,
            height,
            bit_depth,
            size,
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
