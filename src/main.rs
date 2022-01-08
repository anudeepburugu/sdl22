use sdl2::audio;
use std::io::prelude::*;

fn main(){
let init= sdl2::init().unwrap();
let aud= init.audio().unwrap();

let spec= sdl2::audio::AudioSpecDesired{
    channels:Some(1),
    freq:Some(44100),
    samples:None,
};
// sdl2::audio::AudioCallback
aud.open_playback(None,spec,|spec|{
let wav= sdl2::audio::AudioSpecWAV::load_wav(std::path::Path::new("")).unwrap();
let cvt= sdl2::audio::AudioCVT::new(wav.format, wav.channels, wav.freq,spec.format,spec.channels,spec.freq).unwrap();
let data= cvt.convert(wav.buffer().to_vec());
UnitStruct{
    data:data,
    pos:0,
    size:,
}

});
}

struct UnitStruct{
data:Vec<u8>,
pos:u8,
size:usize,
}
impl sdl2::audio::AudioCallback for UnitStruct{
    
type Channel=f32;
fn callback(&mut self, out:&mut [f32]){

}
}