use crate::vec::Vec3f;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Msaa {
    Disable = 1,
    Oct = 8,
    TwentySeven = 27,
    SixtyFour = 64,
}

impl TryFrom<i32> for Msaa {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Msaa::Disable),
            8 => Ok(Msaa::Oct),
            27 => Ok(Msaa::TwentySeven),
            64 => Ok(Msaa::SixtyFour),
            _ => Err("msaa only support 1/8/27"),
        }
    }
}

lazy_static! {
    static ref DISABLE: [Vec3f; 1] = divided_equally::<1>();
    static ref OCT: [Vec3f; 8] = divided_equally::<8>();
    static ref TWENTY_SEVEN: [Vec3f; 27] = divided_equally::<27>();
    static ref SIXTY_FOUR: [Vec3f; 64] = divided_equally::<64>();
    pub static ref MSAA_OPTIONS: HashMap<Msaa, &'static [Vec3f]> = {
        let mut map = HashMap::new();
        map.insert(Msaa::Disable, DISABLE.as_slice());
        map.insert(Msaa::Oct, OCT.as_slice());
        map.insert(Msaa::TwentySeven, TWENTY_SEVEN.as_slice());
        map.insert(Msaa::SixtyFour, SIXTY_FOUR.as_slice());
        map
    };
}

fn divided_equally<const N: usize>() -> [Vec3f; N] {
    let k = f32::cbrt(N as f32) as usize;
    core::array::from_fn(|i| {
        Vec3f::new(
            (((i / (k * k)) % 2) + 1) as f32 / (k + 1) as f32,
            (((i / k) % 2) + 1) as f32 / (k + 1) as f32,
            (((i) % 2) + 1) as f32 / (k + 1) as f32,
        )
    })
}
