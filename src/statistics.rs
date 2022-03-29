pub fn average(v: &Vec<i32>) -> i32 {
    (v.iter().sum::<i32>() as f64 / v.len() as f64) as i32
}

pub fn outlier_average(v: &mut Vec<i32>, outlier_percentile: f32, min_outliers: Option<&mut Vec<i32>>, max_outliers: Option<&mut Vec<i32>>) -> i32 {
    v.sort();

    let num_outliers = (v.len() as f32 * outlier_percentile) as i32;

    let mut sum = 0;
    for n in num_outliers..(v.len() as i32 - num_outliers) {
        sum += v[n as usize];
    }

    if min_outliers.is_some() {
        let min_outliers = min_outliers.unwrap();
        for n in 0..num_outliers {
            min_outliers.push(v[n as usize]);
        }
    }

    if max_outliers.is_some() {
        let max_outliers = max_outliers.unwrap();
        for n in (v.len() as i32 - num_outliers)..(v.len() as i32) {
            max_outliers.push(v[n as usize]);
        }
    }

    (sum as f64 / v.len() as f64) as i32
}

pub fn variance(v: &Vec<i32>) -> f32 {
    let avg = average(v);

    let mut v_dev: Vec<i32> = Vec::new();

    for n in v {
        v_dev.push((n - avg).pow(2));
    }

    v_dev.iter().sum::<i32>() as f32 / v_dev.len() as f32
}

pub fn deviation(v: &Vec<i32>) -> f32 {
    variance(v).sqrt()
}

pub fn deviation_average(v: &mut Vec<i32>) -> f32 {
    let deviation = deviation(v);
    let avg = average(v) as f32;

    let mut v_nodev: Vec<i32> = Vec::new();

    for n in v {
        if (*n as f32) < avg + deviation && (*n as f32) > avg - deviation {
            // Acceptable deviation
            v_nodev.push(*n);
        }
    }

    v_nodev.iter().sum::<i32>() as f32 / v_nodev.len() as f32
}

pub fn min(v: &Vec<i32>) -> i32 {
    let mut cmin = i32::MAX;
    for n in v {
        if *n < cmin {
            cmin = *n;
        }
    }

    cmin
}

pub fn max(v: &Vec<i32>) -> i32 {
    let mut cmax = i32::MIN;
    for n in v {
        if *n < cmax {
            cmax = *n;
        }
    }

    cmax
}