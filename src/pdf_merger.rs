use std::io::{Read, Result, Write};
use std::path::PathBuf;

pub struct Pdf {
    files: Vec<PathBuf>,
}

impl Pdf {
    pub fn new() -> Self {
        Self { files: vec![] }
    }

    pub fn append<R>(&mut self, mut reader: R) -> Result<()>
    where
        R: Read,
    {
        let path = mktemp::Temp::new_file().unwrap().to_path_buf();
        let mut f = std::fs::File::create(&path)?;
        std::io::copy(&mut reader, &mut f)?;
        self.files.push(path);
        Ok(())
    }

    pub fn write_to<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        let mut cmd = std::process::Command::new("gs");
        let mut cmd = cmd
            .arg("-dNOPAUSE")
            .arg("-sDEVICE=pdfwrite")
            .arg("-sOUTPUTFILE=-")
            .arg("-dBATCH")
            .arg("-dQUIET");
        for path in &self.files {
            cmd = cmd.arg(path);
        }
        writer.write_all(&cmd.output()?.stdout)?;
        Ok(())
    }
}

impl Drop for Pdf {
    fn drop(&mut self) {
        for path in self.files.drain(..) {
            let _ = std::fs::remove_file(path);
        }
    }
}
