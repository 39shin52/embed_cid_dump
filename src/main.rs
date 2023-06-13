use async_std::stream::StreamExt;
use ipfs_embed::{Config, DefaultParams, Ipfs, Block};
use libipld::{multihash::Code};
use libipld::cbor::DagCborCodec;
use libipld::DagCbor;

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
struct Identity {
    id: u64,
    name: String,
    age: u8,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ipfs = Ipfs::<DefaultParams>::new(Config::default()).await?;
    ipfs.listen_on("/ip4/127.0.0.1/tcp/4001".parse()?).next().await.unwrap();

    let identity = Identity {
        id: 0,
        name: "David Craven".into(),
        age: 26,
    };

    let str = String::from("hello world");

    // block作成
    let identity_block = Block::<DefaultParams>::encode(DagCborCodec, Code::Sha2_256,&identity).unwrap();
    println!("insert identity block: {:?}", identity_block);
    // blockからCIDを取得
    let identity_cid = *identity_block.cid();
    println!("identity_cid: {:?}", identity_cid);
    // ipfs addする
    ipfs.insert(identity_block)?;

    let str_block = Block::<DefaultParams>::encode(DagCborCodec, Code::Sha2_256, &str).unwrap();
    println!("insert string block: {:?}", str_block);
    let str_cid = *str_block.cid();
    println!("str_cid: {:?}", str_cid);
    ipfs.insert(str_block)?;

    // ipfs getする(CIDは29行目で取得しておいたもの)
    let get_identity_block = ipfs.get(&identity_cid)?;
    println!("get_identity_block: {:?}", get_identity_block);
    // 取得したblockをデコード
    let identity_rep:Identity = get_identity_block.decode()?;
    println!("identity: {:?}", identity_rep);
    // addしたものとgetしたものを比較
    assert_eq!(identity,identity_rep);

    let get_str_block = ipfs.get(&str_cid)?;
    println!("get_str_block: {:?}", get_str_block.data());

    // println!("identity.name: {}, block: {}, cid: {}", identity.name, block.to_string(), cid);
    Ok(())
}
