#![forbid(unsafe_code)]
use crate::handle::{
    DirHandle, FileHandle, Handle,
    Handle::{Content, Dir, File},
};
use std::{
    collections::HashSet,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

type Callback<'a> = dyn FnMut(&mut Handle) + 'a;

#[derive(Default)]
pub struct Walker<'a> {
    callbacks: Vec<Box<Callback<'a>>>,
}

impl<'a> Walker<'a> {
    pub fn new() -> Self {
        // TODO: your code goes here.
        Self { callbacks: vec![] }
    }

    pub fn add_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Handle) + 'a,
    {
        // TODO: your code goes here.
        self.callbacks.push(Box::new(callback));
    }

    pub fn add_boxed_callback<F>(&mut self, callback: Box<F>)
    where
        F: FnMut(&mut Handle) + 'a,
    {
        // TODO: your code goes here.
        self.callbacks.push(callback);
    }

    pub fn walk<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        // TODO: your code goes here.
        if !path.as_ref().exists() && !self.callbacks.is_empty() {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                format!(
                    "The path {} does not exist.",
                    path.as_ref().to_str().unwrap()
                ),
            ));
        }
        if path.as_ref().is_dir() {
            for entry_res in std::fs::read_dir(path.as_ref().clone())? {
                let entry = entry_res?;
                let p = entry.path();
                let mut dir_paths: HashSet<PathBuf> = HashSet::new();
                let mut w = Walker::new();
                if p.is_dir() {
                    for c in &mut self.callbacks {
                        let mut dir = Dir {
                            0: DirHandle {
                                entry_: &entry,
                                path_: p.clone(),
                                descent_: false,
                            },
                        };
                        c(&mut dir);
                        if let Handle::Dir(handler) = dir {
                            if handler.descent_ {
                                dir_paths.insert(p.clone());
                                w.add_callback(c);
                            }
                        }
                    }
                } else {
                    let mut contents: Vec<u8> = vec![];
                    let mut contents_is_read = false;
                    for c in &mut self.callbacks {
                        let mut file = File {
                            0: FileHandle {
                                entry_: &entry,
                                path_: p.clone(),
                                read_: false,
                            },
                        };
                        c(&mut file);
                        if let Handle::File(handler) = file {
                            if handler.read_ {
                                contents_is_read = if !contents_is_read {
                                    contents = std::fs::read(handler.path())?;
                                    true
                                } else {
                                    true
                                };
                                c(&mut Content {
                                    file_path: handler.path(),
                                    content: &contents,
                                });
                            }
                        }
                    }
                }
                println!("walk dir_paths len {:}", dir_paths.len());
                for p in dir_paths {
                    w.walk(p)?
                }
            }
        }
        Ok(())
    }

    // TODO: your code goes here.
}
