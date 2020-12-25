use crate::data::byte::u7::U7;
use crate::data::byte::from_traits::FromOverFlow;
use num_enum::TryFromPrimitive;
/// A simple enum type that represents all the midi 'notes'
/// note the flat versions are associated constants
/// but can be referenced like Note::Bb3
/// C1m is the C-1
#[derive(Debug,Copy,Clone,TryFromPrimitive)]
#[repr(u8)]
pub enum Note {
    C1m, Cs1m, D1m, Ds1m, E1m, F1m, Fs1m, G1m, Gs1m, A1m, As1m, B1m,
    C0 , Cs0 , D0 , Ds0 , E0 , F0 , Fs0 , G0 , Gs0 , A0 , As0 , B0 ,
    C1 , Cs1 , D1 , Ds1 , E1 , F1 , Fs1 , G1 , Gs1 , A1 , As1 , B1 , 
    C2 , Cs2 , D2 , Ds2 , E2 , F2 , Fs2 , G2 , Gs2 , A2 , As2 , B2 ,
    C3 , Cs3 , D3 , Ds3 , E3 , F3 , Fs3 , G3 , Gs3 , A3 , As3 , B3 ,
    C4 , Cs4 , D4 , Ds4 , E4 , F4 , Fs4 , G4 , Gs4 , A4 , As4 , B4 ,
    C5 , Cs5 , D5 , Ds5 , E5 , F5 , Fs5 , G5 , Gs5 , A5 , As5 , B5 ,
    C6 , Cs6 , D6 , Ds6 , E6 , F6 , Fs6 , G6 , Gs6 , A6 , As6 , B6 ,
    C7 , Cs7 , D7 , Ds7 , E7 , F7 , Fs7 , G7 , Gs7 , A7 , As7 , B7 ,
    C8 , Cs8 , D8 , Ds8 , E8 , F8 , Fs8 , G8 , Gs8 , A8 , As8 , B8 ,
    C9 , Cs9 , D9 , Ds9 , E9 , F9 , Fs9 , G9 , Gs9
}


impl Into<u8> for Note {
    fn into(self) -> u8 {
        self as u8 
    }
}

impl From<Note> for U7 {
    fn from(value:Note) -> U7{
        let byte = value as u8;
        U7::from_overflow(byte)
    }
}


impl Note {
    #[allow(non_upper_case_globals)] pub const Db1m : Note = Note::Cs1m;
    #[allow(non_upper_case_globals)] pub const Eb1m : Note = Note::Ds1m;
    #[allow(non_upper_case_globals)] pub const Gb1m : Note = Note::Fs1m;
    #[allow(non_upper_case_globals)] pub const Ab1m : Note = Note::Gs1m;
    #[allow(non_upper_case_globals)] pub const Bb1m : Note = Note::As1m;
    #[allow(non_upper_case_globals)] pub const Db0 : Note = Note::Cs0;
    #[allow(non_upper_case_globals)] pub const Eb0 : Note = Note::Ds0;
    #[allow(non_upper_case_globals)] pub const Gb0 : Note = Note::Fs0;
    #[allow(non_upper_case_globals)] pub const Ab0 : Note = Note::Gs0;
    #[allow(non_upper_case_globals)] pub const Bb0 : Note = Note::As0;
    #[allow(non_upper_case_globals)] pub const Db1 : Note = Note::Cs1;
    #[allow(non_upper_case_globals)] pub const Eb1 : Note = Note::Ds1;
    #[allow(non_upper_case_globals)] pub const Gb1 : Note = Note::Fs1;
    #[allow(non_upper_case_globals)] pub const Ab1 : Note = Note::Gs1;
    #[allow(non_upper_case_globals)] pub const Bb1 : Note = Note::As1;
    #[allow(non_upper_case_globals)] pub const Db2 : Note = Note::Cs2;
    #[allow(non_upper_case_globals)] pub const Eb2 : Note = Note::Ds2;
    #[allow(non_upper_case_globals)] pub const Gb2 : Note = Note::Fs2;
    #[allow(non_upper_case_globals)] pub const Ab2 : Note = Note::Gs2;
    #[allow(non_upper_case_globals)] pub const Bb2 : Note = Note::As2;
    #[allow(non_upper_case_globals)] pub const Db3 : Note = Note::Cs3;
    #[allow(non_upper_case_globals)] pub const Eb3 : Note = Note::Ds3;
    #[allow(non_upper_case_globals)] pub const Gb3 : Note = Note::Fs3;
    #[allow(non_upper_case_globals)] pub const Ab3 : Note = Note::Gs3;
    #[allow(non_upper_case_globals)] pub const Bb3 : Note = Note::As3;
    #[allow(non_upper_case_globals)] pub const Db4 : Note = Note::Cs4;
    #[allow(non_upper_case_globals)] pub const Eb4 : Note = Note::Ds4;
    #[allow(non_upper_case_globals)] pub const Gb4 : Note = Note::Fs4;
    #[allow(non_upper_case_globals)] pub const Ab4 : Note = Note::Gs4;
    #[allow(non_upper_case_globals)] pub const Bb4 : Note = Note::As4;
    #[allow(non_upper_case_globals)] pub const Db5 : Note = Note::Cs5;
    #[allow(non_upper_case_globals)] pub const Eb5 : Note = Note::Ds5;
    #[allow(non_upper_case_globals)] pub const Gb5 : Note = Note::Fs5;
    #[allow(non_upper_case_globals)] pub const Ab5 : Note = Note::Gs5;
    #[allow(non_upper_case_globals)] pub const Bb5 : Note = Note::As5;
    #[allow(non_upper_case_globals)] pub const Db6 : Note = Note::Cs6;
    #[allow(non_upper_case_globals)] pub const Eb6 : Note = Note::Ds6;
    #[allow(non_upper_case_globals)] pub const Gb6 : Note = Note::Fs6;
    #[allow(non_upper_case_globals)] pub const Ab6 : Note = Note::Gs6;
    #[allow(non_upper_case_globals)] pub const Bb6 : Note = Note::As6;
    #[allow(non_upper_case_globals)] pub const Db7 : Note = Note::Cs7;
    #[allow(non_upper_case_globals)] pub const Eb7 : Note = Note::Ds7;
    #[allow(non_upper_case_globals)] pub const Gb7 : Note = Note::Fs7;
    #[allow(non_upper_case_globals)] pub const Ab7 : Note = Note::Gs7;
    #[allow(non_upper_case_globals)] pub const Bb7 : Note = Note::As7;
    #[allow(non_upper_case_globals)] pub const Db8 : Note = Note::Cs8;
    #[allow(non_upper_case_globals)] pub const Eb8 : Note = Note::Ds8;
    #[allow(non_upper_case_globals)] pub const Gb8 : Note = Note::Fs8;
    #[allow(non_upper_case_globals)] pub const Ab8 : Note = Note::Gs8;
    #[allow(non_upper_case_globals)] pub const Bb8 : Note = Note::As8;
    #[allow(non_upper_case_globals)] pub const Db9 : Note = Note::Cs9;
    #[allow(non_upper_case_globals)] pub const Eb9 : Note = Note::Ds9;
    #[allow(non_upper_case_globals)] pub const Gb9 : Note = Note::Fs9;
    #[allow(non_upper_case_globals)] pub const Ab9 : Note = Note::Gs9;
}


#[cfg(test)]
mod tests {
    
    use super::*;
    macro_rules! note_test {
        ($($id:ident:$value:expr,)*) => {
            $(
                #[test]
                fn $id() {
                    let (input,expected) = $value;
                    assert_eq!(input as u8, expected);
                }
            )*
        }
    }

    //These test mainly prove we are generating all the sharps/flats
    //(as the same number) correctly.
    note_test! {
            note_c1m:   (Note::C1m,0),
            note_cs1m:  (Note::Cs1m,1),
            note_db1m:  (Note::Db1m,1),
            note_d1m:   (Note::D1m,2),
            note_ds1m:  (Note::Ds1m,3),
            note_eb1m:  (Note::Eb1m,3),
            note_e1m:   (Note::E1m,4),
            note_f1m:   (Note::F1m,5),
            note_fs1m:  (Note::Fs1m,6),
            note_gb1m:  (Note::Gb1m,6),
            note_g1m:   (Note::G1m,7),
            note_gs1m:  (Note::Gs1m,8),
            note_ab1m:  (Note::Ab1m,8),
            note_a1m:   (Note::A1m,9),
            note_as1m:  (Note::As1m,10),
            note_bb1m:  (Note::Bb1m,10),
            note_b1m:   (Note::B1m,11),
            note_c0:    (Note::C0,12),
            note_cs0:   (Note::Cs0,13),
            note_db0:   (Note::Db0,13),
            note_d0:    (Note::D0,14),
            note_ds0:   (Note::Ds0,15),
            note_eb0:   (Note::Eb0,15),
            note_e0:    (Note::E0,16),
            note_f0:    (Note::F0,17),
            note_fs0:   (Note::Fs0,18),
            note_gb0:   (Note::Gb0,18),
            note_g0:    (Note::G0,19),
            note_gs0:   (Note::Gs0,20),
            note_ab0:   (Note::Ab0,20),
            note_a0:    (Note::A0,21),
            note_as0:   (Note::As0,22),
            note_bb0:   (Note::Bb0,22),
            note_b0:    (Note::B0,23),
            note_c1:    (Note::C1,24),
            note_cs1:   (Note::Cs1,25),
            note_db1:   (Note::Db1,25),
            note_d1:    (Note::D1,26),
            note_ds1:   (Note::Ds1,27),
            note_eb1:   (Note::Eb1,27),
            note_e1:    (Note::E1,28),
            note_f1:    (Note::F1,29),
            note_fs1:   (Note::Fs1,30),
            note_gb1:   (Note::Gb1,30),
            note_g1:    (Note::G1,31),
            note_gs1:   (Note::Gs1,32),
            note_ab1:   (Note::Ab1,32),
            note_a1:    (Note::A1,33),
            note_as1:   (Note::As1,34),
            note_bb1:   (Note::Bb1,34),
            note_b1:    (Note::B1,35),
            note_c2:    (Note::C2,36),
            note_cs2:   (Note::Cs2,37),
            note_db2:   (Note::Db2,37),
            note_d2:    (Note::D2,38),
            note_ds2:   (Note::Ds2,39),
            note_eb2:   (Note::Eb2,39),
            note_e2:    (Note::E2,40),
            note_f2:    (Note::F2,41),
            note_fs2:   (Note::Fs2,42),
            note_gb2:   (Note::Gb2,42),
            note_g2:    (Note::G2,43),
            note_gs2:   (Note::Gs2,44),
            note_ab2:   (Note::Ab2,44),
            note_a2:    (Note::A2,45),
            note_as2:   (Note::As2,46),
            note_bb2:   (Note::Bb2,46),
            note_b2:    (Note::B2,47),
            note_c3:    (Note::C3,48),
            note_cs3:   (Note::Cs3,49),
            note_db3:   (Note::Db3,49),
            note_d3:    (Note::D3,50),
            note_ds3:   (Note::Ds3,51),
            note_eb3:   (Note::Eb3,51),
            note_e3:    (Note::E3,52),
            note_f3:    (Note::F3,53),
            note_fs3:   (Note::Fs3,54),
            note_gb3:   (Note::Gb3,54),
            note_g3:    (Note::G3,55),
            note_gs3:   (Note::Gs3,56),
            note_ab3:   (Note::Ab3,56),
            note_a3:    (Note::A3,57),
            note_as3:   (Note::As3,58),
            note_bb3:   (Note::Bb3,58),
            note_b3:    (Note::B3,59),
            note_c4:    (Note::C4,60),
            note_cs4:   (Note::Cs4,61),
            note_db4:   (Note::Db4,61),
            note_d4:    (Note::D4,62),
            note_ds4:   (Note::Ds4,63),
            note_eb4:   (Note::Eb4,63),
            note_e4:    (Note::E4,64),
            note_f4:    (Note::F4,65),
            note_fs4:   (Note::Fs4,66),
            note_gb4:   (Note::Gb4,66),
            note_g4:    (Note::G4,67),
            note_gs4:   (Note::Gs4,68),
            note_ab4:   (Note::Ab4,68),
            note_a4:    (Note::A4,69),
            note_as4:   (Note::As4,70),
            note_bb4:   (Note::Bb4,70),
            note_b4:    (Note::B4,71),
            note_c5:    (Note::C5,72),
            note_cs5:   (Note::Cs5,73),
            note_db5:   (Note::Db5,73),
            note_d5:    (Note::D5,74),
            note_ds5:   (Note::Ds5,75),
            note_eb5:   (Note::Eb5,75),
            note_e5:    (Note::E5,76),
            note_f5:    (Note::F5,77),
            note_fs5:   (Note::Fs5,78),
            note_gb5:   (Note::Gb5,78),
            note_g5:    (Note::G5,79),
            note_gs5:   (Note::Gs5,80),
            note_ab5:   (Note::Ab5,80),
            note_a5:    (Note::A5,81),
            note_as5:   (Note::As5,82),
            note_bb5:   (Note::Bb5,82),
            note_b5:    (Note::B5,83),
            note_c6:    (Note::C6,84),
            note_cs6:   (Note::Cs6,85),
            note_db6:   (Note::Db6,85),
            note_d6:    (Note::D6,86),
            note_ds6:   (Note::Ds6,87),
            note_eb6:   (Note::Eb6,87),
            note_e6:    (Note::E6,88),
            note_f6:    (Note::F6,89),
            note_fs6:   (Note::Fs6,90),
            note_gb6:   (Note::Gb6,90),
            note_g6:    (Note::G6,91),
            note_gs6:   (Note::Gs6,92),
            note_ab6:   (Note::Ab6,92),
            note_a6:    (Note::A6,93),
            note_as6:   (Note::As6,94),
            note_bb6:   (Note::Bb6,94),
            note_b6:    (Note::B6,95),
            note_c7:    (Note::C7,96),
            note_cs7:   (Note::Cs7,97),
            note_db7:   (Note::Db7,97),
            note_d7:    (Note::D7,98),
            note_ds7:   (Note::Ds7,99),
            note_eb7:   (Note::Eb7,99),
            note_e7:    (Note::E7,100),
            note_f7:    (Note::F7,101),
            note_fs7:   (Note::Fs7,102),
            note_gb7:   (Note::Gb7,102),
            note_g7:    (Note::G7,103),
            note_gs7:   (Note::Gs7,104),
            note_ab7:   (Note::Ab7,104),
            note_a7:    (Note::A7,105),
            note_as7:   (Note::As7,106),
            note_bb7:   (Note::Bb7,106),
            note_b7:    (Note::B7,107),
            note_c8:    (Note::C8,108),
            note_cs8:   (Note::Cs8,109),
            note_db8:   (Note::Db8,109),
            note_d8:    (Note::D8,110),
            note_ds8:   (Note::Ds8,111),
            note_eb8:   (Note::Eb8,111),
            note_e8:    (Note::E8,112),
            note_f8:    (Note::F8,113),
            note_fs8:   (Note::Fs8,114),
            note_gb8:   (Note::Gb8,114),
            note_g8:    (Note::G8,115),
            note_gs8:   (Note::Gs8,116),
            note_ab8:   (Note::Ab8,116),
            note_a8:    (Note::A8,117),
            note_as8:   (Note::As8,118),
            note_bb8:   (Note::Bb8,118),
            note_b8:    (Note::B8,119),
            note_c9:    (Note::C9,120),
            note_cs9:   (Note::Cs9,121),
            note_db9:   (Note::Db9,121),
            note_d9:    (Note::D9,122),
            note_ds9:   (Note::Ds9,123),
            note_eb9:   (Note::Eb9,123),
            note_e9:    (Note::E9,124),
            note_f9:    (Note::F9,125),
            note_fs9:   (Note::Fs9,126),
            note_gb9:   (Note::Gb9,126),
            note_g9:    (Note::G9,127),
            note_gs9:   (Note::Gs9,128),
            note_ab9:   (Note::Ab9,128),
    }
}