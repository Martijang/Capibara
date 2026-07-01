use figlet_rs::{FIGlet, FIGure, Toilet};
use rand::{RngExt, rngs::ThreadRng};

#[derive(Debug)]
struct Fonts {
    standard: FIGlet,
    slant: FIGlet,
    future: Toilet,
}

#[derive(Debug)]
pub struct BannerMaker {
    fonts: Fonts,
    rng: ThreadRng,
}

impl Fonts {
    //I assume that this function will never fail(for now)
    //TODO: Fix this to handle error properly
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            standard: FIGlet::standard()?,
            slant: FIGlet::slant()?,
            future: Toilet::future()?,
        })
    }

    pub fn standard_font(&self) -> FIGure<'_> {
        self.standard.convert("Capi !! bara").unwrap()
    }
    pub fn slant_font(&self) -> FIGure<'_> {
        self.slant.convert("Capibara").unwrap()
    }
    pub fn future_font(&self) -> FIGure<'_> {
        self.future
            .convert("I am Capibara not Capybara :]")
            .unwrap()
    }
}

impl BannerMaker {
    pub fn new() -> Self {
        Self {
            fonts: Fonts::new().unwrap(),
            rng: rand::rng(),
        }
    }

    pub fn print_banner(&mut self) {
        let index = self.random_index();
        match index {
            1 => println!("{}", self.fonts.standard_font()),
            2 => println!("{}", self.fonts.slant_font()),
            3 => println!("{}", self.fonts.future_font()),
            _ => println!("{}", self.fonts.slant_font()),
        }
    }

    fn random_index(&mut self) -> u32 {
        self.rng.random_range(1..=3)
    }
}

#[cfg(test)]
mod test{


use crate::banner::*;

    #[test]
    fn fonts_inited_successfuly(){
        let fonts = Fonts::new();
        assert!(fonts.is_ok())
    }

    #[test]
    fn try_convert_without_panicing(){
        let fonts = Fonts::new().unwrap();

        //try running all these and hoping that it won't gonna panic
        let _ = fonts.slant_font();
        let _ = fonts.standard_font();
        let _ = fonts.future_font();
    }
}
