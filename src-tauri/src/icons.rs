use crate::storage;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

pub fn cache_path_for(source: &str) -> Option<PathBuf> {
    let dir = storage::icons_dir().ok()?;
    let mut hasher = Sha256::new();
    hasher.update(source.as_bytes());
    let hash = hasher.finalize();
    let name = format!("{:x}.png", hash);
    Some(dir.join(name))
}

pub fn read_cached(source: &str) -> Option<String> {
    let path = cache_path_for(source)?;
    let bytes = fs::read(&path).ok()?;
    Some(BASE64.encode(bytes))
}

pub fn store_cache(source: &str, png: &[u8]) -> Result<(), String> {
    let path = cache_path_for(source).ok_or_else(|| "Cache indisponible".to_string())?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("Cache: {error}"))?;
    }
    fs::write(&path, png).map_err(|error| format!("Cache: {error}"))
}

pub fn clean_icon_path(raw: &str) -> String {
    let trimmed = raw.trim().trim_matches('"');
    if let Some((path, _index)) = trimmed.rsplit_once(',') {
        let candidate = path.trim().trim_matches('"');
        if candidate.contains(':') || candidate.starts_with("\\\\") {
            return candidate.to_string();
        }
    }
    trimmed.to_string()
}

#[cfg(windows)]
pub fn extract_icon_png(source_path: &str) -> Result<Vec<u8>, String> {
    use std::os::windows::ffi::OsStrExt;
    use std::path::Path;

    let cleaned = clean_icon_path(source_path);
    let path = Path::new(&cleaned);
    if !path.exists() {
        return Err(format!("Chemin introuvable: {cleaned}"));
    }

    let wide: Vec<u16> = std::ffi::OsStr::new(&cleaned)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    win::extract_icon(&wide)
}

#[cfg(not(windows))]
pub fn extract_icon_png(_source_path: &str) -> Result<Vec<u8>, String> {
    Err("Extraction d'icone disponible uniquement sur Windows.".to_string())
}

#[cfg(windows)]
mod win {
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Graphics::Gdi::{
        DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BITMAP, BITMAPINFO,
        BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
    };
    use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
    use windows::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON};
    use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO};

    pub fn extract_icon(wide_path: &[u16]) -> Result<Vec<u8>, String> {
        unsafe {
            let mut info = SHFILEINFOW::default();
            let result = SHGetFileInfoW(
                PCWSTR(wide_path.as_ptr()),
                FILE_FLAGS_AND_ATTRIBUTES(0),
                Some(&mut info as *mut _),
                std::mem::size_of::<SHFILEINFOW>() as u32,
                SHGFI_ICON | SHGFI_LARGEICON,
            );

            if result == 0 || info.hIcon.is_invalid() {
                return Err("SHGetFileInfo a echoue".to_string());
            }

            let png = icon_to_png(info.hIcon);
            let _ = DestroyIcon(info.hIcon);
            png
        }
    }

    unsafe fn icon_to_png(hicon: HICON) -> Result<Vec<u8>, String> {
        let mut icon_info = ICONINFO::default();
        GetIconInfo(hicon, &mut icon_info).map_err(|error| format!("GetIconInfo: {error}"))?;

        let color_bitmap = icon_info.hbmColor;
        let mask_bitmap = icon_info.hbmMask;

        if color_bitmap.is_invalid() {
            let _ = DeleteObject(mask_bitmap.into());
            return Err("Icone sans bitmap couleur".to_string());
        }

        let mut bitmap = BITMAP::default();
        let bytes = GetObjectW(
            color_bitmap.into(),
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bitmap as *mut _ as *mut std::ffi::c_void),
        );
        if bytes == 0 {
            let _ = DeleteObject(color_bitmap.into());
            let _ = DeleteObject(mask_bitmap.into());
            return Err("GetObject a echoue".to_string());
        }

        let width = bitmap.bmWidth;
        let height = bitmap.bmHeight;
        if width <= 0 || height <= 0 {
            let _ = DeleteObject(color_bitmap.into());
            let _ = DeleteObject(mask_bitmap.into());
            return Err("Dimensions bitmap invalides".to_string());
        }

        let hdc = GetDC(Some(HWND(std::ptr::null_mut())));

        let header = BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            ..Default::default()
        };

        let mut info = BITMAPINFO {
            bmiHeader: header,
            ..Default::default()
        };

        let pixel_count = (width as usize) * (height as usize);
        let mut pixels: Vec<u8> = vec![0; pixel_count * 4];

        let scanlines = GetDIBits(
            hdc,
            color_bitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut std::ffi::c_void),
            &mut info,
            DIB_RGB_COLORS,
        );

        ReleaseDC(Some(HWND(std::ptr::null_mut())), hdc);
        let _ = DeleteObject(color_bitmap.into());
        let _ = DeleteObject(mask_bitmap.into());

        if scanlines == 0 {
            return Err("GetDIBits a echoue".to_string());
        }

        let has_alpha = pixels.chunks(4).any(|chunk| chunk[3] != 0);
        if !has_alpha {
            for chunk in pixels.chunks_mut(4) {
                chunk[3] = 0xFF;
            }
        }

        for chunk in pixels.chunks_mut(4) {
            chunk.swap(0, 2);
        }

        encode_png(&pixels, width as u32, height as u32)
    }

    fn encode_png(rgba: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut output, width, height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder
                .write_header()
                .map_err(|error| format!("PNG header: {error}"))?;
            writer
                .write_image_data(rgba)
                .map_err(|error| format!("PNG data: {error}"))?;
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::clean_icon_path;

    #[test]
    fn cleans_icon_path_with_index() {
        assert_eq!(
            clean_icon_path("C:\\Program Files\\App\\app.exe,0"),
            "C:\\Program Files\\App\\app.exe"
        );
    }

    #[test]
    fn cleans_icon_path_quoted() {
        assert_eq!(
            clean_icon_path("\"C:\\Program Files\\App\\app.exe\",0"),
            "C:\\Program Files\\App\\app.exe"
        );
    }

    #[test]
    fn cleans_plain_path() {
        assert_eq!(
            clean_icon_path("C:\\Program Files\\App\\app.exe"),
            "C:\\Program Files\\App\\app.exe"
        );
    }

    #[test]
    fn keeps_path_without_drive_letter_intact() {
        assert_eq!(clean_icon_path("foo.exe,0"), "foo.exe,0");
    }
}
