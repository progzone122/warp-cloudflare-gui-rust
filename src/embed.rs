use std::sync::OnceLock;
use iced::widget::image::Handle;
use rust_embed::RustEmbed;
use bytes::Bytes;

#[derive(RustEmbed)]
#[folder = "images/"]
pub struct Images;

fn load_image(name: &str) -> Option<Handle> {
    if let Some(image_data) = Images::get(name) {
        let bytes: &[u8] = &image_data.data;
        let handle: Handle = Handle::from_bytes(Bytes::copy_from_slice(bytes));

        return Some(handle);
    }

    None
}
pub fn get_image(cell: &OnceLock<Handle>, path: &str) {
    cell.get_or_init(|| {
        load_image(path).unwrap_or_else(|| {
            eprintln!("ERROR: Error loading image: {}", path);
            "".into()
        })
    });
}