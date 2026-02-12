use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct AudioHandle {
    sink: Sink,
    _stream: OutputStream, // must keep alive
}

impl AudioHandle {
    pub fn new() -> Self {
        let stream = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(&stream.mixer());

        Self {
            sink,
            _stream: stream,
        }
    }

    pub fn load(&self, path: &PathBuf) {
        self.sink.stop();
        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        self.sink.append(source);
        self.sink.pause();
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn position(&self) -> std::time::Duration {
        self.sink.get_pos()
    }
}
