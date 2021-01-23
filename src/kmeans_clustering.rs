use rand::Rng;
use rgb::RGB;
use std::cmp::Ordering;

pub trait EucledeanDistance {
    fn eucledean_distance(&self, other: &Self) -> f32;
}

pub trait Centroid: EucledeanDistance {
    fn average(data_points: &Vec<&Self>) -> Self;
}

pub struct Cluster<'a, A: Centroid> {
    centroid: A,
    data_points: Vec<&'a A>,
}

impl<A: Centroid> Cluster<'_, A> {
    fn distance(&self, other: &A) -> f32 {
        self.centroid.eucledean_distance(other)
    }

    fn add(&mut self, other: &A) {
        self.centroid = A::average(&self.data_points);
    }

    fn len(&self) -> usize {
        self.data_points.len()
    }
}

impl<A: Centroid> Ord for Cluster<'_, A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len().cmp(&other.len())
    }
}

fn rndm_select<A: Centroid>(data: &Vec<A>, n_clusters: usize) -> Vec<Cluster<A>> {
    let mut rng = rand::thread_rng();
    let mut i: usize = 0;
    let mut clusters: Vec<Cluster<A>> = Vec::with_capacity(n_clusters);
    let mut centroid: A;
    let mut data_points: Vec<&A>;
    while i < n_clusters {
        unsafe {
            // Safe, because rnd_index is always 0..1, resulting in an index that will always be smaller than the data size
            data_points = vec![data.get_unchecked((rng.gen::<f32>() * data.len() as f32) as usize)];
        }
        centroid = A::average(&data_points);
        clusters.push(Cluster {
            centroid,
            data_points,
        });
        i += 1;
    }
    clusters
}

pub fn find_clusters<A: Centroid>(data: &Vec<A>, n_clusters: usize) -> Vec<Cluster<A>> {
    let mut clusters: Vec<Cluster<A>> = rndm_select(data, n_clusters);
    let mut distance: f32 = f32::MAX;
    let mut curr_distance: f32;
    let mut index_of_closest: usize = 0;
    let mut i: usize = 0;
    for val in data.iter() {
        for cluster in clusters.iter_mut() {
            curr_distance = cluster.distance(val);
            if curr_distance < distance {
                distance = curr_distance;
                index_of_closest = i;
            }
            i += 1;
        }
        unsafe {
            // Safe, because the index_of_closest was initialized from a research inside the vector
            clusters.get_unchecked_mut(index_of_closest).add(val);
        }
    }
    todo!()
}
