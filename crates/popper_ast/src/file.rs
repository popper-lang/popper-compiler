use crate::ast::LangAst;
use popper_index::Idx;
use std::path::Path;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FileId(u32);

impl Idx for FileId {
    const MAX_ID: usize = u32::MAX as usize;
    const MAX: Self = FileId(u32::MAX);
    fn new(val: usize) -> Self {
        FileId(u32::new(val))
    }

    fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone)]
pub struct File {
    id: FileId,
    info: SourceFileInfo,
    ast: Option<LangAst>,
}

impl File {
    pub fn new(id: FileId, info: SourceFileInfo, ast: Option<LangAst>) -> File {
        File { id, info, ast }
    }
    pub fn info(&self) -> &SourceFileInfo {
        &self.info
    }
}

#[derive(Clone)]
pub struct FileTable {
    files: Vec<File>,
}

impl FileTable {
    pub fn new() -> FileTable {
        FileTable { files: Vec::new() }
    }

    pub fn insert(&mut self, info: SourceFileInfo, ast: Option<LangAst>) -> FileId {
        let id = FileId::new(self.files.len());
        let file = File::new(id, info, ast);
        self.files.push(file);
        id
    }

    pub fn push(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn get(&self, id: FileId) -> Option<&File> {
        self.files.get(id.index())
    }
}

#[derive(Debug, Clone)]
pub struct SourceFileInfo {
    name: String,
    source: String,
    hash: u64,
    path: String,
    absolute_path: String,
}

impl SourceFileInfo {
    pub fn from_file(raw_path: &str) -> Option<SourceFileInfo> {
        let path = Path::new(raw_path);
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let source = std::fs::read_to_string(path).ok()?;
        let hash = crc::Crc::<u64>::new(&crc::CRC_64_MS).checksum(source.as_bytes());

        Some(SourceFileInfo {
            name,
            source,
            hash,
            path: raw_path.to_string(),
            absolute_path: path.to_str()?.to_string(),
        })
    }

    pub fn new(
        name: String,
        source: String,
        hash: u64,
        path: String,
        absolute_path: String,
    ) -> SourceFileInfo {
        SourceFileInfo {
            name,
            source,
            hash,
            path,
            absolute_path,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn absolute_path(&self) -> &str {
        &self.absolute_path
    }
}
