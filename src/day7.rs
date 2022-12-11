use std::collections::HashMap;
use std::mem;

#[derive(Debug, Clone)]
pub struct FileTree {
    paths: HashMap<String, usize>,
    inner: Vec<FileTreeNode>,
    head: Option<usize>,
}

impl Default for FileTree {
    fn default() -> Self {
        let mut hash = HashMap::default();
        hash.insert("/".to_string(), 0);
        let fo = FileTreeNode::new(FileObject::Dir(Directory::new("/")), None, 0, "".to_string());
        Self {
            paths: hash,
            inner: vec![fo],
            head: Some(0),
        }
    }
}

impl FileTree {
    pub fn add_file(&mut self, fo: FileObject) {
        let name = match &fo {
            FileObject::Dir(d) => &(*d.name),
            FileObject::File(f) => &(*f.name),
        };
        let path = self.cur().path.clone() + name;
        let fo_ptr = self.inner.len();
        self.paths.insert(path.clone(), fo_ptr);
        self.inner.push(FileTreeNode::new(fo, self.head, fo_ptr, path));
        self.cur_mut().add_child(fo_ptr);
    }

    pub fn move_dir(&mut self, name: &'static str) {
        let path = self.cur().path.clone() + name;
        dbg!(&path);
        self.head = Some(
            *self
                .paths
                .get(&path)
                .expect("OOPS Moving to a Dir that hasn't been added yet"),
        );
    }

    pub fn move_root(&mut self) {
        self.head = Some(0)
    }

    pub fn move_to_parent(&mut self) {
        let cur = &self.inner[self.head.unwrap()];
        let parent_node = &self.inner[cur.parent().unwrap()];
        self.head = Some(parent_node.self_ptr);
    }

    fn cur(&self) -> &FileTreeNode {
        &self.inner[self.head.unwrap()]
    }

    fn cur_mut(&mut self) -> &mut FileTreeNode {
        &mut self.inner[self.head.unwrap()]
    }

    pub fn size(&self) -> u32 {
        self.cur().size(&self.inner)
    }

    pub fn sizes(&self) -> Vec<(&'static str, u32)> {
        let mut res = vec![];
        for node in self.inner.iter() {
            if let FileObject::Dir(n) = &node.item {
                res.push((n.name, node.size(&self.inner)))
            }
        }
        res
    }
}

/// A Node on the Tree that allows us to go up and down the tree
#[derive(Debug, Clone)]
struct FileTreeNode {
    item: FileObject,
    children: Vec<usize>,
    parent: Option<usize>,
    self_ptr: usize,
    path: String,
}

impl FileTreeNode {
    pub(crate) fn new(
        item: FileObject,
        parent: Option<usize>,
        self_ptr: usize,
        parent_path: String,
    ) -> Self {
        let name = match &item {
            FileObject::Dir(d) => &(*d.name),
            FileObject::File(f) => &(*f.name),
        };
        Self {
            item,
            children: vec![],
            parent,
            self_ptr,
            path: parent_path + &name.to_string(),
        }
    }

    fn add_child(&mut self, i_ptr: usize) {
        self.children.push(i_ptr);
    }

    fn parent(&self) -> Option<usize> {
        self.parent
    }

    fn name(&self) -> &'static str {
        match &self.item {
            FileObject::Dir(d) => &(*d.name),
            FileObject::File(f) => &(*f.name),
        }
    }

    fn size(&self, child_vec: &Vec<FileTreeNode>) -> u32 {
        match &self.item {
            FileObject::Dir(d) => self
                .children
                .iter()
                .map(|&i| child_vec[i].size(child_vec))
                .sum(),
            FileObject::File(f) => f.size,
        }
    }
}

pub trait HasSize {
    fn size(self) -> u32;
}

#[derive(Debug, Clone)]
pub enum FileObject {
    Dir(Directory),
    File(File),
}

#[derive(Debug, Clone)]
pub struct Directory {
    name: &'static str,
    children: Vec<FileObject>,
}

impl Directory {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            children: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    name: &'static str,
    size: u32,
}

impl File {
    pub fn new(name: &'static str, size: u32) -> Self {
        Self { name, size }
    }
}

impl HasSize for File {
    fn size(self) -> u32 {
        self.size
    }
}
