//! Current-user DPAPI only. TokenDaily's own authorization, never host credentials.
use std::ptr::{null, null_mut};
use windows_sys::Win32::{
    Foundation::LocalFree,
    Security::Cryptography::{
        CryptProtectData, CryptUnprotectData, CRYPTPROTECT_UI_FORBIDDEN, CRYPT_INTEGER_BLOB,
    },
    UI::Shell::ShellExecuteW,
};
fn crypt(bytes: &[u8], protect: bool) -> Result<Vec<u8>, &'static str> {
    let input = CRYPT_INTEGER_BLOB {
        cbData: u32::try_from(bytes.len()).map_err(|_| "授权数据过大")?,
        pbData: bytes.as_ptr() as *mut u8,
    };
    let mut output = CRYPT_INTEGER_BLOB {
        cbData: 0,
        pbData: null_mut(),
    };
    let ok = unsafe {
        if protect {
            CryptProtectData(
                &input,
                null(),
                null(),
                null(),
                null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        } else {
            CryptUnprotectData(
                &input,
                null_mut(),
                null(),
                null(),
                null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        }
    };
    if ok == 0 {
        return Err("Windows 授权保护失败，请重新登录");
    }
    let result =
        unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize).to_vec() };
    unsafe {
        LocalFree(output.pbData as _);
    }
    Ok(result)
}
pub(crate) fn protect_secret(v: &[u8]) -> Result<Vec<u8>, &'static str> {
    crypt(v, true)
}
pub(crate) fn unprotect_secret(v: &[u8]) -> Result<Vec<u8>, &'static str> {
    crypt(v, false)
}
pub(crate) fn open_official_url(url: &str) -> Result<(), &'static str> {
    // Shared layer has validated the exact official HTTPS host before calling this.
    let operation: Vec<u16> = "open\0".encode_utf16().collect();
    let target: Vec<u16> = url.encode_utf16().chain(Some(0)).collect();
    let result = unsafe {
        ShellExecuteW(
            null_mut(),
            operation.as_ptr(),
            target.as_ptr(),
            null(),
            null(),
            1,
        )
    };
    if result as isize <= 32 {
        Err("无法打开默认浏览器，请检查系统设置")
    } else {
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn own_authorization_roundtrips_and_rejects_corruption() {
        let plain = b"synthetic-token-not-a-credential";
        let mut protected = protect_secret(plain).unwrap();
        assert_ne!(&protected, plain);
        assert_eq!(unprotect_secret(&protected).unwrap(), plain);
        protected[0] ^= 255;
        assert!(unprotect_secret(&protected).is_err());
    }
}
