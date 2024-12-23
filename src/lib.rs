mod march_indices;

use march_indices::*;

#[derive(Copy, Clone)]
pub struct Mask<const N: usize> {
    mask: u8,
}

impl<const N: usize> Mask<N> {
    const MAX: u8 = u8::MAX >> (8 - N);

    pub const fn new(mask: u8) -> Self {
        assert!(mask <= Self::MAX);
        Self { mask }
    }

    pub fn march(self) -> MarchIndices {
        debug_assert!(self.mask <= Self::MAX);

        if self.mask == 0 {
            return MarchIndices::default();
        }

        if self.mask == Self::MAX {
            return MarchIndices::filled(N as u8);
        }

        let mut res: MarchIndices = MarchIndices::default();

        // Step 1: start at a bit that is one, with a zero bit to the right (wrapping around).
        let mut start_idx: u8 = 0;
        while start_idx < N as u8 {
            if self.bit_at(start_idx) && !self.bit_at((start_idx + 1) % N as u8) {
                break;
            }

            start_idx += 1;
        }

        debug_assert!(start_idx < N as u8);

        struct OnesCluster {
            end_idx: u8,
            join_point: MarchPoint,
            end_point: MarchPoint,
        }

        // At this point we know that the current bit is one, at the bit (wrapping around) to the left is zero.
        let mut was_in_cluster = false;
        let mut clusters: Vec<OnesCluster> = Vec::with_capacity(N / 2);

        for cur_idx in (start_idx..N as u8).chain(0..start_idx) {
            if self.bit_at(cur_idx) {
                if was_in_cluster {
                    // We are already in a cluster, connect to the existing cluster
                    let current_cluster = clusters.last_mut().unwrap();

                    // Create two triangles from ourselve to the join point
                    res.add_triangle(
                        MarchPoint::Between(cur_idx, (cur_idx + N as u8 - 1) % N as u8),
                        MarchPoint::At(cur_idx),
                        current_cluster.join_point,
                    );

                    res.add_triangle(
                        MarchPoint::At(cur_idx),
                        MarchPoint::Between(cur_idx, (cur_idx + 1) % N as u8),
                        current_cluster.join_point,
                    );

                    debug_assert_eq!(current_cluster.end_idx, (cur_idx + N as u8 - 1) % N as u8);
                    current_cluster.end_idx = cur_idx;
                    current_cluster.end_point = MarchPoint::Between(cur_idx, (cur_idx + 1) % N as u8);
                } else {
                    let join_point = MarchPoint::Between(cur_idx, (cur_idx + N as u8 - 1) % N as u8);
                    let end_point = MarchPoint::Between(cur_idx, (cur_idx + 1) % N as u8);

                    // We are the first one, create a little triangle
                    res.add_triangle(
                        MarchPoint::At(cur_idx),
                        MarchPoint::Between(cur_idx, (cur_idx + 1) % N as u8),
                        join_point,
                    );

                    // If there was a previous cluster, connect to it
                    if true /* self.fill */ {
                        if let Some(prev_cluster) = clusters.last() {
                            res.add_triangle(
                                join_point,
                                prev_cluster.join_point,
                                prev_cluster.end_point,
                            );
                        }
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
                // We just left the cluster
                debug_assert!(!clusters.is_empty());
                was_in_cluster = false;
            }
        }

        // if there are two or more clusters, connect the first cluster to the last one
        if true /* self.fill */ {
            if clusters.len() > 1 {
                let first_cluster = clusters.first().unwrap();
                let last_cluster = clusters.last().unwrap();

                res.add_triangle(
                    first_cluster.join_point,
                    last_cluster.join_point,
                    last_cluster.end_point,
                );
            }
        }

        // fill in the gap in the middle. We need num_clusters - 2 triangles
        if true /* self.fill */ {
            let mut idx = 0;
            while idx < clusters.len() - 1 {
                let c1 = &clusters[idx + 0];
                let c2 = &clusters[idx + 1];
                let c3 = &clusters[(idx + 2) % clusters.len()];
                res.add_triangle(c1.join_point, c2.join_point, c3.join_point);
                idx += 2;
            }
        }

        res
    }

    const fn bit_at(self, idx: u8) -> bool {
        ((self.mask >> idx) & 1) != 0
    }
}

impl<const N: usize> std::fmt::Debug for Mask<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0b{:0>N$b}", self.mask)
    }
}
