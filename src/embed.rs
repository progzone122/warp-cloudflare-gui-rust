use iced::widget::image::Handle;
use rust_embed::RustEmbed;
use bytes::Bytes;

#[derive(RustEmbed)]
#[folder = "images/"]
pub struct Images;

pub fn load_image(name: &str) -> Option<Handle> {
    if let Some(image_data) = Images::get(name) {
        let bytes: &[u8] = &image_data.data;
        let handle: Handle = Handle::from_bytes(Bytes::copy_from_slice(bytes));

        return Some(handle);
    }

    None
}