use core::f32;

#[derive(Copy, Clone, Default)]
pub struct Mask<const N: usize>(u8);

#[derive(Copy, Clone, Debug)]
pub enum MarchPoint {
    At(u8),
    Between(u8, u8),
}

impl MarchPoint {
    fn shift_back(&mut self, amount: u8, n: u8) {
        match self {
            MarchPoint::At(x) => *x = (*x + n - amount) % n,
            MarchPoint::Between(x, y) => {
                *x = (*x + n - amount) % n;
                *y = (*y + n - amount) % n;
            },
        }
    }
}

impl<const N: usize> Mask<N> {
    pub const MAX: u8 = u8::MAX >> (8 - N);

    pub const ANGLE: f32 = 2.0 * f32::consts::PI / N as f32;

    pub fn new(mask: u8) -> Self {
        assert!(mask <= Self::MAX);
        Self { 0: mask }
    }

    pub fn march(self) -> Vec<[MarchPoint; 3]> {
        debug_assert!(self.0 <= Self::MAX);

        if self.0 == 0 {
            return Vec::with_capacity(0);
        }

        if self.0 == Self::MAX {
            return Self::fill().to_vec();
        }

        let mut res = Vec::new();

        let mut mask = self;

        // Step 1: wrap-shift bits to the right until there is a zero at the left and a one at the right
        let mut num_shifts: u8 = 0;
        while mask.leftmost_bit() || !mask.rightmost_bit() {
            debug_assert!(num_shifts < N as u8 - 1, "mask seems to have no/only zeros: {:#08b}", mask.0);
            mask = mask.shift_one();
            num_shifts += 1;
        }

        struct OnesCluster {
            end_idx: u8,
            join_point: MarchPoint,
            end_point: MarchPoint,
        }

        // At this point we know that, starting at the right-most bit, that bit is one, at the bit
        // wrapping around to the left is zero.
        let mut cur_idx = 0;
        let mut was_in_cluster = false;
        let mut clusters: Vec<OnesCluster> = Vec::with_capacity(N / 2);

        while cur_idx < N as u8 - 1 {
            if mask.is_set(cur_idx) {
                if was_in_cluster {
                    // We are already in a cluster, connect to the existing cluster
                    let current_cluster = clusters.last_mut().unwrap();

                    // Create two triangles from ourselve to the join point
                    // We don't have to worry about overflow or underflow here.
                    res.push([
                        MarchPoint::Between(cur_idx, cur_idx - 1),
                        MarchPoint::At(cur_idx),
                        current_cluster.join_point,
                    ]);

                    res.push([
                        MarchPoint::At(cur_idx),
                        MarchPoint::Between(cur_idx, cur_idx + 1),
                        current_cluster.join_point,
                    ]);

                    debug_assert_eq!(current_cluster.end_idx, cur_idx - 1);
                    current_cluster.end_idx = cur_idx;
                    current_cluster.end_point = MarchPoint::Between(cur_idx, cur_idx + 1);
                } else {
                    // We need to wrap correctly if our index is zero (the initial case)
                    let prev_idx = if cur_idx == 0 { N as u8 - 1 } else { cur_idx - 1 };

                    let join_point = MarchPoint::Between(cur_idx, prev_idx);
                    let end_point = MarchPoint::Between(cur_idx, cur_idx + 1);

                    // We are the first one, create a little triangle
                    res.push([
                        MarchPoint::At(cur_idx),
                        MarchPoint::Between(cur_idx, cur_idx + 1),
                        join_point,
                    ]);

                    // If there was a previous cluster, connect to it
                    if let Some(prev_cluster) = clusters.last() {
                        res.push([
                            join_point,
                            prev_cluster.join_point,
                            prev_cluster.end_point,
                        ]);
                    }

                    // Add the new cluster
                    let new_cluster = OnesCluster {
                        end_idx: cur_idx,
                        join_point,
                        end_point,
                    };

                    clusters.push(new_cluster);
                }

                was_in_cluster = true;
            } else {
                // We left the cluster
                debug_assert!(!clusters.is_empty());
                was_in_cluster = false;
            }

            cur_idx += 1;
        }

        // if there are two or more clusters, connect the first cluster to the last one
        if true {
            if clusters.len() > 1 {
                let first_cluster = clusters.first().unwrap();
                let last_cluster = clusters.last().unwrap();

                res.push([
                    first_cluster.join_point,
                    last_cluster.join_point,
                    last_cluster.end_point,
                ]);
            }
        }

        // fill in the gap in the middle. We need num_clusters - 2 triangles
        if true {
            let mut idx = 0;
            while idx < clusters.len() - 1 {
                let c1 = &clusters[idx + 0];
                let c2 = &clusters[idx + 1];
                let c3 = &clusters[(idx + 2) % clusters.len()];
                res.push([
                    c1.join_point,
                    c2.join_point,
                    c3.join_point,
                ]);
                idx += 2;
            }
        }

        // shift indices back to where they were
        for [a, b, c] in &mut res {
            a.shift_back(num_shifts, N as u8);
            b.shift_back(num_shifts, N as u8);
            c.shift_back(num_shifts, N as u8);
        }

        res
    }

    const fn fill() -> [[MarchPoint; 3]; N] {
        let mut res = [[MarchPoint::At(0); 3]; N];

        let mut i = 0;
        while i < N - 2 {
            res[i][0] = MarchPoint::At(0);
            res[i][1] = MarchPoint::At(i as u8 + 1);
            res[i][2] = MarchPoint::At(i as u8 + 2);
            i += 1;
        }

        res
    }

    const fn is_set(self, idx: u8) -> bool {
        ((self.0 >> idx) & 1) != 0
    }

    const fn shift_one(self) -> Self {
        let cy = self.0 & 0b00000001;
        Self { 0: (self.0 >> 1) | (cy << (N - 1)) }
    }

    const fn leftmost_bit(self) -> bool {
        ((self.0 >> (N - 1)) & 0b00000001) != 0
    }

    const fn rightmost_bit(self) -> bool {
        (self.0 & 0b00000001) != 0
    }
}

impl<const N: usize> std::fmt::Debug for Mask<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0b{:0>N$b}", self.0)
    }
}
