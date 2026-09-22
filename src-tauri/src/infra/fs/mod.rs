pub mod canvas_io;
pub mod cas;
pub mod journal;
pub mod lifecycle;
pub mod recovery;
pub mod traits;
pub mod vault;
#[cfg(feature = "app")]
pub mod watcher;
pub mod writer;

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Garante a sincronização física dos buffers de arquivo com a mídia não-volátil (disco)
pub fn sync_file(file: &File) -> io::Result<()> {
    file.sync_all()
}

/// Sincroniza as alterações de metadados do diretório pai com a mídia de disco
pub fn sync_dir(dir_path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        if let Ok(dir) = File::open(dir_path) {
            let _ = dir.sync_all();
        }
    }
    #[cfg(windows)]
    {
        // No Windows, o sistema operacional gerencia metadata writes;
        // abrir com FILE_FLAG_BACKUP_SEMANTICS e FlushFileBuffers se disponível
        let _ = dir_path;
    }
    Ok(())
}

/// Executa a substituição atômica de arquivos no sistema operacional
pub fn atomic_rename(from: &Path, to: &Path) -> io::Result<()> {
    #[cfg(windows)]
    {
        extern "system" {
            fn MoveFileExW(
                lpExistingFileName: *const u16,
                lpNewFileName: *const u16,
                dwFlags: u32,
            ) -> i32;
        }
        use std::os::windows::ffi::OsStrExt;
        let from_wide: Vec<u16> = from.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
        let to_wide: Vec<u16> = to.as_os_str().encode_wide().chain(std::iter::once(0)).collect();

        unsafe {
            // MOVEFILE_REPLACE_EXISTING (0x1) | MOVEFILE_WRITE_THROUGH (0x8)
            let flags = 0x00000001 | 0x00000008;
            let res = MoveFileExW(from_wide.as_ptr(), to_wide.as_ptr(), flags);
            if res == 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    #[cfg(not(windows))]
    {
        std::fs::rename(from, to)
    }
}

/// Grava conteúdo em um arquivo temporário com flush físico e substitui atomicamente no destino
pub fn write_file_atomic(destination: &Path, content: &[u8]) -> io::Result<()> {
    let parent = destination.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Caminho de destino sem diretório pai")
    })?;

    let tmp_file_name = format!(".tmp_{}", uuid::Uuid::new_v4());
    let tmp_path = parent.join(tmp_file_name);

    {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&tmp_path)?;
        file.write_all(content)?;
        file.sync_all()?;
    }

    atomic_rename(&tmp_path, destination).map_err(|e| {
        let _ = std::fs::remove_file(&tmp_path);
        e
    })?;

    sync_dir(parent)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_atomic_write_and_replace() {
        let temp_dir = std::env::temp_dir().join(format!("sandland_atomic_test_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();

        let target_file = temp_dir.join("test_file.txt");
        let initial_data = b"Hello, Sandland Initial!";
        write_file_atomic(&target_file, initial_data).unwrap();

        let read_data = fs::read(&target_file).unwrap();
        assert_eq!(read_data, initial_data);

        let updated_data = b"Hello, Sandland Updated Atomic!";
        write_file_atomic(&target_file, updated_data).unwrap();

        let read_updated = fs::read(&target_file).unwrap();
        assert_eq!(read_updated, updated_data);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
