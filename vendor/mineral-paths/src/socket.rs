//! Runtime / IPC socket 目录解析与加固。

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

use crate::{Error, Result};

/// `sun_path` 字段上限(含结尾 NUL):macOS 104 / Linux 108 字节。
#[cfg(target_os = "macos")]
const SUN_PATH_MAX: usize = 104;

/// `sun_path` 字段上限(含结尾 NUL):macOS 104 / Linux 108 字节。
#[cfg(target_os = "linux")]
const SUN_PATH_MAX: usize = 108;

/// IPC socket 文件名(固定)。
const SOCKET_FILE: &str = "mineral.sock";

/// 解析 runtime 目录(socket 落它下面)。**不创建**。
pub(crate) fn runtime_dir() -> Result<PathBuf> {
    if let Some(v) = std::env::var_os("MINERAL_SOCKET_DIR").filter(|v| !v.is_empty())
        && Path::new(&v).is_absolute()
    {
        return Ok(PathBuf::from(v));
    }
    #[cfg(unix)]
    {
        if let Some(dir) = xdg_runtime_dir() {
            return Ok(dir);
        }
        let uid = nix::unistd::geteuid().as_raw();
        Ok(std::env::temp_dir().join(format!("mineral-{uid}")))
    }
    #[cfg(windows)]
    {
        Ok(std::env::temp_dir().join("mineral"))
    }
}

/// `$XDG_RUNTIME_DIR/mineral`:仅当其为绝对路径、且(若已存在)属主是当前用户;否则 `None`(退 tmp)。
#[cfg(unix)]
fn xdg_runtime_dir() -> Option<PathBuf> {
    let v = std::env::var_os("XDG_RUNTIME_DIR").filter(|v| !v.is_empty())?;
    if !Path::new(&v).is_absolute() {
        return None;
    }
    if let Ok(md) = std::fs::metadata(&v)
        && md.uid() != nix::unistd::geteuid().as_raw()
    {
        return None;
    }
    Some(PathBuf::from(v).join("mineral"))
}

/// IPC socket 的完整路径(`<runtime_dir>/mineral.sock`)。
pub(crate) fn socket_path() -> Result<PathBuf> {
    let dir = runtime_dir()?;
    std::fs::create_dir_all(&dir).map_err(|source| Error::RuntimeDirectory {
        operation: "创建",
        path: dir.clone(),
        source,
    })?;
    #[cfg(unix)]
    harden_dir(&dir)?;
    let sock = dir.join(SOCKET_FILE);
    #[cfg(unix)]
    {
        let len = sock.as_os_str().as_bytes().len();
        if len >= SUN_PATH_MAX {
            return Err(Error::SocketPathTooLong {
                length: len,
                maximum: SUN_PATH_MAX,
                path: sock,
            });
        }
    }
    Ok(sock)
}

/// 校验目录属主是当前用户,并把权限收紧到 `0700`。
#[cfg(unix)]
fn harden_dir(dir: &Path) -> Result<()> {
    let md = std::fs::metadata(dir).map_err(|source| Error::RuntimeDirectory {
        operation: "检查",
        path: dir.to_path_buf(),
        source,
    })?;
    let me = nix::unistd::geteuid().as_raw();
    if md.uid() != me {
        return Err(Error::RuntimeOwner {
            path: dir.to_path_buf(),
            owner: md.uid(),
            current: me,
        });
    }
    std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700)).map_err(|source| {
        Error::RuntimeDirectory {
            operation: "收紧权限",
            path: dir.to_path_buf(),
            source,
        }
    })?;
    Ok(())
}
