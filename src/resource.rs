use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::convert::AsRef;
use std::collections::BTreeMap;
use image;
use image::GenericImage;
use glium::Display;
use texture::Texture;


pub type Map<T> = BTreeMap<PathBuf, Weak<T>>;


pub trait Resource {
    fn load(&Display, &Path) -> Self;
}


pub struct Manager<'a, T: Resource> {
    pub map: RefCell<Map<T>>,
    path: PathBuf,
    display: &'a Display,
    self_ref: RefCell<Vec<Rc<T>>>,
}


impl<'a, T: Resource> Manager<'a, T> {
    pub fn new<P: AsRef<Path>>(display: &'a Display, path: P) -> Manager<'a, T> {
        Manager {
            map: RefCell::new(Map::new()),
            path: path.as_ref().to_path_buf(),
            display: display,
            self_ref: RefCell::new(Vec::new()),
        }
    }

    fn load(&self, key: &PathBuf) -> Rc<T> {
        let mut map = self.map.borrow_mut();
        if map.contains_key(key) { let _ = map.remove(key); }
        let res = Rc::new(T::load(self.display, &key));
        map.insert(key.clone(), Rc::downgrade(&res));
        return res;
    }

    pub fn load_persistent(&mut self, key: &PathBuf) -> Rc<T> {
        let res = self.load(key);
        {
            let mut refs = self.self_ref.borrow_mut();
            refs.push(res.clone())
        }
        return res;
    }

    pub fn get<P: AsRef<Path>>(&self, p: P) -> Rc<T> {
        let key = self.path.join(p);
        let pointer = {
            let map = self.map.borrow();
            if map.contains_key(&key) {
                let weak = self.map.borrow()[&key].clone();
                weak.upgrade()
            }
            else { None }
        };
        match pointer {
            None => self.load(&key),
            Some (x) => x,
        }
    }


    pub fn release_persistent(&mut self) {
        self.self_ref.borrow_mut().clear();
    }
}


impl Resource for Texture {
    fn load(display: &Display, path: &Path) -> Texture {
        let image = image::open(path).unwrap();
        let image_dimensions = image.dimensions();
        let image = ::glium::texture::RawImage2d::from_raw_rgba_reversed(
            image.raw_pixels(), image_dimensions);
        Texture::new(display, image)
    }
}
