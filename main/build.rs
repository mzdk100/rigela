extern crate embed_resource;
fn main() { 
    embed_resource::compile("version.rc", embed_resource::NONE);
}