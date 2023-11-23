use agb::{
    include_wav,
    rng::gen, 
    sound::mixer::{SoundChannel, Frequency, Mixer, MixerController, ChannelId}, 
     println,
    fixnum::num,
};


//const CHASER: &[u8] = include_wav!("sounds/music/CHASER.wav");
// const KATAMARI: &[u8] = include_wav!("sounds/music/KATAMARI.wav");
const BGM: [&[u8]; 0] = [];
// const BGM: [&[u8]; 6] = [
//     include_wav!("sounds/music/KATAMARI.wav"),
//     include_wav!("sounds/music/ACT RIGHT.wav"),
//     include_wav!("sounds/music/GIRL HELL 1999.wav"),
//     include_wav!("sounds/music/MURDER EVERY 1 U KNOW!.wav"),
//     include_wav!("sounds/music/P3T.wav"),
//     include_wav!("sounds/music/PUSH UR T3MPRR.wav")
// ];

pub struct SoundPlayer<'a>{
    mixer: Mixer<'a>,
    current_playing_channel: Option<ChannelId>
}

pub fn start_bgm(mixer: Mixer) -> SoundPlayer {
    let mut player = SoundPlayer{
        mixer: mixer,
        current_playing_channel: None
    };
    player.mixer.enable();
    return player;
}

impl SoundPlayer<'_>{
    pub fn frame(&mut self){
        self.mixer.frame();

        //if there is no currently playing channel or the current one has ended, play a new one
        if self.current_playing_channel.is_none() || self.mixer.channel(&self.current_playing_channel.as_mut().unwrap()).is_none() {
            self.play_random_song()
        }
    }

    pub fn play_random_song(&mut self) {
        //Don't do anything if there are no songs loaded (omitted during development)
        if BGM.len() == 0{
            return;
        }

        //Stop the currently playing song
        if self.current_playing_channel.is_some() && self.mixer.channel(&self.current_playing_channel.as_mut().unwrap()).is_some() {
            self.mixer.channel(&self.current_playing_channel.as_mut().unwrap()).unwrap().stop();
        }

        //Select a new song and play
        let random_index: usize = (gen() % BGM.len() as i32).abs() as usize;
        let mut new_channel =  SoundChannel::new_high_priority(BGM[random_index]);
        new_channel.playback(num!(2.05));
        self.current_playing_channel = self.mixer.play_sound(new_channel);
    }
}