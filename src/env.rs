use memmap2::Mmap;

mod checkpoint;

use std::{fs::File, io::Read};

pub struct TrackEnv{
    player_pos: Mmap,
    checkpoints: Vec<checkpoint::Checkpoint>,
    cur_checkpoint: usize
}

impl TrackEnv{
    pub fn new(player_pos: File, checkpoints_file: &mut File) -> TrackEnv {
        let mmap = unsafe { Mmap::map(&player_pos).unwrap() };
        let checkpoints = Self::clean_checkpoints(checkpoints_file, 2.5);

        TrackEnv{
            player_pos: mmap,
            checkpoints,
            cur_checkpoint: 0
        }
    }

    fn checkpoints_from_json(checkpoints_file: &mut File) -> Vec<checkpoint::Checkpoint>{
        let mut buffer = String::new();
        checkpoints_file.read_to_string(&mut buffer).expect("Failed to read string from Json file");
        let checkpoints: Vec<checkpoint::Checkpoint> = serde_json::from_str(buffer.as_str()).expect("Failed to desteralyie the json file????");

        return checkpoints
    }

    fn clean_checkpoints(checkpoints_file: &mut File, min_distance: f32) -> Vec<checkpoint::Checkpoint>{
        let mut checkpoints = Self::checkpoints_from_json(checkpoints_file);

        let mut i = 0;
        while i < checkpoints.len()-1 {
            if checkpoints[i].distance_to(&checkpoints[i+1]) < min_distance{
                checkpoints.remove(i+1);
                continue;
            }
            i+=1;
        }

        return checkpoints;

    }

    pub fn get_player_pos(&self) -> checkpoint::Checkpoint{
        let data = self.player_pos.as_ref();

        return checkpoint::Checkpoint::from_bytes(data);
    }

    fn update_cur_checkpoint(self){
        let mut i = self.cur_checkpoint+1;
        let pos = self.get_player_pos();
        let last_distace = pos.distance_to(&self.checkpoints[self.cur_checkpoint]);

        while i < self.checkpoints.len(){



        }

    }



}
