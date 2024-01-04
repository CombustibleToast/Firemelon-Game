use core::fmt::Write;

use agb::{
    display::{object::{PaletteVram, ObjectTextRender, Size, OamIterator}, Font, palette16::Palette16},
    include_font, println
};
use alloc::string::ToString;

const FONT: Font = include_font!("graphics/BrunoAce-Regular.ttf", 12); //12?
const PALETTE: [u16; 16] = [0xFF_FF, 0xFF_FF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,];

pub struct Writer<'a>{
    text_renderer: ObjectTextRender<'a>
}

pub fn create_writer() -> Writer<'static>{
    let palette = Palette16::new(PALETTE);
    let palette = PaletteVram::new(&palette).unwrap();
    Writer{
        text_renderer: ObjectTextRender::new(&FONT, Size::S16x16, palette)
    }
}

impl Writer<'_>{
    pub fn frame(&mut self, oam: &mut OamIterator){
        self.text_renderer.next_line();
        self.text_renderer.update((0,0).into()); //"Should be called in the same frame as and after next_line and pop_line."
        self.text_renderer.commit(oam);
    }

    pub fn write_new_score(&mut self, new_score: &i32, oam: &mut OamIterator){
        self.text_renderer.pop_line();
        match self.text_renderer.write_str(&new_score.to_string()){
            Ok(done) => {},
            Err(error) => {println!("FAILED TO WRITE SCORE {} WITH ERROR {}", new_score, error)}
        }
        self.frame(oam);
    }
}