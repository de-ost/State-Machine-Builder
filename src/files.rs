use std::path::PathBuf;

/// A collection of files. Contains the path to the directory and the files.
#[derive(Debug)]
pub struct Files {
    path: PathBuf,
    files: Vec<File>,
}

/// A file. Contains the name of the file and its content.
#[derive(Debug)]
struct File {
    name: String,
    content: String,
}

impl Files {
    /// Creates a new collection of files.
    pub fn new(path: PathBuf) -> Self {
        Files {
            path,
            files: Vec::new(),
        }
    }

    /// Adds a file to the collection.
    pub fn add_file(&mut self, name: String, content: String) {
        self.files.push(File { name, content });
    }

    /// Writes the files to the directory. Returns a message.
    pub fn write(&self) -> Result<String, std::io::Error> {
        std::fs::create_dir_all(&self.path)?;

        for file in &self.files {
            let path = self.path.join(&file.name);
            std::fs::write(path, &file.content)?;
        }

        Ok(format!(
            "The files have been written to {}.",
            self.path.to_str().unwrap()
        ))
    }
}
