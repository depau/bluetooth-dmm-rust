use std::error::Error;

use packed_struct::prelude::*;
use phf::phf_map;

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayValue {
    Number(f32),
    Text(String),
}

impl DisplayValue {
    pub fn parse(text: &str) -> DisplayValue {
        if let Ok(f) = text.parse::<f32>() {
            DisplayValue::Number(f)
        } else {
            DisplayValue::Text(text.to_string())
        }
    }
}

const SEGMENTS_MAP: phf::Map<u8, char> = phf_map! {
    0b1111101_u8 => '0',
    0b0000101_u8 => '1',
    0b1011011_u8 => '2',
    0b0011111_u8 => '3',
    0b0100111_u8 => '4',
    0b0111110_u8 => '5',
    0b1111110_u8 => '6',
    0b0010101_u8 => '7',
    0b1111111_u8 => '8',
    0b0111111_u8 => '9',
    0b1110111_u8 => 'A',
    0b1001100_u8 => 'u',
    0b1101010_u8 => 't',
    0b1001110_u8 => 'o',
    0b1101000_u8 => 'L',
    0b1111010_u8 => 'E',
    0b1110010_u8 => 'F',
    0b0000000_u8 => ' ',
    0b0000010_u8 => '-',
    // Not actually displayed, here just in case
    0b1101110_u8 => 'b',
    0b1111000_u8 => 'C',
    0b1001010_u8 => 'c',
    0b1001111_u8 => 'd',
    0b1100111_u8 => 'H',
    0b1001101_u8 => 'J',
    0b1000110_u8 => 'n',
    0b1110011_u8 => 'P',
    0b1000010_u8 => 'r',
    0b1101101_u8 => 'U',
    0b0101111_u8 => 'Y',
};

#[derive(Debug, Clone, PartialEq)]
pub struct SevenSegmentDisplay {
    pub segments: u8,
}

impl SevenSegmentDisplay {
    pub fn get_top(&self) -> bool {
        self.segments & 0b10000 != 0
    }

    pub fn get_top_left(&self) -> bool {
        self.segments & 0b100000 != 0
    }

    pub fn get_top_right(&self) -> bool {
        self.segments & 0b1 != 0
    }

    pub fn get_middle(&self) -> bool {
        self.segments & 0b10 != 0
    }

    pub fn get_bottom_left(&self) -> bool {
        self.segments & 0b1000000 != 0
    }

    pub fn get_bottom_right(&self) -> bool {
        self.segments & 0b100 != 0
    }

    pub fn get_bottom(&self) -> bool {
        self.segments & 0b1000 != 0
    }

    pub fn get_dot_dash(&self) -> bool {
        self.segments & 0b10000000 != 0
    }

    pub fn get_text(&self) -> Option<char> {
        SEGMENTS_MAP.get(&(self.segments & 0b111_1111)).cloned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisplayIcon {
    AC,
    Ampere,
    Auto,
    Bluetooth,
    Buzz,
    DC,
    DegC,
    DegF,
    Delta,
    Diode,
    Farad,
    Flash,
    Hertz,
    Hold,
    KiloOhm,
    Max,
    MegaOhm,
    MicroAmpere,
    MicroFarad,
    MilliAmpere,
    MilliFarad,
    MilliVolt,
    Min,
    Nano,
    NanoFarad,
    Ohm,
    Percent,
    Volt,
    Unknown(&'static str),
}

static DMM_3_ICONS: [DisplayIcon; 32] = [
    DisplayIcon::Unknown("?1"),
    DisplayIcon::Delta,
    DisplayIcon::Bluetooth,
    DisplayIcon::Buzz,
    DisplayIcon::Hold,
    DisplayIcon::DegF,
    DisplayIcon::DegC,
    DisplayIcon::Diode,
    DisplayIcon::Max,
    DisplayIcon::Min,
    DisplayIcon::Percent,
    DisplayIcon::AC,
    DisplayIcon::Farad,
    DisplayIcon::MicroFarad,
    DisplayIcon::MilliFarad,
    DisplayIcon::NanoFarad,
    DisplayIcon::Hertz,
    DisplayIcon::Ohm,
    DisplayIcon::KiloOhm,
    DisplayIcon::MegaOhm,
    DisplayIcon::Volt,
    DisplayIcon::MilliVolt,
    DisplayIcon::DC,
    DisplayIcon::Ampere,
    DisplayIcon::Auto,
    DisplayIcon::Unknown("?2"),
    DisplayIcon::MicroAmpere,
    DisplayIcon::MilliAmpere,
    DisplayIcon::Unknown("?3"),
    DisplayIcon::Unknown("?4"),
    DisplayIcon::Unknown("?5"),
    DisplayIcon::Unknown("?6"),
];

static DMM_1_ICONS: [DisplayIcon; 32] = [
    DisplayIcon::Unknown("?1"),
    DisplayIcon::Hold,
    DisplayIcon::Flash,
    DisplayIcon::Buzz,
    DisplayIcon::Unknown(" "),
    DisplayIcon::Unknown(" "),
    DisplayIcon::Unknown(" "),
    DisplayIcon::Unknown(" "),
    DisplayIcon::Nano,
    DisplayIcon::Volt,
    DisplayIcon::DC,
    DisplayIcon::AC,
    DisplayIcon::Farad,
    DisplayIcon::Diode,
    DisplayIcon::Ampere,
    DisplayIcon::MicroFarad,
    DisplayIcon::Ohm,
    DisplayIcon::KiloOhm,
    DisplayIcon::MegaOhm,
    DisplayIcon::Unknown(" "),
    DisplayIcon::Hertz,
    DisplayIcon::DegF,
    DisplayIcon::DegC,
    DisplayIcon::Unknown(" "),
    DisplayIcon::Unknown(" "),
    DisplayIcon::Unknown(" "),
    DisplayIcon::Unknown(" "),
    DisplayIcon::Unknown(" "),
    DisplayIcon::Unknown(" "),
    DisplayIcon::Unknown(" "),
    DisplayIcon::Unknown(" "),
    DisplayIcon::Unknown(" "),
];

#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    pub dmm_id: u8,
    pub display_segments: [SevenSegmentDisplay; 4],
    pub displayed_value: DisplayValue,
    pub displayed_icons: Vec<DisplayIcon>,
    pub value_unit: Option<String>,
}

static XOR_KEY: [u8; 20] = [
    0x41, 0x21, 0x73, 0x55, 0xa2, 0xc1, 0x32, 0x71, 0x66, 0xaa, 0x3b, 0xd0, 0xe2, 0xa8, 0x33, 0x14,
    0x20, 0x21, 0xaa, 0xbb,
];

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "msb0", size_bytes = "11", endian = "msb")]
struct MeasurementData {
    #[packed_field(bits = "0..16")]
    preamble: u16,
    #[packed_field(bits = "16..18")]
    dmm_id: Integer<u8, packed_bits::Bits<2>>,
    #[packed_field(bits = "24..28", element_size_bits = "1")]
    icons1: [bool; 4],
    #[packed_field(bits = "28..60")]
    seven_segments: [u8; 4],
    #[packed_field(bits = "60..88", element_size_bits = "1")]
    icons2: [bool; 27],
}

// from_bytes function
impl Measurement {
    pub fn from_bytes(data: &[u8]) -> Result<Measurement, Box<dyn Error>> {
        assert_eq!(data.len(), 11);
        let decoded: Vec<u8> = data
            .iter()
            .zip(XOR_KEY.iter())
            .map(|(a, b)| a ^ b)
            .map(|x| x.reverse_bits())
            .collect();

        let bytearray: [u8; 11] = decoded.as_slice().try_into().unwrap();
        let data: MeasurementData = MeasurementData::unpack(&bytearray)?;

        // Extract the value segments
        let segments = data
            .seven_segments
            .iter()
            .map(|&s| SevenSegmentDisplay { segments: s })
            .collect::<Vec<_>>();

        // Try to parse the displayed value
        let mut display_text = String::with_capacity(5);
        if segments[0].get_dot_dash() {
            display_text.push('-');
        }
        for (i, segment) in segments.iter().enumerate() {
            if i != 0 && segment.get_dot_dash() {
                display_text.push('.');
            }
            if let Some(c) = segment.get_text() {
                display_text.push(c);
            } else {
                display_text.push('?');
            }
        }

        let displayed_value = DisplayValue::parse(&display_text);

        // Extract the displayed icons
        let icons_map = if data.dmm_id.to_be() == 3 {
            &DMM_3_ICONS
        } else {
            &DMM_1_ICONS
        };

        let mut displayed_icons = Vec::new();
        for (i, &icon) in data.icons1.iter().chain(data.icons2.iter()).enumerate() {
            if icon {
                if let Some(icon) = icons_map.get(i) {
                    displayed_icons.push(icon.clone());
                }
            }
        }

        // Try to figure out the value unit based on the displayed icons
        let mut value_unit = if displayed_icons.contains(&DisplayIcon::MilliVolt) {
            Some("mV".to_string())
        } else if displayed_icons.contains(&DisplayIcon::Volt) {
            Some("V".to_string())
        } else if displayed_icons.contains(&DisplayIcon::MilliAmpere) {
            Some("mA".to_string())
        } else if displayed_icons.contains(&DisplayIcon::MicroAmpere) {
            Some("μA".to_string())
        } else if displayed_icons.contains(&DisplayIcon::Ampere) {
            Some("A".to_string())
        } else if displayed_icons.contains(&DisplayIcon::KiloOhm) {
            Some("kΩ".to_string())
        } else if displayed_icons.contains(&DisplayIcon::MegaOhm) {
            Some("MΩ".to_string())
        } else if displayed_icons.contains(&DisplayIcon::Ohm) {
            Some("Ω".to_string())
        } else if displayed_icons.contains(&DisplayIcon::MicroFarad) {
            Some("μF".to_string())
        } else if displayed_icons.contains(&DisplayIcon::MilliFarad) {
            Some("mF".to_string())
        } else if displayed_icons.contains(&DisplayIcon::Farad) {
            Some("F".to_string())
        } else if displayed_icons.contains(&DisplayIcon::DegC) {
            Some("°C".to_string())
        } else if displayed_icons.contains(&DisplayIcon::DegF) {
            Some("°F".to_string())
        } else if displayed_icons.contains(&DisplayIcon::Hertz) {
            Some("Hz".to_string())
        } else if displayed_icons.contains(&DisplayIcon::Percent) {
            Some("%".to_string())
        } else {
            None
        };

        if value_unit.is_some() && displayed_icons.contains(&DisplayIcon::Nano) {
            value_unit = Some("n".to_string() + &value_unit.unwrap());
        }

        let segments: [SevenSegmentDisplay; 4] = segments.try_into().unwrap();
        Ok(Measurement {
            dmm_id: data.dmm_id.to_be(),
            display_segments: segments,
            displayed_value,
            displayed_icons,
            value_unit,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven_segment() {
        let display = SevenSegmentDisplay {
            segments: 0b111_1111, // '8'
        };

        assert_eq!(display.get_top(), true);
        assert_eq!(display.get_top_left(), true);
        assert_eq!(display.get_top_right(), true);
        assert_eq!(display.get_middle(), true);
        assert_eq!(display.get_bottom_left(), true);
        assert_eq!(display.get_bottom_right(), true);
        assert_eq!(display.get_bottom(), true);
        assert_eq!(display.get_dot_dash(), false);
        assert_eq!(display.get_text(), Some('8'));

        let display = SevenSegmentDisplay {
            segments: 0b000_0010, // '-'
        };

        assert_eq!(display.get_top(), false);
        assert_eq!(display.get_top_left(), false);
        assert_eq!(display.get_top_right(), false);
        assert_eq!(display.get_middle(), true);
        assert_eq!(display.get_bottom_left(), false);
        assert_eq!(display.get_bottom_right(), false);
        assert_eq!(display.get_bottom(), false);
        assert_eq!(display.get_dot_dash(), false);
        assert_eq!(display.get_text(), Some('-'));

        let display = SevenSegmentDisplay {
            segments: 0, // ' '
        };

        assert_eq!(display.get_top(), false);
        assert_eq!(display.get_top_left(), false);
        assert_eq!(display.get_top_right(), false);
        assert_eq!(display.get_middle(), false);
        assert_eq!(display.get_bottom_left(), false);
        assert_eq!(display.get_bottom_right(), false);
        assert_eq!(display.get_bottom(), false);
        assert_eq!(display.get_dot_dash(), false);
        assert_eq!(display.get_text(), Some(' '));

        let display = SevenSegmentDisplay {
            segments: 0b11111101, // '.0'
        };

        assert_eq!(display.get_top(), true);
        assert_eq!(display.get_top_left(), true);
        assert_eq!(display.get_top_right(), true);
        assert_eq!(display.get_middle(), false);
        assert_eq!(display.get_bottom_left(), true);
        assert_eq!(display.get_bottom_right(), true);
        assert_eq!(display.get_bottom(), true);
        assert_eq!(display.get_dot_dash(), true);
        assert_eq!(display.get_text(), Some('0'));

        let display = SevenSegmentDisplay {
            segments: 0b0100110, // a 4 without the top-right segment
        };

        assert_eq!(display.get_top(), false);
        assert_eq!(display.get_top_left(), true);
        assert_eq!(display.get_top_right(), false);
        assert_eq!(display.get_middle(), true);
        assert_eq!(display.get_bottom_left(), false);
        assert_eq!(display.get_bottom_right(), true);
        assert_eq!(display.get_bottom(), false);
        assert_eq!(display.get_dot_dash(), false);
        assert_eq!(display.get_text(), None);
    }

    #[test]
    fn test_measurement() {
        // Display: Auto
        // Value type: str
        // Icons: ['BT']
        let data: [u8; 11] = [27, 132, 112, 177, 140, 162, 23, 118, 102, 170, 59];
        let measurement = Measurement::from_bytes(&data).unwrap();
        assert_eq!(measurement.dmm_id, 3);
        assert_eq!(measurement.display_segments[0].get_text(), Some('A'));
        assert_eq!(measurement.display_segments[1].get_text(), Some('u'));
        assert_eq!(measurement.display_segments[2].get_text(), Some('t'));
        assert_eq!(measurement.display_segments[3].get_text(), Some('o'));
        assert_eq!(
            measurement.displayed_value,
            DisplayValue::Text("Auto".to_string())
        );
        assert_eq!(measurement.displayed_icons, vec![DisplayIcon::Bluetooth]);
        assert_eq!(measurement.value_unit, None);

        // Display: 0.000
        // Value type: float
        // Icons:  ['BT', 'V', 'DC', 'AUTO']
        // Unit: V
        let data: [u8; 11] = [27, 132, 112, 177, 89, 42, 217, 122, 102, 250, 58];
        let measurement = Measurement::from_bytes(&data).unwrap();
        assert_eq!(measurement.dmm_id, 3);
        assert_eq!(measurement.display_segments[0].get_dot_dash(), false);
        assert_eq!(measurement.display_segments[0].get_text(), Some('0'));
        assert_eq!(measurement.display_segments[1].get_dot_dash(), true);
        assert_eq!(measurement.display_segments[1].get_text(), Some('0'));
        assert_eq!(measurement.display_segments[2].get_text(), Some('0'));
        assert_eq!(measurement.display_segments[3].get_text(), Some('0'));
        assert_eq!(measurement.displayed_value, DisplayValue::Number(0.0));
        assert_eq!(
            measurement.displayed_icons,
            vec![
                DisplayIcon::Bluetooth,
                DisplayIcon::Volt,
                DisplayIcon::DC,
                DisplayIcon::Auto
            ]
        );
        assert_eq!(measurement.value_unit, Some("V".to_string()));

        // Display:  -09.57
        // Value type:  float
        // Icons:  ['BT', 'V', 'DC', 'AUTO']
        // Unit: V
        let data: [u8; 11] = [27, 132, 112, 161, 105, 30, 181, 123, 102, 250, 58];
        let measurement = Measurement::from_bytes(&data).unwrap();
        assert_eq!(measurement.dmm_id, 3);
        assert_eq!(measurement.display_segments[0].get_dot_dash(), true);
        assert_eq!(measurement.display_segments[0].get_text(), Some('0'));
        assert_eq!(measurement.display_segments[1].get_text(), Some('9'));
        assert_eq!(measurement.display_segments[2].get_dot_dash(), true);
        assert_eq!(measurement.display_segments[2].get_text(), Some('5'));
        assert_eq!(measurement.display_segments[3].get_text(), Some('7'));
        assert_eq!(measurement.displayed_value, DisplayValue::Number(-9.57));
        assert_eq!(
            measurement.displayed_icons,
            vec![
                DisplayIcon::Bluetooth,
                DisplayIcon::Volt,
                DisplayIcon::DC,
                DisplayIcon::Auto
            ]
        );
        assert_eq!(measurement.value_unit, Some("V".to_string()));

        // Display: " .0L "
        // Value type: str
        // Icons: ['BT', 'BUZ', 'DIODE', 'V']
        // Unit: V
        let data: [u8; 11] = [27, 132, 112, 89, 82, 170, 51, 241, 102, 186, 59];
        let measurement = Measurement::from_bytes(&data).unwrap();
        assert_eq!(measurement.dmm_id, 3);
        assert_eq!(measurement.display_segments[0].get_dot_dash(), false);
        assert_eq!(measurement.display_segments[1].get_dot_dash(), true);
        assert_eq!(
            measurement.displayed_value,
            DisplayValue::Text(" .0L ".to_string())
        );
        assert_eq!(
            measurement.displayed_icons,
            vec![
                DisplayIcon::Bluetooth,
                DisplayIcon::Buzz,
                DisplayIcon::Diode,
                DisplayIcon::Volt
            ]
        );
        assert_eq!(measurement.value_unit, Some("V".to_string()));

        // Display:  -0.43
        // Value type:  float
        // Icons:  ['BT', 'V', 'm(V)', 'DC', 'AUTO']
        // Unit: mV
        let data: [u8; 11] = [27, 132, 112, 161, 73, 154, 188, 126, 102, 218, 58];
        let measurement = Measurement::from_bytes(&data).unwrap();
        assert_eq!(
            measurement.displayed_icons,
            vec![
                DisplayIcon::Bluetooth,
                DisplayIcon::Volt,
                DisplayIcon::MilliVolt,
                DisplayIcon::DC,
                DisplayIcon::Auto
            ]
        );
        assert_eq!(measurement.value_unit, Some("mV".to_string()));
    }
}
