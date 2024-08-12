pub struct ColorNames(&'static str, u8, u8, u8);

#[derive(Debug, Copy, Clone)]
pub enum TurtleColor {
    CurrentColor,
    Color(f32, f32, f32),
}

impl Default for TurtleColor {
    fn default() -> Self {
        Self::Color(0., 0., 0.)
    }
}

impl From<[f32; 4]> for TurtleColor {
    fn from(color: [f32; 4]) -> TurtleColor {
        TurtleColor::Color(color[0], color[1], color[2])
    }
}

impl From<TurtleColor> for [f32; 4] {
    fn from(color: TurtleColor) -> [f32; 4] {
        match color {
            TurtleColor::CurrentColor => todo!(),
            TurtleColor::Color(r, g, b) => [r, g, b, 1.],
        }
    }
}

//TODO: use tryfrom instead?
impl From<&TurtleColor> for iced::Color {
    fn from(value: &TurtleColor) -> Self {
        if let TurtleColor::Color(r, g, b) = value {
            iced::Color {
                r: *r,
                g: *g,
                b: *b,
                a: 1.,
            }
        } else {
            todo!()
        }
    }
}

impl From<&str> for TurtleColor {
    fn from(color_name: &str) -> Self {
        if color_name.starts_with('#') {
            // #rrggbbaa
            let value = u32::from_str_radix(&color_name[1..=6], 16)
                .unwrap_or_else(|_| panic!("Could not parse {color_name} as a hex string"));
            let r = (value >> 16) & 0xff;
            let g = (value >> 8) & 0xff;
            let b = value & 0xff;
            return TurtleColor::Color(r as f32 / 255., g as f32 / 255., b as f32 / 255.);
        }
        for c in &COLOR {
            if color_name == c.0 {
                return TurtleColor::Color(c.1 as f32 / 255., c.2 as f32 / 255., c.3 as f32 / 255.);
            }
        }

        TurtleColor::CurrentColor
    }
}

impl From<(f64, f64, f64)> for TurtleColor {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        fn in_range(v: f64) -> bool {
            (0. ..=1.).contains(&v)
        }

        if in_range(r) && in_range(g) && in_range(b) {
            TurtleColor::Color(r as f32, g as f32, b as f32)
        } else {
            TurtleColor::CurrentColor
        }
    }
}

impl From<(f32, f32, f32)> for TurtleColor {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        fn in_range(v: f32) -> bool {
            (0. ..=1.).contains(&v)
        }

        if in_range(r) && in_range(g) && in_range(b) {
            TurtleColor::Color(r, g, b)
        } else {
            TurtleColor::CurrentColor
        }
    }
}

impl From<(u8, u8, u8)> for TurtleColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        TurtleColor::Color(r as f32 / 255., g as f32 / 255., b as f32 / 255.)
    }
}

// Color names came from the page at https://www.tcl.tk/man/tcl/TkCmd/colors.htm
pub const COLOR: [ColorNames; 760] = [
    ColorNames("alice blue", 240, 248, 255),
    ColorNames("AliceBlue", 240, 248, 255),
    ColorNames("antique white", 250, 235, 215),
    ColorNames("AntiqueWhite", 250, 235, 215),
    ColorNames("AntiqueWhite1", 255, 239, 219),
    ColorNames("AntiqueWhite2", 238, 223, 204),
    ColorNames("AntiqueWhite3", 205, 192, 176),
    ColorNames("AntiqueWhite4", 139, 131, 120),
    ColorNames("agua", 0, 255, 255),
    ColorNames("aquamarine", 127, 255, 212),
    ColorNames("aquamarine1", 127, 255, 212),
    ColorNames("aquamarine2", 118, 238, 198),
    ColorNames("aquamarine3", 102, 205, 170),
    ColorNames("aquamarine4", 69, 139, 116),
    ColorNames("azure", 240, 255, 255),
    ColorNames("azure1", 240, 255, 255),
    ColorNames("azure2", 224, 238, 238),
    ColorNames("azure3", 193, 205, 205),
    ColorNames("azure4", 131, 139, 139),
    ColorNames("beige", 245, 245, 220),
    ColorNames("bisque", 255, 228, 196),
    ColorNames("bisque1", 255, 228, 196),
    ColorNames("bisque2", 238, 213, 183),
    ColorNames("bisque3", 205, 183, 158),
    ColorNames("bisque4", 139, 125, 107),
    ColorNames("black", 0, 0, 0),
    ColorNames("blanched almond", 255, 235, 205),
    ColorNames("BlanchedAlmond", 255, 235, 205),
    ColorNames("blue", 0, 0, 255),
    ColorNames("blue violet", 138, 43, 226),
    ColorNames("blue1", 0, 0, 255),
    ColorNames("blue2", 0, 0, 238),
    ColorNames("blue3", 0, 0, 205),
    ColorNames("blue4", 0, 0, 139),
    ColorNames("BlueViolet", 138, 43, 226),
    ColorNames("brown", 165, 42, 42),
    ColorNames("brown1", 255, 64, 64),
    ColorNames("brown2", 238, 59, 59),
    ColorNames("brown3", 205, 51, 51),
    ColorNames("brown4", 139, 35, 35),
    ColorNames("burlywood", 222, 184, 135),
    ColorNames("burlywood1", 255, 211, 155),
    ColorNames("burlywood2", 238, 197, 145),
    ColorNames("burlywood3", 205, 170, 125),
    ColorNames("burlywood4", 139, 115, 85),
    ColorNames("cadet blue", 95, 158, 160),
    ColorNames("CadetBlue", 95, 158, 160),
    ColorNames("CadetBlue1", 152, 245, 255),
    ColorNames("CadetBlue2", 142, 229, 238),
    ColorNames("CadetBlue3", 122, 197, 205),
    ColorNames("CadetBlue4", 83, 134, 139),
    ColorNames("chartreuse", 127, 255, 0),
    ColorNames("chartreuse1", 127, 255, 0),
    ColorNames("chartreuse2", 118, 238, 0),
    ColorNames("chartreuse3", 102, 205, 0),
    ColorNames("chartreuse4", 69, 139, 0),
    ColorNames("chocolate", 210, 105, 30),
    ColorNames("chocolate1", 255, 127, 36),
    ColorNames("chocolate2", 238, 118, 33),
    ColorNames("chocolate3", 205, 102, 29),
    ColorNames("chocolate4", 139, 69, 19),
    ColorNames("coral", 255, 127, 80),
    ColorNames("coral1", 255, 114, 86),
    ColorNames("coral2", 238, 106, 80),
    ColorNames("coral3", 205, 91, 69),
    ColorNames("coral4", 139, 62, 47),
    ColorNames("cornflower blue", 100, 149, 237),
    ColorNames("CornflowerBlue", 100, 149, 237),
    ColorNames("cornsilk", 255, 248, 220),
    ColorNames("cornsilk1", 255, 248, 220),
    ColorNames("cornsilk2", 238, 232, 205),
    ColorNames("cornsilk3", 205, 200, 177),
    ColorNames("cornsilk4", 139, 136, 120),
    ColorNames("crymson", 220, 20, 60),
    ColorNames("cyan", 0, 255, 255),
    ColorNames("cyan1", 0, 255, 255),
    ColorNames("cyan2", 0, 238, 238),
    ColorNames("cyan3", 0, 205, 205),
    ColorNames("cyan4", 0, 139, 139),
    ColorNames("dark blue", 0, 0, 139),
    ColorNames("dark cyan", 0, 139, 139),
    ColorNames("dark goldenrod", 184, 134, 11),
    ColorNames("dark gray", 169, 169, 169),
    ColorNames("dark green", 0, 100, 0),
    ColorNames("dark grey", 169, 169, 169),
    ColorNames("dark khaki", 189, 183, 107),
    ColorNames("dark magenta", 139, 0, 139),
    ColorNames("dark olive green", 85, 107, 47),
    ColorNames("dark orange", 255, 140, 0),
    ColorNames("dark orchid", 153, 50, 204),
    ColorNames("dark red", 139, 0, 0),
    ColorNames("dark salmon", 233, 150, 122),
    ColorNames("dark sea green", 143, 188, 143),
    ColorNames("dark slate blue", 72, 61, 139),
    ColorNames("dark slate gray", 47, 79, 79),
    ColorNames("dark slate grey", 47, 79, 79),
    ColorNames("dark turquoise", 0, 206, 209),
    ColorNames("dark violet", 148, 0, 211),
    ColorNames("DarkBlue", 0, 0, 139),
    ColorNames("DarkCyan", 0, 139, 139),
    ColorNames("DarkGoldenrod", 184, 134, 11),
    ColorNames("DarkGoldenrod1", 255, 185, 15),
    ColorNames("DarkGoldenrod2", 238, 173, 14),
    ColorNames("DarkGoldenrod3", 205, 149, 12),
    ColorNames("DarkGoldenrod4", 139, 101, 8),
    ColorNames("DarkGray", 169, 169, 169),
    ColorNames("DarkGreen", 0, 100, 0),
    ColorNames("DarkGrey", 169, 169, 169),
    ColorNames("DarkKhaki", 189, 183, 107),
    ColorNames("DarkMagenta", 139, 0, 139),
    ColorNames("DarkOliveGreen", 85, 107, 47),
    ColorNames("DarkOliveGreen1", 202, 255, 112),
    ColorNames("DarkOliveGreen2", 188, 238, 104),
    ColorNames("DarkOliveGreen3", 162, 205, 90),
    ColorNames("DarkOliveGreen4", 110, 139, 61),
    ColorNames("DarkOrange", 255, 140, 0),
    ColorNames("DarkOrange1", 255, 127, 0),
    ColorNames("DarkOrange2", 238, 118, 0),
    ColorNames("DarkOrange3", 205, 102, 0),
    ColorNames("DarkOrange4", 139, 69, 0),
    ColorNames("DarkOrchid", 153, 50, 204),
    ColorNames("DarkOrchid1", 191, 62, 255),
    ColorNames("DarkOrchid2", 178, 58, 238),
    ColorNames("DarkOrchid3", 154, 50, 205),
    ColorNames("DarkOrchid4", 104, 34, 139),
    ColorNames("DarkRed", 139, 0, 0),
    ColorNames("DarkSalmon", 233, 150, 122),
    ColorNames("DarkSeaGreen", 143, 188, 143),
    ColorNames("DarkSeaGreen1", 193, 255, 193),
    ColorNames("DarkSeaGreen2", 180, 238, 180),
    ColorNames("DarkSeaGreen3", 155, 205, 155),
    ColorNames("DarkSeaGreen4", 105, 139, 105),
    ColorNames("DarkSlateBlue", 72, 61, 139),
    ColorNames("DarkSlateGray", 47, 79, 79),
    ColorNames("DarkSlateGray1", 151, 255, 255),
    ColorNames("DarkSlateGray2", 141, 238, 238),
    ColorNames("DarkSlateGray3", 121, 205, 205),
    ColorNames("DarkSlateGray4", 82, 139, 139),
    ColorNames("DarkSlateGrey", 47, 79, 79),
    ColorNames("DarkTurquoise", 0, 206, 209),
    ColorNames("DarkViolet", 148, 0, 211),
    ColorNames("deep pink", 255, 20, 147),
    ColorNames("deep sky blue", 0, 191, 255),
    ColorNames("DeepPink", 255, 20, 147),
    ColorNames("DeepPink1", 255, 20, 147),
    ColorNames("DeepPink2", 238, 18, 137),
    ColorNames("DeepPink3", 205, 16, 118),
    ColorNames("DeepPink4", 139, 10, 80),
    ColorNames("DeepSkyBlue", 0, 191, 255),
    ColorNames("DeepSkyBlue1", 0, 191, 255),
    ColorNames("DeepSkyBlue2", 0, 178, 238),
    ColorNames("DeepSkyBlue3", 0, 154, 205),
    ColorNames("DeepSkyBlue4", 0, 104, 139),
    ColorNames("dim gray", 105, 105, 105),
    ColorNames("dim grey", 105, 105, 105),
    ColorNames("DimGray", 105, 105, 105),
    ColorNames("DimGrey", 105, 105, 105),
    ColorNames("dodger blue", 30, 144, 255),
    ColorNames("DodgerBlue", 30, 144, 255),
    ColorNames("DodgerBlue1", 30, 144, 255),
    ColorNames("DodgerBlue2", 28, 134, 238),
    ColorNames("DodgerBlue3", 24, 116, 205),
    ColorNames("DodgerBlue4", 16, 78, 139),
    ColorNames("firebrick", 178, 34, 34),
    ColorNames("firebrick1", 255, 48, 48),
    ColorNames("firebrick2", 238, 44, 44),
    ColorNames("firebrick3", 205, 38, 38),
    ColorNames("firebrick4", 139, 26, 26),
    ColorNames("floral white", 255, 250, 240),
    ColorNames("FloralWhite", 255, 250, 240),
    ColorNames("forest green", 34, 139, 34),
    ColorNames("ForestGreen", 34, 139, 34),
    ColorNames("fuchsia", 255, 0, 255),
    ColorNames("gainsboro", 220, 220, 220),
    ColorNames("ghost white", 248, 248, 255),
    ColorNames("GhostWhite", 248, 248, 255),
    ColorNames("gold", 255, 215, 0),
    ColorNames("gold1", 255, 215, 0),
    ColorNames("gold2", 238, 201, 0),
    ColorNames("gold3", 205, 173, 0),
    ColorNames("gold4", 139, 117, 0),
    ColorNames("goldenrod", 218, 165, 32),
    ColorNames("goldenrod1", 255, 193, 37),
    ColorNames("goldenrod2", 238, 180, 34),
    ColorNames("goldenrod3", 205, 155, 29),
    ColorNames("goldenrod4", 139, 105, 20),
    ColorNames("gray", 128, 128, 128),
    ColorNames("gray0", 0, 0, 0),
    ColorNames("gray1", 3, 3, 3),
    ColorNames("gray2", 5, 5, 5),
    ColorNames("gray3", 8, 8, 8),
    ColorNames("gray4", 10, 10, 10),
    ColorNames("gray5", 13, 13, 13),
    ColorNames("gray6", 15, 15, 15),
    ColorNames("gray7", 18, 18, 18),
    ColorNames("gray8", 20, 20, 20),
    ColorNames("gray9", 23, 23, 23),
    ColorNames("gray10", 26, 26, 26),
    ColorNames("gray11", 28, 28, 28),
    ColorNames("gray12", 31, 31, 31),
    ColorNames("gray13", 33, 33, 33),
    ColorNames("gray14", 36, 36, 36),
    ColorNames("gray15", 38, 38, 38),
    ColorNames("gray16", 41, 41, 41),
    ColorNames("gray17", 43, 43, 43),
    ColorNames("gray18", 46, 46, 46),
    ColorNames("gray19", 48, 48, 48),
    ColorNames("gray20", 51, 51, 51),
    ColorNames("gray21", 54, 54, 54),
    ColorNames("gray22", 56, 56, 56),
    ColorNames("gray23", 59, 59, 59),
    ColorNames("gray24", 61, 61, 61),
    ColorNames("gray25", 64, 64, 64),
    ColorNames("gray26", 66, 66, 66),
    ColorNames("gray27", 69, 69, 69),
    ColorNames("gray28", 71, 71, 71),
    ColorNames("gray29", 74, 74, 74),
    ColorNames("gray30", 77, 77, 77),
    ColorNames("gray31", 79, 79, 79),
    ColorNames("gray32", 82, 82, 82),
    ColorNames("gray33", 84, 84, 84),
    ColorNames("gray34", 87, 87, 87),
    ColorNames("gray35", 89, 89, 89),
    ColorNames("gray36", 92, 92, 92),
    ColorNames("gray37", 94, 94, 94),
    ColorNames("gray38", 97, 97, 97),
    ColorNames("gray39", 99, 99, 99),
    ColorNames("gray40", 102, 102, 102),
    ColorNames("gray41", 105, 105, 105),
    ColorNames("gray42", 107, 107, 107),
    ColorNames("gray43", 110, 110, 110),
    ColorNames("gray44", 112, 112, 112),
    ColorNames("gray45", 115, 115, 115),
    ColorNames("gray46", 117, 117, 117),
    ColorNames("gray47", 120, 120, 120),
    ColorNames("gray48", 122, 122, 122),
    ColorNames("gray49", 125, 125, 125),
    ColorNames("gray50", 127, 127, 127),
    ColorNames("gray51", 130, 130, 130),
    ColorNames("gray52", 133, 133, 133),
    ColorNames("gray53", 135, 135, 135),
    ColorNames("gray54", 138, 138, 138),
    ColorNames("gray55", 140, 140, 140),
    ColorNames("gray56", 143, 143, 143),
    ColorNames("gray57", 145, 145, 145),
    ColorNames("gray58", 148, 148, 148),
    ColorNames("gray59", 150, 150, 150),
    ColorNames("gray60", 153, 153, 153),
    ColorNames("gray61", 156, 156, 156),
    ColorNames("gray62", 158, 158, 158),
    ColorNames("gray63", 161, 161, 161),
    ColorNames("gray64", 163, 163, 163),
    ColorNames("gray65", 166, 166, 166),
    ColorNames("gray66", 168, 168, 168),
    ColorNames("gray67", 171, 171, 171),
    ColorNames("gray68", 173, 173, 173),
    ColorNames("gray69", 176, 176, 176),
    ColorNames("gray70", 179, 179, 179),
    ColorNames("gray71", 181, 181, 181),
    ColorNames("gray72", 184, 184, 184),
    ColorNames("gray73", 186, 186, 186),
    ColorNames("gray74", 189, 189, 189),
    ColorNames("gray75", 191, 191, 191),
    ColorNames("gray76", 194, 194, 194),
    ColorNames("gray77", 196, 196, 196),
    ColorNames("gray78", 199, 199, 199),
    ColorNames("gray79", 201, 201, 201),
    ColorNames("gray80", 204, 204, 204),
    ColorNames("gray81", 207, 207, 207),
    ColorNames("gray82", 209, 209, 209),
    ColorNames("gray83", 212, 212, 212),
    ColorNames("gray84", 214, 214, 214),
    ColorNames("gray85", 217, 217, 217),
    ColorNames("gray86", 219, 219, 219),
    ColorNames("gray87", 222, 222, 222),
    ColorNames("gray88", 224, 224, 224),
    ColorNames("gray89", 227, 227, 227),
    ColorNames("gray90", 229, 229, 229),
    ColorNames("gray91", 232, 232, 232),
    ColorNames("gray92", 235, 235, 235),
    ColorNames("gray93", 237, 237, 237),
    ColorNames("gray94", 240, 240, 240),
    ColorNames("gray95", 242, 242, 242),
    ColorNames("gray96", 245, 245, 245),
    ColorNames("gray97", 247, 247, 247),
    ColorNames("gray98", 250, 250, 250),
    ColorNames("gray99", 252, 252, 252),
    ColorNames("gray100", 255, 255, 255),
    ColorNames("green", 0, 128, 0),
    ColorNames("green yellow", 173, 255, 47),
    ColorNames("green1", 0, 255, 0),
    ColorNames("green2", 0, 238, 0),
    ColorNames("green3", 0, 205, 0),
    ColorNames("green4", 0, 139, 0),
    ColorNames("GreenYellow", 173, 255, 47),
    ColorNames("grey", 128, 128, 128),
    ColorNames("grey0", 0, 0, 0),
    ColorNames("grey1", 3, 3, 3),
    ColorNames("grey2", 5, 5, 5),
    ColorNames("grey3", 8, 8, 8),
    ColorNames("grey4", 10, 10, 10),
    ColorNames("grey5", 13, 13, 13),
    ColorNames("grey6", 15, 15, 15),
    ColorNames("grey7", 18, 18, 18),
    ColorNames("grey8", 20, 20, 20),
    ColorNames("grey9", 23, 23, 23),
    ColorNames("grey10", 26, 26, 26),
    ColorNames("grey11", 28, 28, 28),
    ColorNames("grey12", 31, 31, 31),
    ColorNames("grey13", 33, 33, 33),
    ColorNames("grey14", 36, 36, 36),
    ColorNames("grey15", 38, 38, 38),
    ColorNames("grey16", 41, 41, 41),
    ColorNames("grey17", 43, 43, 43),
    ColorNames("grey18", 46, 46, 46),
    ColorNames("grey19", 48, 48, 48),
    ColorNames("grey20", 51, 51, 51),
    ColorNames("grey21", 54, 54, 54),
    ColorNames("grey22", 56, 56, 56),
    ColorNames("grey23", 59, 59, 59),
    ColorNames("grey24", 61, 61, 61),
    ColorNames("grey25", 64, 64, 64),
    ColorNames("grey26", 66, 66, 66),
    ColorNames("grey27", 69, 69, 69),
    ColorNames("grey28", 71, 71, 71),
    ColorNames("grey29", 74, 74, 74),
    ColorNames("grey30", 77, 77, 77),
    ColorNames("grey31", 79, 79, 79),
    ColorNames("grey32", 82, 82, 82),
    ColorNames("grey33", 84, 84, 84),
    ColorNames("grey34", 87, 87, 87),
    ColorNames("grey35", 89, 89, 89),
    ColorNames("grey36", 92, 92, 92),
    ColorNames("grey37", 94, 94, 94),
    ColorNames("grey38", 97, 97, 97),
    ColorNames("grey39", 99, 99, 99),
    ColorNames("grey40", 102, 102, 102),
    ColorNames("grey41", 105, 105, 105),
    ColorNames("grey42", 107, 107, 107),
    ColorNames("grey43", 110, 110, 110),
    ColorNames("grey44", 112, 112, 112),
    ColorNames("grey45", 115, 115, 115),
    ColorNames("grey46", 117, 117, 117),
    ColorNames("grey47", 120, 120, 120),
    ColorNames("grey48", 122, 122, 122),
    ColorNames("grey49", 125, 125, 125),
    ColorNames("grey50", 127, 127, 127),
    ColorNames("grey51", 130, 130, 130),
    ColorNames("grey52", 133, 133, 133),
    ColorNames("grey53", 135, 135, 135),
    ColorNames("grey54", 138, 138, 138),
    ColorNames("grey55", 140, 140, 140),
    ColorNames("grey56", 143, 143, 143),
    ColorNames("grey57", 145, 145, 145),
    ColorNames("grey58", 148, 148, 148),
    ColorNames("grey59", 150, 150, 150),
    ColorNames("grey60", 153, 153, 153),
    ColorNames("grey61", 156, 156, 156),
    ColorNames("grey62", 158, 158, 158),
    ColorNames("grey63", 161, 161, 161),
    ColorNames("grey64", 163, 163, 163),
    ColorNames("grey65", 166, 166, 166),
    ColorNames("grey66", 168, 168, 168),
    ColorNames("grey67", 171, 171, 171),
    ColorNames("grey68", 173, 173, 173),
    ColorNames("grey69", 176, 176, 176),
    ColorNames("grey70", 179, 179, 179),
    ColorNames("grey71", 181, 181, 181),
    ColorNames("grey72", 184, 184, 184),
    ColorNames("grey73", 186, 186, 186),
    ColorNames("grey74", 189, 189, 189),
    ColorNames("grey75", 191, 191, 191),
    ColorNames("grey76", 194, 194, 194),
    ColorNames("grey77", 196, 196, 196),
    ColorNames("grey78", 199, 199, 199),
    ColorNames("grey79", 201, 201, 201),
    ColorNames("grey80", 204, 204, 204),
    ColorNames("grey81", 207, 207, 207),
    ColorNames("grey82", 209, 209, 209),
    ColorNames("grey83", 212, 212, 212),
    ColorNames("grey84", 214, 214, 214),
    ColorNames("grey85", 217, 217, 217),
    ColorNames("grey86", 219, 219, 219),
    ColorNames("grey87", 222, 222, 222),
    ColorNames("grey88", 224, 224, 224),
    ColorNames("grey89", 227, 227, 227),
    ColorNames("grey90", 229, 229, 229),
    ColorNames("grey91", 232, 232, 232),
    ColorNames("grey92", 235, 235, 235),
    ColorNames("grey93", 237, 237, 237),
    ColorNames("grey94", 240, 240, 240),
    ColorNames("grey95", 242, 242, 242),
    ColorNames("grey96", 245, 245, 245),
    ColorNames("grey97", 247, 247, 247),
    ColorNames("grey98", 250, 250, 250),
    ColorNames("grey99", 252, 252, 252),
    ColorNames("grey100", 255, 255, 255),
    ColorNames("honeydew", 240, 255, 240),
    ColorNames("honeydew1", 240, 255, 240),
    ColorNames("honeydew2", 224, 238, 224),
    ColorNames("honeydew3", 193, 205, 193),
    ColorNames("honeydew4", 131, 139, 131),
    ColorNames("hot pink", 255, 105, 180),
    ColorNames("HotPink", 255, 105, 180),
    ColorNames("HotPink1", 255, 110, 180),
    ColorNames("HotPink2", 238, 106, 167),
    ColorNames("HotPink3", 205, 96, 144),
    ColorNames("HotPink4", 139, 58, 98),
    ColorNames("indian red", 205, 92, 92),
    ColorNames("IndianRed", 205, 92, 92),
    ColorNames("IndianRed1", 255, 106, 106),
    ColorNames("IndianRed2", 238, 99, 99),
    ColorNames("IndianRed3", 205, 85, 85),
    ColorNames("IndianRed4", 139, 58, 58),
    ColorNames("indigo", 75, 0, 130),
    ColorNames("ivory", 255, 255, 240),
    ColorNames("ivory1", 255, 255, 240),
    ColorNames("ivory2", 238, 238, 224),
    ColorNames("ivory3", 205, 205, 193),
    ColorNames("ivory4", 139, 139, 131),
    ColorNames("khaki", 240, 230, 140),
    ColorNames("khaki1", 255, 246, 143),
    ColorNames("khaki2", 238, 230, 133),
    ColorNames("khaki3", 205, 198, 115),
    ColorNames("khaki4", 139, 134, 78),
    ColorNames("lavender", 230, 230, 250),
    ColorNames("lavender blush", 255, 240, 245),
    ColorNames("LavenderBlush", 255, 240, 245),
    ColorNames("LavenderBlush1", 255, 240, 245),
    ColorNames("LavenderBlush2", 238, 224, 229),
    ColorNames("LavenderBlush3", 205, 193, 197),
    ColorNames("LavenderBlush4", 139, 131, 134),
    ColorNames("lawn green", 124, 252, 0),
    ColorNames("LawnGreen", 124, 252, 0),
    ColorNames("lemon chiffon", 255, 250, 205),
    ColorNames("LemonChiffon", 255, 250, 205),
    ColorNames("LemonChiffon1", 255, 250, 205),
    ColorNames("LemonChiffon2", 238, 233, 191),
    ColorNames("LemonChiffon3", 205, 201, 165),
    ColorNames("LemonChiffon4", 139, 137, 112),
    ColorNames("light blue", 173, 216, 230),
    ColorNames("light coral", 240, 128, 128),
    ColorNames("light cyan", 224, 255, 255),
    ColorNames("light goldenrod", 238, 221, 130),
    ColorNames("light goldenrod yellow", 250, 250, 210),
    ColorNames("light gray", 211, 211, 211),
    ColorNames("light green", 144, 238, 144),
    ColorNames("light grey", 211, 211, 211),
    ColorNames("light pink", 255, 182, 193),
    ColorNames("light salmon", 255, 160, 122),
    ColorNames("light sea green", 32, 178, 170),
    ColorNames("light sky blue", 135, 206, 250),
    ColorNames("light slate blue", 132, 112, 255),
    ColorNames("light slate gray", 119, 136, 153),
    ColorNames("light slate grey", 119, 136, 153),
    ColorNames("light steel blue", 176, 196, 222),
    ColorNames("light yellow", 255, 255, 224),
    ColorNames("LightBlue", 173, 216, 230),
    ColorNames("LightBlue1", 191, 239, 255),
    ColorNames("LightBlue2", 178, 223, 238),
    ColorNames("LightBlue3", 154, 192, 205),
    ColorNames("LightBlue4", 104, 131, 139),
    ColorNames("LightCoral", 240, 128, 128),
    ColorNames("LightCyan", 224, 255, 255),
    ColorNames("LightCyan1", 224, 255, 255),
    ColorNames("LightCyan2", 209, 238, 238),
    ColorNames("LightCyan3", 180, 205, 205),
    ColorNames("LightCyan4", 122, 139, 139),
    ColorNames("LightGoldenrod", 238, 221, 130),
    ColorNames("LightGoldenrod1", 255, 236, 139),
    ColorNames("LightGoldenrod2", 238, 220, 130),
    ColorNames("LightGoldenrod3", 205, 190, 112),
    ColorNames("LightGoldenrod4", 139, 129, 76),
    ColorNames("LightGoldenrodYellow", 250, 250, 210),
    ColorNames("LightGray", 211, 211, 211),
    ColorNames("LightGreen", 144, 238, 144),
    ColorNames("LightGrey", 211, 211, 211),
    ColorNames("LightPink", 255, 182, 193),
    ColorNames("LightPink1", 255, 174, 185),
    ColorNames("LightPink2", 238, 162, 173),
    ColorNames("LightPink3", 205, 140, 149),
    ColorNames("LightPink4", 139, 95, 101),
    ColorNames("LightSalmon", 255, 160, 122),
    ColorNames("LightSalmon1", 255, 160, 122),
    ColorNames("LightSalmon2", 238, 149, 114),
    ColorNames("LightSalmon3", 205, 129, 98),
    ColorNames("LightSalmon4", 139, 87, 66),
    ColorNames("LightSeaGreen", 32, 178, 170),
    ColorNames("LightSkyBlue", 135, 206, 250),
    ColorNames("LightSkyBlue1", 176, 226, 255),
    ColorNames("LightSkyBlue2", 164, 211, 238),
    ColorNames("LightSkyBlue3", 141, 182, 205),
    ColorNames("LightSkyBlue4", 96, 123, 139),
    ColorNames("LightSlateBlue", 132, 112, 255),
    ColorNames("LightSlateGray", 119, 136, 153),
    ColorNames("LightSlateGrey", 119, 136, 153),
    ColorNames("LightSteelBlue", 176, 196, 222),
    ColorNames("LightSteelBlue1", 202, 225, 255),
    ColorNames("LightSteelBlue2", 188, 210, 238),
    ColorNames("LightSteelBlue3", 162, 181, 205),
    ColorNames("LightSteelBlue4", 110, 123, 139),
    ColorNames("LightYellow", 255, 255, 224),
    ColorNames("LightYellow1", 255, 255, 224),
    ColorNames("LightYellow2", 238, 238, 209),
    ColorNames("LightYellow3", 205, 205, 180),
    ColorNames("LightYellow4", 139, 139, 122),
    ColorNames("lime", 0, 255, 0),
    ColorNames("lime green", 50, 205, 50),
    ColorNames("LimeGreen", 50, 205, 50),
    ColorNames("linen", 250, 240, 230),
    ColorNames("magenta", 255, 0, 255),
    ColorNames("magenta1", 255, 0, 255),
    ColorNames("magenta2", 238, 0, 238),
    ColorNames("magenta3", 205, 0, 205),
    ColorNames("magenta4", 139, 0, 139),
    ColorNames("maroon", 128, 0, 0),
    ColorNames("maroon1", 255, 52, 179),
    ColorNames("maroon2", 238, 48, 167),
    ColorNames("maroon3", 205, 41, 144),
    ColorNames("maroon4", 139, 28, 98),
    ColorNames("medium aquamarine", 102, 205, 170),
    ColorNames("medium blue", 0, 0, 205),
    ColorNames("medium orchid", 186, 85, 211),
    ColorNames("medium purple", 147, 112, 219),
    ColorNames("medium sea green", 60, 179, 113),
    ColorNames("medium slate blue", 123, 104, 238),
    ColorNames("medium spring green", 0, 250, 154),
    ColorNames("medium turquoise", 72, 209, 204),
    ColorNames("medium violet red", 199, 21, 133),
    ColorNames("MediumAquamarine", 102, 205, 170),
    ColorNames("MediumBlue", 0, 0, 205),
    ColorNames("MediumOrchid", 186, 85, 211),
    ColorNames("MediumOrchid1", 224, 102, 255),
    ColorNames("MediumOrchid2", 209, 95, 238),
    ColorNames("MediumOrchid3", 180, 82, 205),
    ColorNames("MediumOrchid4", 122, 55, 139),
    ColorNames("MediumPurple", 147, 112, 219),
    ColorNames("MediumPurple1", 171, 130, 255),
    ColorNames("MediumPurple2", 159, 121, 238),
    ColorNames("MediumPurple3", 137, 104, 205),
    ColorNames("MediumPurple4", 93, 71, 139),
    ColorNames("MediumSeaGreen", 60, 179, 113),
    ColorNames("MediumSlateBlue", 123, 104, 238),
    ColorNames("MediumSpringGreen", 0, 250, 154),
    ColorNames("MediumTurquoise", 72, 209, 204),
    ColorNames("MediumVioletRed", 199, 21, 133),
    ColorNames("midnight blue", 25, 25, 112),
    ColorNames("MidnightBlue", 25, 25, 112),
    ColorNames("mint cream", 245, 255, 250),
    ColorNames("MintCream", 245, 255, 250),
    ColorNames("misty rose", 255, 228, 225),
    ColorNames("MistyRose", 255, 228, 225),
    ColorNames("MistyRose1", 255, 228, 225),
    ColorNames("MistyRose2", 238, 213, 210),
    ColorNames("MistyRose3", 205, 183, 181),
    ColorNames("MistyRose4", 139, 125, 123),
    ColorNames("moccasin", 255, 228, 181),
    ColorNames("navajo white", 255, 222, 173),
    ColorNames("NavajoWhite", 255, 222, 173),
    ColorNames("NavajoWhite1", 255, 222, 173),
    ColorNames("NavajoWhite2", 238, 207, 161),
    ColorNames("NavajoWhite3", 205, 179, 139),
    ColorNames("NavajoWhite4", 139, 121, 94),
    ColorNames("navy", 0, 0, 128),
    ColorNames("navy blue", 0, 0, 128),
    ColorNames("NavyBlue", 0, 0, 128),
    ColorNames("old lace", 253, 245, 230),
    ColorNames("OldLace", 253, 245, 230),
    ColorNames("olive", 128, 128, 0),
    ColorNames("olive drab", 107, 142, 35),
    ColorNames("OliveDrab", 107, 142, 35),
    ColorNames("OliveDrab1", 192, 255, 62),
    ColorNames("OliveDrab2", 179, 238, 58),
    ColorNames("OliveDrab3", 154, 205, 50),
    ColorNames("OliveDrab4", 105, 139, 34),
    ColorNames("orange", 255, 165, 0),
    ColorNames("orange red", 255, 69, 0),
    ColorNames("orange1", 255, 165, 0),
    ColorNames("orange2", 238, 154, 0),
    ColorNames("orange3", 205, 133, 0),
    ColorNames("orange4", 139, 90, 0),
    ColorNames("OrangeRed", 255, 69, 0),
    ColorNames("OrangeRed1", 255, 69, 0),
    ColorNames("OrangeRed2", 238, 64, 0),
    ColorNames("OrangeRed3", 205, 55, 0),
    ColorNames("OrangeRed4", 139, 37, 0),
    ColorNames("orchid", 218, 112, 214),
    ColorNames("orchid1", 255, 131, 250),
    ColorNames("orchid2", 238, 122, 233),
    ColorNames("orchid3", 205, 105, 201),
    ColorNames("orchid4", 139, 71, 137),
    ColorNames("pale goldenrod", 238, 232, 170),
    ColorNames("pale green", 152, 251, 152),
    ColorNames("pale turquoise", 175, 238, 238),
    ColorNames("pale violet red", 219, 112, 147),
    ColorNames("PaleGoldenrod", 238, 232, 170),
    ColorNames("PaleGreen", 152, 251, 152),
    ColorNames("PaleGreen1", 154, 255, 154),
    ColorNames("PaleGreen2", 144, 238, 144),
    ColorNames("PaleGreen3", 124, 205, 124),
    ColorNames("PaleGreen4", 84, 139, 84),
    ColorNames("PaleTurquoise", 175, 238, 238),
    ColorNames("PaleTurquoise1", 187, 255, 255),
    ColorNames("PaleTurquoise2", 174, 238, 238),
    ColorNames("PaleTurquoise3", 150, 205, 205),
    ColorNames("PaleTurquoise4", 102, 139, 139),
    ColorNames("PaleVioletRed", 219, 112, 147),
    ColorNames("PaleVioletRed1", 255, 130, 171),
    ColorNames("PaleVioletRed2", 238, 121, 159),
    ColorNames("PaleVioletRed3", 205, 104, 127),
    ColorNames("PaleVioletRed4", 139, 71, 93),
    ColorNames("papaya whip", 255, 239, 213),
    ColorNames("PapayaWhip", 255, 239, 213),
    ColorNames("peach puff", 255, 218, 185),
    ColorNames("PeachPuff", 255, 218, 185),
    ColorNames("PeachPuff1", 255, 218, 185),
    ColorNames("PeachPuff2", 238, 203, 173),
    ColorNames("PeachPuff3", 205, 175, 149),
    ColorNames("PeachPuff4", 139, 119, 101),
    ColorNames("peru", 205, 133, 63),
    ColorNames("pink", 255, 192, 203),
    ColorNames("pink1", 255, 181, 197),
    ColorNames("pink2", 238, 169, 184),
    ColorNames("pink3", 205, 145, 158),
    ColorNames("pink4", 139, 99, 108),
    ColorNames("plum", 221, 160, 221),
    ColorNames("plum1", 255, 187, 255),
    ColorNames("plum2", 238, 174, 238),
    ColorNames("plum3", 205, 150, 205),
    ColorNames("plum4", 139, 102, 139),
    ColorNames("powder blue", 176, 224, 230),
    ColorNames("PowderBlue", 176, 224, 230),
    ColorNames("purple", 128, 0, 128),
    ColorNames("purple1", 155, 48, 255),
    ColorNames("purple2", 145, 44, 238),
    ColorNames("purple3", 125, 38, 205),
    ColorNames("purple4", 85, 26, 139),
    ColorNames("red", 255, 0, 0),
    ColorNames("red1", 255, 0, 0),
    ColorNames("red2", 238, 0, 0),
    ColorNames("red3", 205, 0, 0),
    ColorNames("red4", 139, 0, 0),
    ColorNames("rosy brown", 188, 143, 143),
    ColorNames("RosyBrown", 188, 143, 143),
    ColorNames("RosyBrown1", 255, 193, 193),
    ColorNames("RosyBrown2", 238, 180, 180),
    ColorNames("RosyBrown3", 205, 155, 155),
    ColorNames("RosyBrown4", 139, 105, 105),
    ColorNames("royal blue", 65, 105, 225),
    ColorNames("RoyalBlue", 65, 105, 225),
    ColorNames("RoyalBlue1", 72, 118, 255),
    ColorNames("RoyalBlue2", 67, 110, 238),
    ColorNames("RoyalBlue3", 58, 95, 205),
    ColorNames("RoyalBlue4", 39, 64, 139),
    ColorNames("saddle brown", 139, 69, 19),
    ColorNames("SaddleBrown", 139, 69, 19),
    ColorNames("salmon", 250, 128, 114),
    ColorNames("salmon1", 255, 140, 105),
    ColorNames("salmon2", 238, 130, 98),
    ColorNames("salmon3", 205, 112, 84),
    ColorNames("salmon4", 139, 76, 57),
    ColorNames("sandy brown", 244, 164, 96),
    ColorNames("SandyBrown", 244, 164, 96),
    ColorNames("sea green", 46, 139, 87),
    ColorNames("SeaGreen", 46, 139, 87),
    ColorNames("SeaGreen1", 84, 255, 159),
    ColorNames("SeaGreen2", 78, 238, 148),
    ColorNames("SeaGreen3", 67, 205, 128),
    ColorNames("SeaGreen4", 46, 139, 87),
    ColorNames("seashell", 255, 245, 238),
    ColorNames("seashell1", 255, 245, 238),
    ColorNames("seashell2", 238, 229, 222),
    ColorNames("seashell3", 205, 197, 191),
    ColorNames("seashell4", 139, 134, 130),
    ColorNames("sienna", 160, 82, 45),
    ColorNames("sienna1", 255, 130, 71),
    ColorNames("sienna2", 238, 121, 66),
    ColorNames("sienna3", 205, 104, 57),
    ColorNames("sienna4", 139, 71, 38),
    ColorNames("silver", 192, 192, 192),
    ColorNames("sky blue", 135, 206, 235),
    ColorNames("SkyBlue", 135, 206, 235),
    ColorNames("SkyBlue1", 135, 206, 255),
    ColorNames("SkyBlue2", 126, 192, 238),
    ColorNames("SkyBlue3", 108, 166, 205),
    ColorNames("SkyBlue4", 74, 112, 139),
    ColorNames("slate blue", 106, 90, 205),
    ColorNames("slate gray", 112, 128, 144),
    ColorNames("slate grey", 112, 128, 144),
    ColorNames("SlateBlue", 106, 90, 205),
    ColorNames("SlateBlue1", 131, 111, 255),
    ColorNames("SlateBlue2", 122, 103, 238),
    ColorNames("SlateBlue3", 105, 89, 205),
    ColorNames("SlateBlue4", 71, 60, 139),
    ColorNames("SlateGray", 112, 128, 144),
    ColorNames("SlateGray1", 198, 226, 255),
    ColorNames("SlateGray2", 185, 211, 238),
    ColorNames("SlateGray3", 159, 182, 205),
    ColorNames("SlateGray4", 108, 123, 139),
    ColorNames("SlateGrey", 112, 128, 144),
    ColorNames("snow", 255, 250, 250),
    ColorNames("snow1", 255, 250, 250),
    ColorNames("snow2", 238, 233, 233),
    ColorNames("snow3", 205, 201, 201),
    ColorNames("snow4", 139, 137, 137),
    ColorNames("spring green", 0, 255, 127),
    ColorNames("SpringGreen", 0, 255, 127),
    ColorNames("SpringGreen1", 0, 255, 127),
    ColorNames("SpringGreen2", 0, 238, 118),
    ColorNames("SpringGreen3", 0, 205, 102),
    ColorNames("SpringGreen4", 0, 139, 69),
    ColorNames("steel blue", 70, 130, 180),
    ColorNames("SteelBlue", 70, 130, 180),
    ColorNames("SteelBlue1", 99, 184, 255),
    ColorNames("SteelBlue2", 92, 172, 238),
    ColorNames("SteelBlue3", 79, 148, 205),
    ColorNames("SteelBlue4", 54, 100, 139),
    ColorNames("tan", 210, 180, 140),
    ColorNames("tan1", 255, 165, 79),
    ColorNames("tan2", 238, 154, 73),
    ColorNames("tan3", 205, 133, 63),
    ColorNames("tan4", 139, 90, 43),
    ColorNames("teal", 0, 128, 128),
    ColorNames("thistle", 216, 191, 216),
    ColorNames("thistle1", 255, 225, 255),
    ColorNames("thistle2", 238, 210, 238),
    ColorNames("thistle3", 205, 181, 205),
    ColorNames("thistle4", 139, 123, 139),
    ColorNames("tomato", 255, 99, 71),
    ColorNames("tomato1", 255, 99, 71),
    ColorNames("tomato2", 238, 92, 66),
    ColorNames("tomato3", 205, 79, 57),
    ColorNames("tomato4", 139, 54, 38),
    ColorNames("turquoise", 64, 224, 208),
    ColorNames("turquoise1", 0, 245, 255),
    ColorNames("turquoise2", 0, 229, 238),
    ColorNames("turquoise3", 0, 197, 205),
    ColorNames("turquoise4", 0, 134, 139),
    ColorNames("violet", 238, 130, 238),
    ColorNames("violet red", 208, 32, 144),
    ColorNames("VioletRed", 208, 32, 144),
    ColorNames("VioletRed1", 255, 62, 150),
    ColorNames("VioletRed2", 238, 58, 140),
    ColorNames("VioletRed3", 205, 50, 120),
    ColorNames("VioletRed4", 139, 34, 82),
    ColorNames("wheat", 245, 222, 179),
    ColorNames("wheat1", 255, 231, 186),
    ColorNames("wheat2", 238, 216, 174),
    ColorNames("wheat3", 205, 186, 150),
    ColorNames("wheat4", 139, 126, 102),
    ColorNames("white", 255, 255, 255),
    ColorNames("white smoke", 245, 245, 245),
    ColorNames("WhiteSmoke", 245, 245, 245),
    ColorNames("yellow", 255, 255, 0),
    ColorNames("yellow green", 154, 205, 50),
    ColorNames("yellow1", 255, 255, 0),
    ColorNames("yellow2", 238, 238, 0),
    ColorNames("yellow3", 205, 205, 0),
    ColorNames("yellow4", 139, 139, 0),
    ColorNames("YellowGreen", 154, 205, 50),
];
