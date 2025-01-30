use std::rc::Rc;

#[derive(Debug)]
struct FileName {
    name: String,
    ext: String,
}

#[derive(Debug)]
struct FileNameRc {
    name: Rc<String>,
    ext: Rc<String>,
}

fn use_clone() {
    let name = String::from("hoho");
    let ext = String::from("hihi");

    for _ in 0..3 {
        println!("{:?}", FileName {
            name: name.clone(),
            ext: ext.clone()
        })
    }
}

fn ref_counter() {
    let name = Rc::new(String::from("main"));
    let ext = Rc::new(String::from("rs"));

    for _ in 0..3 {
        println!("{:?}", FileNameRc {
            name: name.clone(),
            ext: ext.clone()
        });
    }
}

fn main() {
    use_clone();
    ref_counter();
}
