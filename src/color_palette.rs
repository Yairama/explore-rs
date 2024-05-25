use egui::Color32;

#[derive(Debug)]
pub struct ColorPalette {
    pub colors: Vec<Color32>,
}

#[allow(dead_code)]
impl ColorPalette {
    fn new(colors: Vec<Color32>) -> Self {
        Self { colors }
    }

    pub fn deep() -> Self {
        Self::new(vec![
            Color32::from_rgb(0x4E, 0x79, 0xA7), // Azul
            Color32::from_rgb(0xF2, 0xB8, 0x70), // Naranja
            Color32::from_rgb(0x59, 0xA1, 0x66), // Verde
            Color32::from_rgb(0xE1, 0x68, 0xA7), // Rosa
            Color32::from_rgb(0xF7, 0xE5, 0xB8), // Amarillo
            Color32::from_rgb(0xAB, 0xA1, 0xA0), // Gris
            Color32::from_rgb(0xF1, 0xCE, 0x79), // Amarillo claro
            Color32::from_rgb(0x84, 0x84, 0x84), // Gris oscuro
            Color32::from_rgb(0xB2, 0x7E, 0x9A), // Magenta
            Color32::from_rgb(0xA4, 0xC7, 0xD3), // Celeste
        ])
    }

    pub fn muted() -> Self {
        Self::new(vec![
            Color32::from_rgb(0xCC, 0x79, 0xA7), // Rosa oscuro
            Color32::from_rgb(0xF3, 0xAE, 0x70), // Naranja claro
            Color32::from_rgb(0x8C, 0xD3, 0x68), // Verde claro
            Color32::from_rgb(0xE1, 0x98, 0xF3), // Lila
            Color32::from_rgb(0xF3, 0xE8, 0xB8), // Amarillo pálido
            Color32::from_rgb(0xAB, 0xA1, 0xA0), // Gris medio
            Color32::from_rgb(0xF1, 0xCE, 0xB8), // Amarillo
            Color32::from_rgb(0x84, 0x84, 0x84), // Gris oscuro
            Color32::from_rgb(0xB2, 0x7E, 0x9A), // Magenta
            Color32::from_rgb(0xA4, 0xC7, 0xD3), // Celeste
        ])
    }

    fn pastel() -> Self {
        Self::new(vec![
            Color32::from_rgb(0x8D, 0xA0, 0xCB), // Azul pastel
            Color32::from_rgb(0xFF, 0xA0, 0xA0), // Rojo pastel
            Color32::from_rgb(0xA4, 0xD3, 0xB3), // Verde pastel
            Color32::from_rgb(0xF4, 0xD4, 0xA4), // Amarillo pastel
            Color32::from_rgb(0xD4, 0xA4, 0xD4), // Rosa pastel
            Color32::from_rgb(0xF4, 0xA4, 0xA4), // Naranja pastel
            Color32::from_rgb(0xA4, 0xF4, 0xD4), // Verde claro pastel
            Color32::from_rgb(0xD4, 0xD4, 0xF4), // Lila pastel
            Color32::from_rgb(0xF4, 0xF4, 0xA4), // Amarillo pálido pastel
            Color32::from_rgb(0xA4, 0xA4, 0xF4), // Azul claro pastel
        ])
    }

    // Puedes agregar más paletas de colores según tus necesidades
}