mod point;

pub use point::MarchPoint;

type Triangle = [MarchPoint; 3];

pub fn march<const N: u8>(mask: u8) -> Vec<Triangle> {
    if mask == 0 {
        Vec::new()
    } else if mask >= (u8::MAX >> (8 - N)) {
        fill::<N>()
    } else {
        triangulate::<N>(mask)
    }
}

fn fill<const N: u8>() -> Vec<Triangle> {
    (0..N - 2).map(|i| [
        MarchPoint::At(0),
        MarchPoint::At(i + 1),
        MarchPoint::At(i + 2),
    ]).collect()
}

fn triangulate<const N: u8>(mask: u8) -> Vec<Triangle> {
    let mut res = Vec::new();

    // Step 1: start at a bit that is one, with a zero bit to the right (wrapping around).
    let mut start_idx: u8 = 0;
    while start_idx < N {
        if bit_at(mask, start_idx) && !bit_at(mask, (start_idx + 1) % N) {
            break;
        }
        start_idx += 1;
    }

    debug_assert!(start_idx < N);

    struct Cluster {
        start: MarchPoint,
        end: MarchPoint,
    }

    let mut num_clusters = 0;
    let mut was_in_cluster = false;
    let mut clusters: Vec<Cluster> = Vec::with_capacity((N / 2) as usize);

    // We start at a one-bit, where the (wrapping) bit to the right is zero
    for idx in (start_idx..N).chain(0..start_idx) {
        if bit_at(mask, idx) {
            let start = MarchPoint::Between(idx, prev_idx::<N>(idx));
            let end = MarchPoint::Between(idx, next_idx::<N>(idx));

            if was_in_cluster {
                // We are already in a cluster, connect to the existing cluster
                let cur_cluster = &mut clusters[num_clusters - 1];

                // Create two triangles from this bit to the join point
                res.push([start, MarchPoint::At(idx), cur_cluster.start]);
                res.push([MarchPoint::At(idx), end, cur_cluster.start]);

                // Update cluster end-point
                cur_cluster.end = end;
            } else {
                // This is the first bit in this cluster, only this one gets a little triangle
                res.push([
                    MarchPoint::At(idx),
                    MarchPoint::Between(idx, next_idx::<N>(idx)),
                    start,
                ]);

                // If there was a previous cluster, connect to it
                if let Some(prev_cluster) = clusters.last() {
                    res.push([start, prev_cluster.start, prev_cluster.end]);
                }

                // Create a new cluster
                num_clusters += 1;
                was_in_cluster = true;
                clusters.push(Cluster { start, end });
            }
        } else {
            was_in_cluster = false;
        }
    }

    if num_clusters > 1 {
        // Connect the first cluster to the last one
        res.push([
            clusters[0].start,
            clusters[num_clusters - 1].start,
            clusters[num_clusters - 1].end,
        ]);

        // Fill in gaps in between clusters
        for idx in (0..num_clusters - 1).step_by(2) {
            res.push([
                clusters[idx + 0].start,
                clusters[idx + 1].start,
                clusters[(idx + 2) % num_clusters].start
            ]);
        }
    }

    res
}

const fn next_idx<const N: u8>(idx: u8) -> u8 {
    (idx + 1) % N
}

const fn prev_idx<const N: u8>(idx: u8) -> u8 {
    (idx + N - 1) % N
}

#[inline]
const fn bit_at(mask: u8, idx: u8) -> bool {
    (mask >> idx) & 1 != 0
}
