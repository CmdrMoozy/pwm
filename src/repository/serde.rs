use error::Result;
use repository::Repository;
use serde_json::{from_str, to_string_pretty};
use std::collections::HashMap;
use util::data::SensitiveData;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Contents {
    pub contents: HashMap<String, String>,
}

pub fn export(repository: &Repository) -> Result<Contents> {
    let mut contents: Contents = Contents { contents: HashMap::new() };

    for path in try!(repository.list(None)) {
        let plaintext: String = try!(repository.read_decrypt(&path)).encode();
        contents.contents.insert(try!(path.to_str()).to_owned(), plaintext);
    }

    Ok(contents)
}

pub fn export_serialize(repository: &Repository) -> Result<String> {
    Ok(try!(to_string_pretty(&try!(export(repository)))))
}

pub fn import(repository: &Repository, contents: Contents) -> Result<()> {
    for (path, plaintext) in contents.contents {
        let path = try!(repository.path(path));
        try!(repository.write_encrypt(&path, try!(SensitiveData::decode(plaintext))));
    }
    Ok(())
}

pub fn import_deserialize(repository: &Repository, s: &str) -> Result<()> {
    import(repository, try!(from_str(s)))
}
