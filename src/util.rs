pub fn gray_data(image_data: &[u8], width: usize, height: usize) -> Vec<u8> {
    let pixel_count = width * height;
    let mut data = vec![0; pixel_count];
    for i in 0..pixel_count {
        let r = image_data[i * 4] as u32;
        let g = image_data[i * 4 + 1] as u32;
        let b = image_data[i * 4 + 2] as u32;

        let gray = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
        data[i] = gray;
    }
    data
}
pub fn gaussian_kernal(size: usize, sigma: f32) -> Vec<f32> {
    let mut kernel = vec![0.0; size];
    let half = (size >> 1) as f32;
    let mut sum = 0.0;

    for i in 0..size {
        let x = i as f32 - half;
        let g = (-(x * x) / (2.0 * sigma * sigma)).exp();
        kernel[i] = g;
        sum += g;
    }
    for i in 0..size {
        kernel[i] /= sum;
    }
    kernel
}

pub fn filter_image_data(
    image_data: &[u8],
    width: usize,
    height: usize,
    size: usize,
    sigma: f32,
) -> Vec<u8> {
    let pixel_count = (width * height) as usize;
    let kernel = gaussian_kernal(size, sigma);
    let mut data_h = image_data.to_vec();
    println!("kernel: {:?}", kernel);
    for i in 0..(pixel_count) {
        let mut sum = 0.0;
        let matrix = calc_convolution_matrix_horizontal(width, height, i, size);
        for ki in 0..matrix.len() {
            let kdi = matrix[ki];
            sum += kernel[ki] * (image_data[kdi] as f32);
        }
        data_h[i] = (sum.round() as u8).clamp(0, 255); // r
    }

    let mut data = data_h.to_vec();
    for i in 0..(pixel_count) {
        let mut sum = 0.0;
        let matrix = calc_convolution_matrix_vertical(width, height, i, size);
        for ki in 0..matrix.len() {
            let kdi = matrix[ki];
            sum += kernel[ki] * data_h[kdi] as f32;
        }
        data[i] = sum.round().clamp(0.0, 255.0) as u8; // r
    }
    data
}


// pub fn calc_convolution_matrix(width: usize, height: usize, i: usize, size: usize) -> Vec<usize> {
//     // 确保不会发生整数溢出
//     if i >= width * height {
//         panic!("Index out of bounds");
//     }

//     let mut matrix = vec![0; size * size];
//     let half = size / 2;
//     let y = i / width;
//     let x = i % width;

//     for i in 0..size * size {
//         let mx = i % size;
//         let my = i / size;
//         let calc_x = (x + mx).checked_sub(half).unwrap_or(0).clamp(0, width - 1);
//         let calc_y = (y + my).checked_sub(half).unwrap_or(0).clamp(0, height - 1);
//         matrix[i] = calc_y * width + calc_x;
//     }
//     matrix
// }

pub fn calc_convolution_matrix_horizontal(
    width: usize,
    height: usize,
    i: usize,
    size: usize,
) -> Vec<usize> {
    // 确保不会发生整数溢出
    if i >= width * height {
        panic!("Index out of bounds");
    }

    let mut matrix = vec![0; size];
    let half = size / 2;
    let y = i / width;
    let x = i % width;

    for i in 0..size {
        let calc_x = (x + i).checked_sub(half).unwrap_or(0).clamp(0, width - 1);
        matrix[i] = y * width + calc_x;
    }
    matrix
}

pub fn calc_convolution_matrix_vertical(
    width: usize,
    height: usize,
    i: usize,
    size: usize,
) -> Vec<usize> {
    // 确保不会发生整数溢出
    if i >= width * height {
        panic!("Index out of bounds");
    }

    let mut matrix = vec![0; size];
    let half = size / 2;
    let y = i / width;
    let x = i % width;

    for i in 0..size {
        let calc_y = (y + i).checked_sub(half).unwrap_or(0).clamp(0, height - 1);
        matrix[i] = calc_y * width + x;
    }
    matrix
}



#[cfg(test)]
mod tests {
    use crate::util;

    #[test]
    fn test_gray_data() {
        let image_data = [
            10, 10, 10, 255, // line 1
            20, 20, 20, 255, // line 1
            30, 30, 30, 255, // line 1
            40, 40, 40, 255, // line 2
            50, 50, 50, 255, // line 2
            60, 60, 60, 255, // line 2
            70, 70, 70, 255, // line 3
            80, 80, 80, 255, // line 3
            90, 90, 90, 255, // line 3
        ];
        let width = 3;
        let height = 3;
        let result = util::gray_data(&image_data, width, height);
        assert_eq!(
            result,
            [
                10, 20, 30, // line 1
                40, 50, 60, // line 2
                70, 80, 90, // line 3
            ]
        );
    }

    #[test]
    fn test_gaussian_kernal() {
        assert_eq!(1 >> 1, 0);
        assert_eq!(2 >> 1, 1);
        assert_eq!(3 >> 1, 1);
        assert_eq!(4 >> 1, 2);
        assert_eq!(5 >> 1, 2);

        let kernel = util::gaussian_kernal(3, 1.0);
        assert_eq!(kernel[0], 0.27406862);
        assert_eq!(kernel[1], 0.45186275);
        assert_eq!(kernel[2], 0.27406862);
        assert!(kernel.iter().sum::<f32>() - 1.0 < 0.000001);

        let kernel = util::gaussian_kernal(3, 3.0);
        assert!(kernel.iter().sum::<f32>() - 1.0 < 0.000001);

        let kernel = util::gaussian_kernal(5, 1.0);
        assert_eq!(kernel[0], 0.05448869);
        assert_eq!(kernel[1], 0.24420136);
        assert_eq!(kernel[2], 0.40261996);
        assert_eq!(kernel[3], 0.24420136);
        assert_eq!(kernel[4], 0.05448869);
        assert!(kernel.iter().sum::<f32>() - 1.0 < 0.000001);
    }

    #[test]
    fn test_filter_image_data() {
        let image_data = [
            10, 20, 30, // line 1
            40, 50, 60, // line 2
            70, 80, 90, // line 3
        ];
        let width = 3;
        let height = 3;
        let result = util::filter_image_data(&image_data, width, height, 3, 1.0);
        assert_eq!(
            result,
            [
                21, 28, 35, // line 1
                43, 50, 57, // line 2
                65, 72, 79, // line 3
            ]
        )
    }
    
    // #[test]
    // fn test_calc_convolution_matrix() {
    //     // 00, 01, 02, 03, 04, 05,
    //     // 06, 07, 08, 09, 10, 11,
    //     // 12, 13, 14, 15, 16, 17,
    //     // 18, 19, 20, 21, 22, 23
    //     let width = 6;
    //     let height = 4;
    //     let size = 3;
    //     let i = 0;
    //     let result = util::calc_convolution_matrix(width, height, i, size);
    //     println!("{:?}", result);
    //     assert_eq!(result, vec![0, 0, 1, 0, 0, 1, 6, 6, 7]);

    //     let i = 1;
    //     let result = util::calc_convolution_matrix(width, height, i, size);
    //     assert_eq!(result, vec![0, 1, 2, 0, 1, 2, 6, 7, 8]);

    //     let i = 5;
    //     let result = util::calc_convolution_matrix(width, height, i, size);
    //     assert_eq!(result, vec![4, 5, 5, 4, 5, 5, 10, 11, 11]);

    //     let i = 6;
    //     let result = util::calc_convolution_matrix(width, height, i, size);
    //     assert_eq!(result, vec![0, 0, 1, 6, 6, 7, 12, 12, 13]);

    //     let i = 18;
    //     let result = util::calc_convolution_matrix(width, height, i, size);
    //     assert_eq!(result, vec![12, 12, 13, 18, 18, 19, 18, 18, 19]);

    //     let i = 20;
    //     let result = util::calc_convolution_matrix(width, height, i, size);
    //     assert_eq!(result, vec![13, 14, 15, 19, 20, 21, 19, 20, 21]);

    //     let i = 23;
    //     let result = util::calc_convolution_matrix(width, height, i, size);
    //     assert_eq!(result, vec![16, 17, 17, 22, 23, 23, 22, 23, 23]);
    // }

    #[test]
    fn test_calc_convolution_matrix_horizontal() {
        // 00, 01, 02, 03, 04, 05,
        // 06, 07, 08, 09, 10, 11,
        // 12, 13, 14, 15, 16, 17,
        // 18, 19, 20, 21, 22, 23
        let width = 6;
        let height = 4;
        let size = 3;
        let i = 0;
        let result = util::calc_convolution_matrix_horizontal(width, height, i, size);
        assert_eq!(result, vec![0, 0, 1]);

        let i = 1;
        let result = util::calc_convolution_matrix_horizontal(width, height, i, size);
        assert_eq!(result, vec![0, 1, 2]);

        let i = 5;
        let result = util::calc_convolution_matrix_horizontal(width, height, i, size);
        assert_eq!(result, vec![4, 5, 5]);

        let i = 6;
        let result = util::calc_convolution_matrix_horizontal(width, height, i, size);
        assert_eq!(result, vec![6, 6, 7]);

        let i = 18;
        let result = util::calc_convolution_matrix_horizontal(width, height, i, size);
        assert_eq!(result, vec![18, 18, 19]);

        let i = 20;
        let result = util::calc_convolution_matrix_horizontal(width, height, i, size);
        assert_eq!(result, vec![19, 20, 21]);

        let i = 23;
        let result = util::calc_convolution_matrix_horizontal(width, height, i, size);
        assert_eq!(result, vec![22, 23, 23]);
    }

    #[test]
    fn test_calc_convolution_matrix_vertical() {
        // 00, 01, 02, 03, 04, 05,
        // 06, 07, 08, 09, 10, 11,
        // 12, 13, 14, 15, 16, 17,
        // 18, 19, 20, 21, 22, 23
        let width = 6;
        let height = 4;
        let size = 3;
        let i = 0;
        let result = util::calc_convolution_matrix_vertical(width, height, i, size);
        assert_eq!(result, vec![0, 0, 6]);

        let i = 1;
        let result = util::calc_convolution_matrix_vertical(width, height, i, size);
        assert_eq!(result, vec![1, 1, 7]);

        let i = 5;
        let result = util::calc_convolution_matrix_vertical(width, height, i, size);
        assert_eq!(result, vec![5, 5, 11]);

        let i = 6;
        let result = util::calc_convolution_matrix_vertical(width, height, i, size);
        assert_eq!(result, vec![0, 6, 12]);

        let i = 18;
        let result = util::calc_convolution_matrix_vertical(width, height, i, size);
        assert_eq!(result, vec![12, 18, 18]);

        let i = 20;
        let result = util::calc_convolution_matrix_vertical(width, height, i, size);
        assert_eq!(result, vec![14, 20, 20]);

        let i = 23;
        let result = util::calc_convolution_matrix_vertical(width, height, i, size);
        assert_eq!(result, vec![17, 23, 23]);
    }
}
