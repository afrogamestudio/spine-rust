extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod spine
{
    pub mod bone_data;
    pub mod blend_mode;
    pub mod slot_data;
    pub mod load_models;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
