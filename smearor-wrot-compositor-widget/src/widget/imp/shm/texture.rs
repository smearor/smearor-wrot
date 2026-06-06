use glib::Bytes;
use gtk4::gdk;
use gtk4::gdk::MemoryTexture;
use smearor_wrot_compositor::buffer::metadata::BufferMetadata;
use smearor_wrot_compositor::texture::cache::TextureCacheEntry;
use smearor_wrot_compositor::texture::pixel_data::BGRA;
use smearor_wrot_compositor::texture::pixel_data::PixelData;

pub fn create_memory_texture_bgra(texture_cache_entry: &TextureCacheEntry<BGRA>) -> MemoryTexture {
    create_memory_texture_from_pixel_data_bgra(&texture_cache_entry.buffer_metadata, &texture_cache_entry.pixel_data)
}

pub fn create_memory_texture_from_pixel_data_bgra(buffer_metadata: &BufferMetadata, pixel_data: &PixelData<BGRA>) -> MemoryTexture {
    let pixel_bytes = Bytes::from(&pixel_data[..]);
    MemoryTexture::new(
        buffer_metadata.width,
        buffer_metadata.height,
        gdk::MemoryFormat::B8g8r8a8,
        &pixel_bytes,
        buffer_metadata.stride as usize,
    )
}
