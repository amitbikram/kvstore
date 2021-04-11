use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    let mut database = Database::new().expect("Creating db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    match database.flush() {
        Ok(()) => println!("Yay!!"),
        Err(err) => println!("error {}", err),
    }
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Self, std::io::Error> {
        let mut map = HashMap::new();
        let content = std::fs::read_to_string("kv.db")?;
        for line in content.lines() {
            let mut item_iter = line.split("\t");
            let key = item_iter.next().expect("no key!");
            let value = item_iter.next().expect("no value!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Self {
            map,
        })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(self) -> std::io::Result<()> {
        do_flush(self)
    }
}

impl Drop for Database{
    fn drop(&mut self) {
        println!("i am here");
        todo!()
    }
}

fn do_flush(database: Database) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        let kvpair = format!("{}\t{}\n", key, value);
        contents.push_str(&kvpair);
    }
    std::fs::write("kv.db", contents)
}
