use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use chromaprint::Chromaprint;
use rodio::{Decoder, Source};

/// Parsing stage 2
/// for the information not found by local tags, we obtain the "chromaprint" of
/// the file and use the AcoustID database to match more information about the file.
pub fn parse_local_chromaprint(file: File) -> Option<(String, Duration)> {
    let bf: BufReader<File> = BufReader::new(file);
    let source = Decoder::new(bf).ok()?;
    let sample_rate: i32 = source.sample_rate() as i32; // cast to c_int
    let num_channels: i32 = source.channels() as i32; // cast to c_int
    let duration: Duration = source.total_duration()?;
    let sound_data: Vec<i16> = source.collect();
    byte_stream_chromaprint(&sound_data, sample_rate, num_channels).map(|s| (s, duration))
}

// this function looks pretty awkward because it's wrapping a C API
fn byte_stream_chromaprint(stream: &[i16], sample_rate: i32, num_channels: i32) -> Option<String> {
    let mut chromaprint: Chromaprint = Chromaprint::new();
    chromaprint.start(sample_rate, num_channels);
    chromaprint.feed(stream);
    chromaprint.finish();
    chromaprint.fingerprint()
}
