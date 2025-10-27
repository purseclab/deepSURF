#[cfg(test)]
mod toodee_tests_sort {
    
    use crate::*;
    use rand::Rng;
    use rand::distributions::Uniform;
    
    #[test]
    fn sort_by_row() {
        let rng = rand::thread_rng();
        let samples = rng.sample_iter(Uniform::new(0,100));
        let mut toodee = TooDee::from_vec(10, 10, samples.take(100).collect());
        println!("{:?}",&toodee[3]);
        toodee.sort_by_row(3, |a, b| a.cmp(b));
        let r = &toodee[3];
        println!("{:?}",r);
        for i in 0..9 {
            assert!(r[i] <= r[i+1]);
        }
    }
        
    #[test]
    fn sort_by_col() {
        let rng = rand::thread_rng();
        let samples = rng.sample_iter(Uniform::new(0,100));
        let mut toodee = TooDee::from_vec(10, 10, samples.take(100).collect());
        println!("{:?}", toodee.col(3).copied().collect::<Vec<u32>>());
        toodee.sort_by_col(3, |a, b| a.cmp(b));
        let c : Vec<u32> = toodee.col(3).copied().collect();
        println!("{:?}",c);
        for i in 0..9 {
            assert!(c[i] <= c[i+1]);
        }
    }
    
    #[test]
    fn sort_by_row_view() {
        let rng = rand::thread_rng();
        let samples = rng.sample_iter(Uniform::new(0,100));
        let mut toodee = TooDee::from_vec(10, 10, samples.take(100).collect());
        println!("{:?}",&toodee[3]);
        toodee.view_mut((0, 0), (10, 10)).sort_by_row(3, |a, b| a.cmp(b));
        let r = &toodee[3];
        println!("{:?}",r);
        for i in 0..9 {
            assert!(r[i] <= r[i+1]);
        }
    }

    #[test]
    fn sort_by_col_view() {
        let rng = rand::thread_rng();
        let samples = rng.sample_iter(Uniform::new(0,100));
        let mut toodee = TooDee::from_vec(10, 10, samples.take(100).collect());
        println!("{:?}", toodee.col(3).copied().collect::<Vec<u32>>());
        toodee.view_mut((0, 0), (10, 10)).sort_by_col(3, |a, b| a.cmp(b));
        let c : Vec<u32> = toodee.col(3).copied().collect();
        println!("{:?}",c);
        for i in 0..9 {
            assert!(c[i] <= c[i+1]);
        }
    }

    
}
