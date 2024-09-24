use oxdb_engine::doc::op::BlockFile;

fn main() {
    println!("Hello, world!");
    println!("blsz : {}", &BlockFile::gen_block_size());
}
