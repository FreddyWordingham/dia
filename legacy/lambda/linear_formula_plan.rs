use super::Lambda;
use crate::utility::{Order, Range};
use dbc::{dbc_panic, require};
use ndarray::Array1;

pub struct Linear {
    xns: Array1<f64>,
    yns: Array1<f64>,
    grads: Array1<f64>,
    range: Range,
}

impl Linear {
    pub fn new(xns: Array1<f64>, yns: Array1<f64>) -> Self {
        require!(xns.len() >= 2);
        require!(xns.len() == yns.len());
        require!(xns.is_ascending());

        let mut grads = Vec::with_capacity(xns.len() - 1);
        for (curr_xn, (next_xn, (curr_yn, next_yn))) in xns
            .iter()
            .zip(xns.iter().skip(1).zip(yns.iter().zip(yns.iter().skip(1))))
        {
            let delta_x = next_xn - curr_xn;
            let delta_y = next_yn - curr_yn;

            println!("grad: {}", delta_y / delta_x);

            grads.push(delta_y / delta_x);
        }

        let range = Range::new(xns[0], xns[xns.len() - 1]);

        Self {
            xns,
            yns,
            grads: Array1::from_vec(grads),
            range,
        }
    }

    pub fn range(&self) -> &Range {
        &self.range
    }
}

impl Lambda for Linear {
    fn y(&self, x: f64) -> f64 {
        require!(self.range.contains(x));

        for (xn, (yn, grad)) in self
            .xns
            .iter()
            .skip(1)
            .zip(self.yns.iter().skip(1).zip(self.grads.iter()))
        {
            if x <= *xn {
                println!("{} -> {}", x, *xn);
                let delta = x - *xn;

                return *yn + (delta * grad);
            }
        }

        unreachable!();
    }
}
