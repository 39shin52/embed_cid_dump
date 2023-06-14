use async_std::io;
use ipfs_embed::{DefaultParams, Block};
use libipld::{multihash::Code};
use libipld::cbor::DagCborCodec;
use std::fs::File;
use std::io::Read;

async fn file_open(path: &str) -> String {
    let path: String = path.lines().collect::<String>();
    println!("Input Path: {:?}", path);
    println!("--------------------");
    let mut f = File::open(path).expect("file not found");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("failed to read file");
    buf
}

fn hex(bytes: &[u8]) -> String {
    let mut string: String = String::new();
    for s in bytes {
        let str = &format!("{:02X}", s) as &str;
        string += str;
    }
    string
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--------------------");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).await.expect("failed to read line");
    let buf = file_open(&input).await;
    println!("original: {:?}", buf);

    let file_block =Block::<DefaultParams>::encode(DagCborCodec, Code::Sha2_256, &buf).unwrap();
    println!("--------------------");

    let file_cid = *file_block.cid();
    println!("file_cid: {:?}", file_cid);

    let data = file_block.data();
    println!("--------------------");

    let res = hex(data);
    println!("block: {:?}", res);
    println!("--------------------");

    Ok(())
}
