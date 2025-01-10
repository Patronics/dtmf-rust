use std::env;
use std::time::Duration;
use std::collections::HashMap;
use std::sync::LazyLock;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source, Amplify, TakeDuration, FadeIn, FadeOut};

type ToneGridMapType = HashMap<char, [i32; 2]>;

static TONE_GRID: LazyLock<ToneGridMapType> = LazyLock::new(|| {
	let mut tone_grid = HashMap::new();
	tone_grid.insert('1',[0,0]);
	tone_grid.insert('2',[0,1]);
	tone_grid.insert('3',[0,2]);
	tone_grid.insert('A',[0,3]);
	tone_grid.insert('4',[1,0]);
	tone_grid.insert('5',[1,1]);
	tone_grid.insert('6',[1,2]);
	tone_grid.insert('B',[1,3]);
	tone_grid.insert('7',[2,0]);
	tone_grid.insert('8',[2,1]);
	tone_grid.insert('9',[2,2]);
	tone_grid.insert('C',[2,3]);
	tone_grid.insert('*',[3,0]);
	tone_grid.insert('0',[3,1]);
	tone_grid.insert('#',[3,2]);
	tone_grid.insert('D',[3,3]);
	tone_grid
});

static LOW_TONES: [f32; 4] = [697.0, 770.0, 852.0, 941.0];
static HIGH_TONES: [f32; 4] = [1209.0, 1336.0, 1477.0, 1633.0];

const TONE_DURATION: f32 = 0.1;
const FADE_DURATION: f32 = 0.001;


fn main() {
	let args: Vec<String> = env::args().collect();
	
	// _stream must live as long as the sink
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let sink1 = Sink::try_new(&stream_handle).unwrap();
	let sink2 = Sink::try_new(&stream_handle).unwrap();
	sink1.pause();
	sink2.pause();
	
	if args.len() == 1 {
		eprintln!("usage: {} \"DTMFC0DE1234\"", args[0]);
		return;
	}
	
	gen_sequence(args[1].to_string(), &sink1, &sink2);
	// The sound plays in a separate thread. This call will block the current thread until the sink
	// has finished playing all its queued sounds.
	sink1.play();
	sink2.play();
	sink2.sleep_until_end();
}

fn gen_sequence(mut seq: String, sink1: &Sink, sink2: &Sink) {
	seq.make_ascii_uppercase();
	for c in seq.chars(){
		if TONE_GRID.contains_key(&c){
			sink1.append(new_tone(LOW_TONES[*TONE_GRID.get(&c).unwrap().get(0).unwrap() as usize]));
			sink2.append(new_tone(HIGH_TONES[*TONE_GRID.get(&c).unwrap().get(1).unwrap() as usize]));
				//sink1.play();
				//sink2.play();
				//sink2.sleep_until_end();
		} else {
			eprintln!("invalid char: {}", c)
		}
	}
}

fn new_tone(tone_freq: f32) -> FadeOut<FadeIn<Amplify<TakeDuration<SineWave>>>> {
	
	let tone = SineWave::new(tone_freq).take_duration(Duration::from_secs_f32(TONE_DURATION)).amplify(0.50).fade_in(Duration::from_secs_f32(FADE_DURATION));
	return tone.fade_out(Duration::from_secs_f32(TONE_DURATION-FADE_DURATION));
}