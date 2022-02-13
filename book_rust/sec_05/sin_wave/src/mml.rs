use std::result;

use super::mics::write_tone;
use hound::WavWriter;

const SAMPLE_RATE: f32 = 44100.0;

#[derive(Debug)]
pub struct Note {
    pub no: i32,
    pub len: i32,
}

pub fn write(filename: &str, notes: Vec<Note>, bpm: f32) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut fw = WavWriter::create(filename, spec).unwrap();
    for note in notes.into_iter() {
        let len = (4.0 / note.len as f32 * (60.0 / bpm) * SAMPLE_RATE as f32) as u32;
        let tone = if note.no >= 0 {
            440.0 * 2.0f32.powf((note.no - 69) as f32 / 12.0)
        } else {
            0.0
        };
        write_tone(&mut fw, tone, len)
    }
}

pub fn parse(src: String) -> Vec<Note> {
    let mut result = vec![];
    let mut octave = 5;
    let mut length = 4;
    let mut it = src.chars();
    while let Some(ch) = it.next() {
        match ch {
            'a'..='g' => {
                let note = match ch {
                    'c' => 0,
                    'd' => 2,
                    'e' => 4,
                    'f' => 5,
                    'g' => 7,
                    'a' => 9,
                    'b' => 11,
                    _ => 0,
                };
                let no = note + octave * 12;
                result.push(Note { no, len: length });
            }
            'r' => result.push(Note {
                no: -1,
                len: length,
            }),
            'o' => {
                let v = it.next().expect("oの後は数値");
                let o = v as i32 - '0' as i32;
                if o >= 0 && o < 9 {
                    octave = o;
                }
            }
            'l' => {
                let v = it.next().expect("lの後は数値");
                let l = v as i32 - '0' as i32;
                if l >= 1 && l <= 9 {
                    length = l;
                }
            }
            _ => {}
        };
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_test() {
        let notes: Vec<Note> = vec![
            Note { no: 60, len: 4 },
            Note { no: 62, len: 4 },
            Note { no: 64, len: 4 },
        ];
        write("test.wav", notes, 120.0)
    }

    #[test]
    fn parse_test() {
        let mml = "l2 o5 cde".to_string();
        let notes = parse(mml);
        assert_eq!(notes[0].no, 60);
        assert_eq!(notes[0].len, 2);
        assert_eq!(notes[1].no, 62);
        assert_eq!(notes[2].no, 64);
    }
}
