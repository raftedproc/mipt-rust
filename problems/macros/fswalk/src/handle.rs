#![forbid(unsafe_code)]

use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

pub enum Handle<'a> {
    Dir(DirHandle<'a>),
    File(FileHandle<'a>),
    Content {
        file_path: &'a Path,
        content: &'a [u8],
    },
}

pub struct DirHandle<'a> {
    // TODO: your code goes here.
    pub entry_: &'a std::fs::DirEntry,
    // pub u_: PhantomData<&'a u32>,
    pub path_: PathBuf,
    pub descent_: bool,
}

impl<'a> DirHandle<'a> {
    pub fn descend(&mut self) {
        // TODO: your code goes here.
        self.descent_ = true;
    }

    pub fn path(&self) -> &Path {
        // TODO: your code goes here.
        self.path_.as_path()
    }
}

pub struct FileHandle<'a> {
    // TODO: your code goes here.
    pub entry_: &'a std::fs::DirEntry,
    pub path_: PathBuf,
    // pub content_: Vec<u8>,
    pub read_: bool,
}

impl<'a> FileHandle<'a> {
    pub fn read(&mut self) {
        // TODO: your code goes here.
        self.read_ = true;
    }

    pub fn path(&self) -> &Path {
        // TODO: your code goes here.
        self.path_.as_path()
    }
}
