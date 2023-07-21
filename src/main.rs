use std::fs::File;

mod env;

fn main() {

    let playerpos = File::open("/tmp/gamepos.bin").expect("Failed to find gamepos");
    let mut checkpoint_json = File::open("./src/checkpoints.json").unwrap();

    let trackenv = env::TrackEnv::new(playerpos,&mut checkpoint_json);

    loop {
        let pos = trackenv.get_player_pos();
        println!("{}", pos.to_str());
        
    }

    
}
