//! Macros shared across test suites.

/// Checks the bounds of a group.
macro_rules! group {
    ($group: ident, $count: expr,
        [ $id_left: ident, $id_right: ident ],
        mux: [ $mux_left: expr, $mux_right: expr ],
        pad: [ $pad_left: expr, $pad_right: expr ]) => {
        mod $group {
            use super::pads;
            use imxrt_iomuxc::Iomuxc;

            #[test]
            fn erased_pad_count() {
                let erased = unsafe { pads::$group::Pads::new().erase() };
                assert_eq!(erased.len(), $count);
            }

            #[test]
            fn pad_bounds() {
                let mut left = unsafe { pads::$group::$id_left::new() };
                let mut right = unsafe { pads::$group::$id_right::new() };

                assert_eq!(left.mux() as u32, $mux_left);
                assert_eq!(right.mux() as u32, $mux_right);

                assert_eq!(left.pad() as u32, $pad_left);
                assert_eq!(right.pad() as u32, $pad_right);
            }
        }
    };
}
