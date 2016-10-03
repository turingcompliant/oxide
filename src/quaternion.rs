use matrix::Matrix;

use std::ops::Mul; // multiplication overload

// ============================================================
// Quaternions
// ============================================================
#[derive(Copy,Clone,Debug)]
pub struct Quaternion {
    _contents : [f32;4],
}

impl Quaternion {
    pub fn new (
        in_r : &f32,
        in_i : &f32,
        in_j : &f32,
        in_k : &f32,
    ) -> Quaternion {
        Quaternion {
            _contents : [in_r.clone(), in_i.clone(), in_j.clone(), in_k.clone(), ],
        }
    }
    
    pub fn r(&self) -> &f32 {&self._contents[0]}
    pub fn i(&self) -> &f32 {&self._contents[1]}
    pub fn j(&self) -> &f32 {&self._contents[2]}
    pub fn k(&self) -> &f32 {&self._contents[3]}

    pub fn rotation_matrix (&self) -> Matrix {
        let r = self.r();
        let i = self.i();
        let j = self.j();
        let k = self.k();
        Matrix::new([
            [1.0-2.0*(j*j+k*k),     2.0*(i*j-k*r),     2.0*(k*i+j*r), 0.0],
            [    2.0*(i*j+k*r), 1.0-2.0*(k*k+i*i),     2.0*(j*k-i*r), 0.0],
            [    2.0*(k*i-j*r),     2.0*(j*k+i*r), 1.0-2.0*(i*i+j*j), 0.0],
            [0.0              , 0.0              , 0.0              , 1.0]
        ])
    }

    pub fn normalise (&mut self) {
        let mut norm = 0.0;
        for element in &self._contents {
            norm += element*element;
        }
        norm = norm.sqrt();
        for element in &mut self._contents {
            *element /= norm;
        }
    }

    pub fn invert (&mut self) {
        for element in &mut self._contents[1..4] {
            *element *= -1.0;
        }
    }

    #[allow(dead_code)]
    pub fn right_multiply(&mut self, in_other : &Quaternion) {
        let sr : f32 = self.r().clone();
        let si : f32 = self.i().clone();
        let sj : f32 = self.j().clone();
        let sk : f32 = self.k().clone();
        let or : f32 = in_other.r().clone();
        let oi : f32 = in_other.i().clone();
        let oj : f32 = in_other.j().clone();
        let ok : f32 = in_other.k().clone();
        self._contents = [
            sr*or-si*oi-sj*oj-sk*ok,
            sr*oi+si*or+sj*ok-sk*oj,
            sr*oj+sj*or+sk*oi-si*ok,
            sr*ok+sk*or+si*oj-sj*oi,
        ];
    }
    
    #[allow(dead_code)]
    pub fn left_multiply(&mut self, in_other : &Quaternion) {
        let sr : f32 = self.r().clone();
        let si : f32 = self.i().clone();
        let sj : f32 = self.j().clone();
        let sk : f32 = self.k().clone();
        let or : f32 = in_other.r().clone();
        let oi : f32 = in_other.i().clone();
        let oj : f32 = in_other.j().clone();
        let ok : f32 = in_other.k().clone();
        self._contents = [
            or*sr-oi*si-oj*sj-ok*sk,
            or*si+oi*sr+oj*sk-ok*sj,
            or*sj+oj*sr+ok*si-oi*sk,
            or*sk+ok*sr+oi*sj-oj*si,
        ];
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul (self, in_other : Quaternion) -> Quaternion {
        let sr : f32 = self.r().clone();
        let si : f32 = self.i().clone();
        let sj : f32 = self.j().clone();
        let sk : f32 = self.k().clone();
        let or : f32 = in_other.r().clone();
        let oi : f32 = in_other.i().clone();
        let oj : f32 = in_other.j().clone();
        let ok : f32 = in_other.k().clone();
        Quaternion::new(
            &(sr*or-si*oi-sj*oj-sk*ok),
            &(sr*oi+si*or+sj*ok-sk*oj),
            &(sr*oj+sj*or+sk*oi-si*ok),
            &(sr*ok+sk*or+si*oj-sj*oi),
        )
    }
}
