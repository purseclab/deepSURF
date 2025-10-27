#[cfg(test)]
mod toodee_tests_copy {

    use crate::*;

    #[test]
    fn copy_within_1() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        toodee.copy_within(((0, 0), (2, 2)), (8, 8));
        let orig = (100*100 - 100) / 2;
        assert_eq!(toodee.data().iter().sum::<u32>(), orig - 98-99-88-89+1+10+11);
    }
    
    #[test]
    fn copy_within_2() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        toodee.copy_within(((8, 8), (10, 10)), (0, 0));
        let orig = (100*100 - 100) / 2;
        assert_eq!(toodee.data().iter().sum::<u32>(), orig +98+99+88+89-1-10-11);
    }

    #[test]
    fn copy_within_overlap_1() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        toodee.copy_within(((0, 0), (2, 2)), (1, 1));
        let orig = (100*100 - 100) / 2;
        assert_eq!(toodee.data().iter().sum::<u32>(), orig -11-12-21-22 +1+10+11);
    }

    #[test]
    fn copy_within_overlap_2() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        toodee.copy_within(((1, 1), (3, 3)), (0, 0));
        let orig = (100*100 - 100) / 2;
        assert_eq!(toodee.data().iter().sum::<u32>(), orig +11+12+21+22 -1-10-11);
    }

    #[test]
    fn copy_from_toodee() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let mut dest : TooDee<u32> = TooDee::new(10, 10);
        dest.copy_from_toodee(&toodee);
        assert_eq!(dest.data().iter().sum::<u32>(), (100*100 - 100) / 2);
    }

    #[test]
    fn clone_from_toodee() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let mut dest : TooDee<u32> = TooDee::new(10, 10);
        dest.clone_from_toodee(&toodee);
        assert_eq!(dest.data().iter().sum::<u32>(), (100*100 - 100) / 2);
    }

    #[test]
    fn view_copy_from_toodee() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let mut dest : TooDee<u32> = TooDee::new(10, 10);
        dest.view_mut((0, 0), (10, 10)).copy_from_toodee(&toodee);
        assert_eq!(dest.data().iter().sum::<u32>(), (100*100 - 100) / 2);
    }

    #[test]
    fn view_clone_from_toodee() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let mut dest : TooDee<u32> = TooDee::new(10, 10);
        dest.view_mut((0, 0), (10, 10)).clone_from_toodee(&toodee);
        assert_eq!(dest.data().iter().sum::<u32>(), (100*100 - 100) / 2);
    }

}
