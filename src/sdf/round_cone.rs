use super::sdf::{Hit, SDF};
use crate::vec::{self, Vec3f};

pub struct RoundCone {
    a: Vec3f,
    b: Vec3f,
    ra: f32,
    rb: f32,
    bounding_box: (Vec3f, Vec3f),
}

impl RoundCone {
    pub fn new(a: Vec3f, ra: f32, b: Vec3f, rb: f32) -> RoundCone {
        let (min_a, min_b) = (a - ra, b - rb);
        let (max_a, max_b) = (a + ra, b + rb);
        let bounding_box = (vec::minimum(min_a, min_b), vec::maximum(max_a, max_b));
        Self {
            a,
            b,
            ra,
            rb,
            bounding_box,
        }
    }
}

impl SDF for RoundCone {
    fn signed_distance(&self, p: Vec3f) -> f32 {
        sd_round_cone(p, self.a, self.b, self.ra, self.rb)
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.bounding_box
    }

    fn hit(&self, p: Vec3f) -> Hit {
        let sd = self.signed_distance(p);
        Hit {
            signed_distance: sd,
            u: sd.clamp(0.0, 1.0),
            v: proj_p_to_line(p, self.a, self.b).clamp(0.0, 1.0),
        }
    }
}

fn sd_round_cone(p: Vec3f, a: Vec3f, b: Vec3f, ra: f32, rb: f32) -> f32 {
    // sampling independent computations (only depend on shape)
    let ba = b - a;
    let l2 = vec::dot(ba, ba);
    let rr = ra - rb;
    let a2 = l2 - rr * rr;
    let il2 = 1.0 / l2;

    // sampling dependant computations
    let pa = p - a;
    let y = vec::dot(pa, ba);
    let z = y - l2;
    let x2 = (pa * l2 - ba * y).dot2();
    let y2 = y * y * l2;
    let z2 = z * z * l2;

    // single square root!
    let k = f32::signum(rr) * rr * rr * x2;
    if f32::signum(z) * a2 * z2 > k {
        f32::sqrt(x2 + z2) * il2 - rb
    } else if f32::signum(y) * a2 * y2 < k {
        f32::sqrt(x2 + y2) * il2 - ra
    } else {
        (f32::sqrt(x2 * a2 * il2) + y * rr) * il2 - ra
    }
}

fn proj_p_to_line(p: Vec3f, a: Vec3f, b: Vec3f) -> f32 {
    let ap = p - a;
    let ab = b - a;
    let ao = a + ab * (vec::dot(ap, ab) / vec::dot(ab, ab)); // proj P to AB in O
    vec::dot(ao, ab).signum() * ao.norm() / ab.norm() // O = A + k * (AB)
}
