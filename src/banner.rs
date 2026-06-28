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
    pub fn new() -> Self {
        Self {
            standard: FIGlet::standard().unwrap(),
            slant: FIGlet::slant().unwrap(),
            future: Toilet::future().unwrap(),
        }
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
            fonts: Fonts::new(),
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
