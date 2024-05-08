use std::arch::x86_64::_mulx_u32;
use std::collections::VecDeque;
use std::{fs, vec};
use std::time::SystemTime;

#[derive(Debug)]
struct File {
    name: String,
    modified: SystemTime,
    content: Vec<u8>,
}

#[derive(Debug)]
struct Dir {
    name: String,
    modified: SystemTime,
    children: Vec<Node>,
}

// Define this enum in order to be able to store different types in the same vector
#[derive(Debug)]
enum Node {
    File(File),
    Dir(Dir),
}

impl Node{
    fn name(&self)->String{
        match self{
            Node::File(f) => {return f.name.to_string()},
            Node::Dir(d) => {return d.name.to_string()}
        }
    }
}

#[derive(Debug)]
enum FSError {
    NotFound,     // file or dir not found
    NotADir,      // when trying to ad children to a file
    Duplicate,    // duplicate name in dir
    DirNotEmpty,  // try to remove a dir with children
    GenericError, // generic error
}

// define lifetimes
struct MatchResult<'a, 'b> {
    q: &'a str, // matched query string
    path: String, // matched path
    node: &'b Node, // matched node
}

struct Filesystem {
    root: Node,
}

impl Filesystem {
    // create a new empty filesystem with a root dir
    // (name of the root dir is empty string: "")
    pub fn new() -> Self {
        Self{
            root: Node::Dir(Dir{
                name: "C:/Users/gabri/Desktop/Lab3".to_string(),
                modified: SystemTime::now(),
                children: vec![]
            })
        }
    }

    // create a new filesystem reading from disk all the structure under the given path
    // in the file content just write the firt 1k bytes of the file
    // return the root node of the filesystem
    // (implement this function at the end, after all the other methods, the only purpose is to take a look std::fs functions, use std::fs:read_dir)
    pub fn from(path: &str) -> Self {
        unimplemented!()
    }

    // create a new directory in the filesystem under the given path
    // return a reference the created dir
    // possible errors: NotFound, path NotADir, Duplicate
    pub fn mkdir<'a>(&mut self, path: &str, name: &str) -> Result<&mut Dir, FSError> {
        //Check duplicate
        match self.get_mut(format!("{path}{name}").as_str()){
            Ok(_) => {return Err(FSError::Duplicate)}
            Err(_) => {}
        }



        let root_str = String::from(self.root.name().as_str());
        let father_node = self.get_mut(path)?;
        let mut correct_path: String = String::from(path);
        if path.chars().last().unwrap() != '/' {
            correct_path.push('/');
        }
        let absolute_path = format!("{root_str}{correct_path}{name}");
        match fs::create_dir(absolute_path){
            Ok(_) => {
                match father_node {
                    Node::File(_) => {Err(FSError::NotADir)}
                    Node::Dir(father_dir) => {
                        let final_dir = Node::Dir(Dir{
                            name: name.to_string(),
                            modified: SystemTime::now(),
                            children: vec![]
                        });
                        father_dir.children.push(final_dir);
                        let mut correct_path: String = String::from(path);
                        if path.chars().last().unwrap() != '/' {
                            correct_path.push('/');
                        }
                        match self.get_mut(format!("{correct_path}{name}").as_str())?{
                            Node::File(_) => {Err(FSError::NotADir)}
                            Node::Dir(mutdir) => {return Ok(mutdir)}
                        }
                    }
                }
            }
            Err(_) => {Err(FSError::GenericError)}
        }

    }

    // possible errors: NotFound, path is NotADir, Duplicate
    pub fn create_file(&mut self, path: &str, name: &str) -> Result<&mut File, FSError> {
        let root_str = String::from(self.root.name().as_str());
        let mut correct_path = String::from(path);
        if path.chars().last().unwrap() != '/' { correct_path.push('/'); }
        let absolute_path = format!("{root_str}{correct_path}{name}");
        match fs::File::create_new(absolute_path){
            Ok(_) => {
                let mut correct_path = String::from(path);
                if path.chars().last().unwrap() == '/'  && path.len() > 1 { correct_path.pop(); }
                //Ora trovo il padre e lo aggiungo
                match self.get_mut(correct_path.as_str()){
                    Ok(father_node) => {
                        match father_node{
                            Node::File(_) => {Err(FSError::NotADir)}
                            Node::Dir(father_dir) => {
                                let file = Node::File(File {
                                    name: name.to_string(),
                                    modified: SystemTime::now(),
                                    content: vec![],
                                });
                                father_dir.children.push(file);
                                let mut correct_path = String::from(path);
                                if path.chars().last().unwrap() != '/'  && path.len() > 1 { correct_path.push('/'); }
                                match self.get_mut(format!("{correct_path}{name}").as_str())?{
                                    Node::File(file) => {Ok(file)}
                                    Node::Dir(_) => {Err(FSError::NotADir)}
                                }
                            }
                        }
                    }
                    Err(_) => {Err(FSError::NotFound)}
                }
            }
            Err(_) => {return Err(FSError::GenericError)}
        }
    }

    // updated modification time of the file or the dir
    // possible errors: NotFound
    pub fn touch(&mut self, path: &str) -> Result<(), FSError> {
        unimplemented!()
    }

    // remove a node from the filesystem and return it
    // if it's a dir, it must be empty
    // possible errors: NotFound, DirNotEmpty
    pub fn delete(&mut self, path: &str) -> Result<Node, FSError> {
        unimplemented!()
    }

    // get a reference to a node in the filesystem, given the path
    pub fn get(&mut self, path: &str) -> Result<&Node, FSError> {
        if path == "/" { return Ok(&self.root)}

        let mut path_nodes: VecDeque<&str> = path.split("/").collect();
        let mut current_node = &self.root;

        while let Some(p) = path_nodes.pop_front(){
            if path_nodes.len() == 0 {
                if current_node.name() == p{
                    return Ok(current_node)
                }
                else {return Err(FSError::NotFound)}
            }

            match current_node {
                Node::File(_) => {return Err(FSError::NotADir)}
                Node::Dir(d) => {
                    let pos = d.children.iter().position(|n| n.name() == path_nodes[0]);
                    // now we can get it as mutable
                    if let Some(i) = pos {
                        current_node = d.children.get(i).unwrap();
                    } else {
                        return Err(FSError::NotFound);
                    }
                }
            }

        }
        Err(FSError::GenericError)
    }

    // get a mutable reference to a node in the filesystem, given the path
    pub fn get_mut(&mut self, path: &str) -> Result<&mut Node, FSError> {

        if path == "/" {
            return  Ok(&mut self.root);
        }

        let mut parts = path.split("/").collect::<VecDeque<&str>>();
        let mut cnode = &mut self.root;

        while let Some(part) = parts.pop_front() {
            if parts.len() == 0 {
                if cnode.name() == part {
                    return Ok(cnode);
                } else {
                    return Err(FSError::NotFound);
                }
            }

            match cnode {
                Node::Dir(d) => {

                    let pos = d.children.iter().position(|n| n.name() == parts[0]);
                    // now we can get it as mutable
                    if let Some(i) = pos {
                        cnode = d.children.get_mut(i).unwrap();
                    } else {
                        return Err(FSError::NotFound);
                    }
                }
                Node::File(_) => {
                    return Err(FSError::NotADir);
                }
            }
        }
        Err(FSError::GenericError)
    }

    // search for a list of paths in the filesystem
    // qs is a list query strings with constraints
    // the constraints must be matched in or (it's returned any node matching at least one constraint)
    // constraint format: "type:pattern"
    // constraints:
    // - "type:dir" -> match only directories
    // - "type:file" -> match only files
    // - "name:value" -> match only nodes with the given name
    // - "partname:value" -> match only nodes with the given string in the name

    pub fn find<'a>(&'a self, qs: &[&'a str]) -> Vec<MatchResult> {
        let mut res:Vec<MatchResult>= Vec::new();
        let mut visit = VecDeque::from([&self.root]);
        while let Some(n) = visit.pop_front() {
            //Per ogni filtro
            for q in qs {
                if Filesystem::do_match(q, n) {
                    let m = MatchResult {
                        q,
                        path: "".to_string(),
                        node: &n,
                    };
                    res.push(m);
                }
            }
            match n {
                Node::File(_) => {}
                Node::Dir(dir) => {
                    for c in &dir.children {visit.push_back(c);}
                }
            }
        }
        res
    }

    fn do_match(qs: &str, n: &Node) -> bool {
        let parts: Vec<&str> = qs.split(":").collect();
        match parts[0]{
            "type" => {
                match n {
                    Node::File(_)       =>      {return parts[1] == "file"}
                    Node::Dir(_)        =>      {return parts[1] == "dir"}
                }
            },
            "name" => {
                    return n.name()     ==      parts[1]
            },
            "partname" => {
                    return n.name().contains(parts[1])
            },
            _ => {return false}
        }
    }

    // walk the filesystem, starting from the root, and call the closure for each node with its path
    // the first parameter of the closure is the path of the node, second is the node itself
    pub fn walk(&self, f: impl Fn(&str, &Node)) {
        let mut visit = VecDeque::from([(String::from(""), &self.root)]);
        while let Some((path, node)) = visit.pop_front() {
            f(&path, node);

            match node {
                Node::Dir(dir) => {
                    for c in &dir.children {
                        visit.push_back((format!("{}/{}", path, c.name()), c));
                    }
                },
                _ => {}
            }
        }
    }
}

#[test]
fn demo() {

    let mut fs = Filesystem::new();

    // create a directory structure, 10 dirs with a child dir and file each one
    for i in 0..10 {
        fs.mkdir("/", format!("dir{}", i).as_str()).unwrap();
        fs.mkdir(format!("/dir{}", i).as_str(), "child1").unwrap();
        fs.create_file(format!("/dir{}", i).as_str(), "file1").unwrap();
    }

    println!("find /child2");
    if let Ok(res) = fs.get_mut("/dir2/child1") {
        match res {
            Node::Dir(d) => {
                d.name = "dir2 found".to_string();
            }
            // try to match all possible errros
            _ => {}
        }
    } else {
        println!("not found");
    }

    // let's try with matches
    let matches = fs.find(&["name:child1", "type:file"]);
    for m in matches {
        match m.node {
            Node::File(f) => {
                println!("{:?}", f);
            },
            Node::Dir(d) => {
                // inspect children
                println!("{:?}", d);
            },
            _ => {}
        }
    }

    // see note "riferimenti mutabili" in exercise text
    // now let's try to modify the filesystem using the found matches
    // is it possible to do it? which error do you get from the compiler?
    let matches = fs.find(&["/dir2/child1", "/dir3/child1"]);
    // for m in matches {
    //     let node = fs.get_mut(m.path.as_str()).unwrap();
    //     match node {
    //         Node::File(f) => {
    //             // inspect content
    //         }
    //         _ => {}
    //     }
    // }

    // how can you fix the previous code?
    // suggestion: this code using paths which are not referenced by MatchResults should compile. Why?
    // Therefore how can you use the paths returned in the MatchResults to modify the filesystem?
    let paths = ["/dir1/child1", "/dir2/child1", "/dir3/child1"];
    for p in paths {
        let n = fs.get_mut(p);
    }


    // now let's try to walk the filesystem
    fs.walk(|path, node| {
        match node {
            Node::File(f) => {
                println!("file: {}", path);
            }
            Node::Dir(d) => {
                println!("dir: {}", path);
            }
        }
    });

}

