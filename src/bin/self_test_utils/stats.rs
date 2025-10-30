
use meansd::MeanSD;
use single_statistics::testing::{inference::parametric::t_test, TTestType, Alternative, TestResult};

// Mean and std dev.

pub fn mean_stddev(data: &[f64]) -> (f64, f64) {
    let mut msd = MeanSD::default();
    
    for point in data {
        msd.update(*point);
    }

    (msd.mean(), msd.sstdev())
}

#[test]
fn test_mean_stddev() {
    let (mean, stddev) = mean_stddev(&[1.0, 2.0, 4.4, 4.9, 4.71]);
    assert!((stddev - 1.7808761888464).abs() < 0.001);
    assert!((mean - 3.402).abs() < 0.001);
}

// This test exists in part to test the T-test function of the statistics crate
// I use, because it's small and not widely used and I don't trust statistics
// crates after three others all failed me in wild ways.

pub fn two_sided_welch_t_test(dataset1: &[f64], dataset2: &[f64]) -> TestResult<f64> {
    t_test::<f64>(
        dataset1,
        dataset2,
        TTestType::Welch,
        Alternative::TwoSided,
    )
}

#[test]
fn test_t_test() {
    let set1 = vec![2.9, 3.0, 8.4, 17.4, 100.0];
    let set2 = vec![4.4, 4.4, 4.2, 4.17];

    let (mean1, stddev1) = mean_stddev(&set1);
    let (mean2, stddev2) = mean_stddev(&set2);

    let var1 = stddev1 * stddev1;
    let var2 = stddev2 * stddev2;

    let expected_t = (mean1 - mean2) / (var1 / set1.len() as f64 + var2 / set2.len() as f64).sqrt();
    let expected_p = 0.3016; // Calculated by hand after seeing the T value.

    let actual = two_sided_welch_t_test(&set1, &set2);

    dbg!(expected_t);
    dbg!(actual.statistic);
    dbg!(expected_p);
    dbg!(actual.p_value);

    assert!((actual.p_value - expected_p).abs() < 0.001);
    assert!((actual.statistic - expected_t).abs() < 0.001);
}

