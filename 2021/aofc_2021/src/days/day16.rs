use std::{collections::VecDeque, fs};

use self::buildable_number::BuildableNumber;

#[derive(Debug, Clone)]
enum PacketVal {
    #[allow(dead_code)]
    None,
    Literal(u64),
    SubPackets(Vec<Packet>),
}

impl PacketVal {
    fn eval(&self, type_id: u8) -> u64 {
        match self {
            PacketVal::None => unreachable!(),
            PacketVal::Literal(val) => *val,
            PacketVal::SubPackets(packs) => match type_id {
                0 => packs.iter().fold(0, |f, val| f + val.eval()),
                1 => packs.iter().fold(1, |f, val| f * val.eval()),
                2 => packs
                    .iter()
                    .skip(1)
                    .fold(packs[0].eval(), |fold_val, val| fold_val.min(val.eval())),
                3 => packs
                    .iter()
                    .skip(1)
                    .fold(packs[0].eval(), |fold_val, val| fold_val.max(val.eval())),
                5 => (packs[0].eval() > packs[1].eval()) as u64,
                6 => (packs[0].eval() < packs[1].eval()) as u64,
                7 => (packs[0].eval() == packs[1].eval()) as u64,

                _ => unreachable!(),
            },
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Packet {
    pub type_id: u8,
    pub version_id: u8,
    pub val: PacketVal,
}

impl Packet {
    fn new(version_id: u8, type_id: u8, val: PacketVal) -> Self {
        Self {
            type_id,
            version_id,
            val,
        }
    }

    fn eval(&self) -> u64 {
        self.val.eval(self.type_id)
    }
}

struct IteratorPacketBuilder {
    bits: VecDeque<u8>,
}

impl IteratorPacketBuilder {
    fn new(bits: VecDeque<u8>) -> Self {
        Self { bits }
    }

    fn build_num<T>(&mut self, bits: u8) -> Option<T>
    where
        T: BuildableNumber,
    {
        let mut comp_bin_number = String::new();
        for _ in 0..bits {
            comp_bin_number.push((self.bits.pop_front()? + 0x30) as char);
        }
        T::from_str_radix(comp_bin_number.as_str(), 2)
    }

    fn build_lit_val<T: BuildableNumber>(&mut self) -> Option<T> {
        let mut comp_bin_number = String::new();
        loop {
            let first_bit = self.bits.pop_front()?;
            for _ in 0..4 {
                comp_bin_number.push((self.bits.pop_front()? + 0x30) as char);
            }
            if first_bit == 0 {
                break;
            }
        }
        T::from_str_radix(comp_bin_number.as_str(), 2)
    }

    fn build_op_pack(&mut self) -> Option<PacketVal> {
        let first_bit = self.bits.pop_front()?;
        match first_bit {
            0 => {
                let num_of_bits = self.build_num(15)?;
                let sub_bits = self.bits.drain(0..num_of_bits).collect();
                Some(PacketVal::SubPackets(
                    IteratorPacketBuilder::new(sub_bits).collect(),
                ))
            }
            1 => {
                let num_of_sub_packets: usize = self.build_num(11)?;
                let mut sub_iter = IteratorPacketBuilder::new(self.bits.drain(0..).collect());
                #[allow(clippy::needless_borrow)]
                let sub_packs = (&mut sub_iter).take(num_of_sub_packets).collect();
                self.bits.append(&mut sub_iter.into());
                Some(PacketVal::SubPackets(sub_packs))
            }
            _ => unreachable!(),
        }
    }
}

impl From<IteratorPacketBuilder> for VecDeque<u8> {
    fn from(other: IteratorPacketBuilder) -> Self {
        other.bits
    }
}

impl Iterator for IteratorPacketBuilder {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        let version_id = self.build_num(3)?;
        let type_id = self.build_num(3)?;
        let pack_val = if type_id == 4 {
            PacketVal::Literal(self.build_lit_val()?)
        } else {
            self.build_op_pack()?
        };

        Some(Packet::new(version_id, type_id, pack_val))
    }
}

mod buildable_number {

    macro_rules! impl_buildable_number {


        ($t:ty) => {
            impl BuildableNumber for $t {
                fn from_str_radix<S: AsRef<str>>(src: S, radix: u32) -> Option<$t> {
                    match <$t>::from_str_radix(src.as_ref(), radix) {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    }
                }
            }
        };

        ($($t:ty),*) =>{
            $(
                impl_buildable_number!($t);
            )*
        };
    }

    pub trait BuildableNumber
    where
        Self: Sized,
    {
        fn from_str_radix<S: AsRef<str>>(src: S, radix: u32) -> Option<Self>;
    }

    impl_buildable_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128);
}

pub fn day16_1(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );

    let packets = IteratorPacketBuilder::new(
        fs::read_to_string(input_file)
            .unwrap()
            .split("")
            .filter_map(|c| match u8::from_str_radix(c, 16) {
                Ok(v) => Some(format!("{:04b}", v)),
                Err(_) => None,
            })
            .fold("".into(), |built, this| format!("{}{}", built, this))
            .split("")
            .filter_map(|c| match u8::from_str_radix(c, 2) {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .collect::<VecDeque<_>>(),
    )
    .collect::<Vec<Packet>>();

    fn rec_get_sum(packs: Vec<Packet>) -> u32 {
        let mut sum = 0;
        for packet in &packs {
            sum += match &packet.val {
                PacketVal::None => panic!("why is this happening?"),
                PacketVal::Literal(_) => packet.version_id as u32,
                PacketVal::SubPackets(subs) => packet.version_id as u32 + rec_get_sum(subs.clone()),
            };
        }
        sum
    }

    rec_get_sum(packets)
}

pub fn day16_2(file_name: &str) -> impl crate::AnsType {
    let input_file = format!(
        "{}/aofc_2021/input/{}",
        env!("ADVENT_OF_CODE_2021"),
        file_name
    );
    let packets = IteratorPacketBuilder::new(
        fs::read_to_string(input_file)
            .unwrap()
            .split("")
            .filter_map(|c| match u8::from_str_radix(c, 16) {
                Ok(v) => Some(format!("{:04b}", v)),
                Err(_) => None,
            })
            .fold("".into(), |built, this| format!("{}{}", built, this))
            .split("")
            .filter_map(|c| match u8::from_str_radix(c, 2) {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .collect::<VecDeque<_>>(),
    )
    .collect::<Vec<_>>();

    packets[0].eval()
}
