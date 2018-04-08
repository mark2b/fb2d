extern crate tempdir;
extern crate zip;
extern crate unzip;

use std::error;
use std::ffi;
use std::fs;
use std::io;
use std::path;
use std::rc;

pub struct SceneBundle<'a> {
    source_path: Option<&'a str>,
    pub temp_dir: Option<tempdir::TempDir>,
}

impl<'a> SceneBundle<'a> {
    pub fn new() -> Self {
        SceneBundle {
            source_path: None,
            temp_dir: None,
        }
    }

//    pub fn new3(path: &'a str) -> Result<Self, String>  {
//        let mut bundle_path = path::Path::new(path);
//        if bundle_path.is_dir() {
//            Ok(SceneBundle {
//                source_path: Some(path),
//                temp_dir: None,
//            })
//        } else if bundle_path.is_file() {
//            if let Some(extension) = bundle_path.extension() {
//                if extension == "zip" || extension == "scene" {
//                    match tempdir::TempDir::new("scene") {
//                        Ok(ref temp_dir) => {
//                            let temp_dir_path = temp_dir.path();
//                            let scene_bundle = SceneBundle {
//                                source_path: Some(path),
//                                temp_dir: Some(*temp_dir),
//                            };
////                            if let &Some(ref temp_dir) = &scene_bundle.temp_dir {
//                                match fs::File::open(path) {
//                                    Ok(archive_file) => {
//                                        let unzipper = unzip::Unzipper::new(archive_file, temp_dir_path);
//                                        unzipper.unzip();
//                                        Ok(scene_bundle)
//                                    },
//                                    Err(e) => Err(format!("{:?}", e))
//                                }
////                            } else {
////                                Err(format!("Missing temp_dir parameter"))
////                            }
//                        },
//                        Err(e) => Err(format!("{:?}", e))
//                    }
//                } else {
//                    Err(format!("Wrong bundle file. Expected .zip or .scene but {}.", path))
//                }
//            } else {
//                Err(format!("Wrong bundle file. Expected .zip or .scene but {}.", path))
//            }
//        } else {
//            Err(format!("Wrong bundle path. {}.", path))
//        }
////
////
////
////        if let Ok(temp_dir) = tempdir::TempDir::new("scene") {
////
////        } else {
////            Err(String::from(""))
////        }
//    }

    pub fn open(&mut self, path: &'a str) -> Result<(), String> {
        if let Ok(temp_dir) = tempdir::TempDir::new("scene") {
            self.temp_dir = Some(temp_dir);
            self.source_path = Some(path);

            if let &Some(ref temp_dir2) = &self.temp_dir {

                let archive_file = fs::File::open(path).unwrap();
                let unzipper = unzip::Unzipper::new(archive_file, temp_dir2.path());
                unzipper.unzip();

            }

            Ok(())
        } else {
            Err(String::from(""))
        }
    }

    pub fn target_path(&self) -> path::PathBuf {
        if let &Some(ref temp_dir) = &self.temp_dir {
            temp_dir.path().to_path_buf()
        } else if let Some(source_path) = self.source_path {
            path::Path::new(source_path).to_path_buf()
        } else {
            path::Path::new("").to_path_buf()
        }
    }
}

//pub struct SceneBundle<'a> {
//    pub path:&'a str
//}
//
//impl<'a> SceneBundle<'a> {
//
//    pub fn reader_by_name(&self, resource_name : &str) -> zip::read::ZipFile<'a> {
//        use std::io::Read;
//
//        let archive = zip::ZipArchive::new(fs::File::open("/Users/mberner/projects/markberner/fb-2d/assets/scene1.zip").unwrap()).unwrap();
//        archive.by_name(resource_name).unwrap()
//
////        return zip_file;
//    }
//
//    pub fn bufreader_by_name<T>(&self, resource_name : &str) -> &'a io::BufRead {
//        use std::io::Read;
//
//        let mut archive = zip::ZipArchive::new(fs::File::open("/Users/mberner/projects/markberner/fb-2d/assets/scene1.zip").unwrap()).unwrap();
//        let zip_file = archive.by_name(resource_name).unwrap();
//
//        let mut reader = io::BufReader::new(zip_file);
//        return &reader;
//    }
//}

//impl<'a> SceneBundle<'a> {
//    pub fn new(path:&'a str) -> Self {
//        SceneBundle {
//            path : path
//        }
//    }
//
//    pub fn open(&'a self) -> Result<&'a ResourceBundle, String> {
//        let mut bundle_path = path::Path::new(self.path);
//        if bundle_path.is_dir() {
//            let bundle = DirectoryResourceBundle {
//                path : self.path
//            };
//            return Ok(&bundle);
//        } else if bundle_path.is_file() {
//            if let Some(extension) = bundle_path.extension() {
//                if extension == "zip" || extension == "scene" {
//                    match fs::File::open(self.path) {
//                        Ok(file) => {
//                            match zip::ZipArchive::new(file) {
//                                Ok(archive) => {
//                                    let bundle = ArchiveResourceBundle {
//                                        archive : &archive
//                                    };
//                                    Ok(&bundle)
//                                },
//                                Err(e) => Err(format!("{:?}", e))
//                            }
//                        },
//                        Err(e) => Err(format!("{:?}", e))
//                    }
//                } else {
//                    return Err(format!("Wrong bundle file. Expected .zip or .scene but {}", self.path));
//                }
//            } else {
//                return Err(format!("Wrong bundle file. Expected .zip or .scene but {}", self.path));
//            }
//        } else {
//            return Err(format!("Wrong bundle path. {}", self.path));
//        }
//    }
//
//}
//
////pub fn new_resource_bundle<'a>(path:&str) -> Result<Box<ResourceBundle+'a>, String> {
////    let mut bundle_path = path::Path::new(path);
////    if bundle_path.is_dir() {
////        let bundle = DirectoryResourceBundle {
////            path : String::from(path)
////        };
////        Ok(Box::new(bundle))
////    } else if bundle_path.is_file() {
////        if let Some(extension) = bundle_path.extension() {
////            if extension == "zip" || extension == "scene" {
////                match fs::File::open(path) {
////                    Ok(file) => {
////                        match zip::ZipArchive::new(file) {
////                            Ok(archive) => {
////                                let bundle = ArchiveResourceBundle {
////                                    archive : &archive
////                                };
////                                Ok(Box::new(bundle))
////                            },
////                            Err(e) => Err(format!("{:?}", e))
////                        }
////                    },
////                    Err(e) => Err(format!("{:?}", e))
////                }
////            } else {
////                return Err(format!("Wrong bundle file. Expected .zip or .scene but {}", path));
////            }
////        } else {
////            return Err(format!("Wrong bundle file. Expected .zip or .scene but {}", path));
////        }
////    } else {
////        return Err(format!("Wrong bundle path. {}", path));
////    }
////}
//
//pub struct DirectoryResourceBundle<'a> {
//    path:&'a str
//}
//
//pub struct ArchiveResourceBundle<'a> {
//    archive:&'a zip::ZipArchive<fs::File>
//}
//
//pub trait ResourceBundle<'a> {
//    fn reader_by_name(&self, resource_name:&str) -> Result<Box<io::Read>, String>;
//    fn file_by_name(&self, resource_name:&str) -> Result<fs::File, String>;
//}
//
//impl<'a> ResourceBundle<'a> for DirectoryResourceBundle<'a> {
//    fn reader_by_name(&self, resource_name:&str) -> Result<Box<io::Read>, String> {
//        let resource_path = path::Path::new(self.path.as_ref() as &str).join(resource_name);
//        if resource_path.exists() {
//            match fs::File::open(resource_path) {
//                Ok(file) => {
//                    Ok(Box::new(file))
//                },
//                Err(e) => Err(format!("{:?}", e))
//            }
//
//        } else {
//            return Err(format!("Resource does not exist. {} of {:?}", resource_name, resource_path));
//        }
//    }
//
//    fn file_by_name(&self, resource_name:&str) -> Result<fs::File, String> {
//        let resource_path = path::Path::new(self.path.as_ref() as &str).join(resource_name);
//        if resource_path.exists() {
//            match fs::File::open(resource_path) {
//                Ok(file) => Ok(file),
//                Err(e) => Err(format!("{:?}", e))
//            }
//        } else {
//            return Err(format!("Resource does not exist. {} of {:?}", resource_name, resource_path));
//        }
//    }
//}
//
//impl<'a> ResourceBundle<'a> for ArchiveResourceBundle<'a> {
//    fn reader_by_name(&self, resource_name:&str) -> Result<Box<io::Read>, String> {
////        let file = self.archive.by_name(resource_name).unwrap();
//        return Err(format!("Not implemented"));
////        Ok(Box::new(file))
//    }
//
//    fn file_by_name(&self, resource_name:&str) -> Result<fs::File, String> {
//        return Err(format!("Not implemented"));
//    }
//}
//
