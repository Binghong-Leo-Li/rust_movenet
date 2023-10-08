use opencv::core::{Point, Scalar};
use opencv::imgproc::LINE_AA;
use opencv::{core::*, imgproc::*};

pub fn resize_with_padding(img: &Mat, new_shape: [i32; 2]) -> Mat {
    let img_shape = [img.cols(), img.rows()];
    let width: i32;
    let height: i32;
    if img_shape[0] as f64 / img_shape[1] as f64 > new_shape[0] as f64 / new_shape[1] as f64 {
        width = new_shape[0];
        height = (new_shape[0] as f64 / img_shape[0] as f64 * img_shape[1] as f64) as i32;
    } else {
        width = (new_shape[1] as f64 / img_shape[1] as f64 * img_shape[0] as f64) as i32;
        height = new_shape[1];
    }

    let mut resized = Mat::default();
    resize(
        img,
        &mut resized,
        Size { width, height },
        0.0,
        0.0,
        INTER_LINEAR,
    )
    .expect("resize_with_padding: resize [FAILED]");

    let delta_w = new_shape[0] - width;
    let delta_h = new_shape[1] - height;
    let (top, bottom) = (delta_h / 2, delta_h - delta_h / 2);
    let (left, right) = (delta_w / 2, delta_w - delta_w / 2);

    let mut rslt = Mat::default();
    copy_make_border(
        &resized,
        &mut rslt,
        top,
        bottom,
        left,
        right,
        BORDER_CONSTANT,
        Scalar::new(0.0, 0.0, 0.0, 0.0),
    )
    .expect("resize_with_padding: copy_make_border [FAILED]");
    rslt
}

pub fn draw_keypoints(img: &mut Mat, keypoints: &[f32], threshold: f32) {
    // keypoints: [1, 17, 3]
    let base: f32;
    let pad_x: i32;
    let pad_y: i32;
    if img.rows() > img.cols() {
        base = img.rows() as f32;
        pad_x = (img.rows() - img.cols()) / 2;
        pad_y = 0;
    } else {
        base = img.cols() as f32;
        pad_x = 0;
        pad_y = (img.cols() - img.rows()) / 2;
    }

    for index in 0..17 {
        let y_ratio = keypoints[index * 3];
        let x_ratio = keypoints[index * 3 + 1];
        let confidence = keypoints[index * 3 + 2];
        if confidence > threshold {
            circle(
                img,
                Point {
                    x: (x_ratio * base) as i32 - pad_x,
                    y: (y_ratio * base) as i32 - pad_y,
                },
                0,
                Scalar::new(0.0, 255.0, 0.0, 0.0),
                5,
                LINE_AA,
                0,
            )
            .expect("Draw circle [FAILED]");
        }
    }
}

pub fn draw_connections(
    img: &mut Mat,
    keypoints: &[f32],
    edges: &[(usize, usize)],
    threshold: f32,
) {
    // keypoints: [1, 17, 3]
    let base: f32;
    let pad_x: i32;
    let pad_y: i32;
    if img.rows() > img.cols() {
        base = img.rows() as f32;
        pad_x = (img.rows() - img.cols()) / 2;
        pad_y = 0;
    } else {
        base = img.cols() as f32;
        pad_x = 0;
        pad_y = (img.cols() - img.rows()) / 2;
    }

    for &(p1, p2) in edges.iter() {
        let y1_ratio = keypoints[p1 * 3];
        let x1_ratio = keypoints[p1 * 3 + 1];
        let conf1 = keypoints[p1 * 3 + 2];

        let y2_ratio = keypoints[p2 * 3];
        let x2_ratio = keypoints[p2 * 3 + 1];
        let conf2 = keypoints[p2 * 3 + 2];

        if conf1 > threshold && conf2 > threshold {
            let start_point = Point {
                x: (x1_ratio * base) as i32 - pad_x,
                y: (y1_ratio * base) as i32 - pad_y,
            };
            let end_point = Point {
                x: (x2_ratio * base) as i32 - pad_x,
                y: (y2_ratio * base) as i32 - pad_y,
            };

            line(
                img,
                start_point,
                end_point,
                Scalar::new(0.0, 0.0, 255.0, 0.0), // RGB(0,0,255) = Red color for the line
                2,
                LINE_AA,
                0,
            )
            .expect("Draw line [FAILED]");
        }
    }
}
