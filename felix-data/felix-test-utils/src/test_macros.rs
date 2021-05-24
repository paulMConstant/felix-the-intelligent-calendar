/// # Parameters
///
/// $data: ident, $builder: expr, $test\_block: expr, $expected\_error: literal, $failure\_message:
/// literal
#[macro_export]
macro_rules! test_err {
    ($data: ident, $builder: expr, $test_block: expr, $expected_error: literal, $failure_message: literal) => {
        #[allow(unused_mut)]
        let mut $data = $builder.into_data();
        assert_not_modified!($data, {
            if let Err(e) = $test_block {
                assert_eq!(
                    e.to_string(),
                    $expected_error,
                    "The error message is not the one we expected."
                );
            } else {
                panic!($failure_message);
            }
        });
    };
}

/// # Parameters
///
/// $data: ident, $builder: expr, $test\_block: expr
#[macro_export]
macro_rules! test_ok {
    ($data: ident, $builder: expr, $test_block: expr) => {
        #[allow(unused_mut)]
        let mut $data = $builder.into_data();
        $test_block;
    };
}
