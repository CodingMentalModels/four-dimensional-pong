use bevy::prelude::*;

struct Matrix;

impl Matrix {
    pub fn approximately_equal(a: Mat4, b: Mat4) -> bool {
        let diff = a - b;
        diff.to_cols_array().iter().all(|x| x.abs() < 0.0001)
    }
}

#[cfg(test)]
mod test_transforms {
    use std::f32::consts::TAU;

    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let m_1 = Mat2::from_cols_array(&[
            1., 2.,
            3., 4.,
        ]).transpose();

        let m_2 = Mat2::from_cols_array(&[
            -5., 10.,
            -5., 5.,
        ]).transpose();

        assert_eq!(m_1 * m_2, Mat2::from_cols_array(&[
            -15., 20.,
            -35., 50.,
        ]).transpose());
    }

    #[test]
    fn test_transforms_to_matrices() {
        let translation = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let matrix = translation.compute_matrix();
        assert_eq!(matrix, Mat4::from_cols_array(&[
            1.0, 0.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 2.0,
            0.0, 0.0, 1.0, 3.0,
            0.0, 0.0, 0.0, 1.0,
        ]).transpose());

        let rotation = Transform::from_rotation(Quat::from_rotation_x(TAU/8.0));
        let matrix = rotation.compute_matrix();
        let r_2 = f32::sqrt(2.) / 2.;
        assert!(
            Matrix::approximately_equal(matrix,
                Mat4::from_cols_array(&[
                    1.0, 0.0,  0.0, 0.0,
                    0.0, r_2, -r_2, 0.0,
                    0.0, r_2,  r_2, 0.0,
                    0.0, 0.0,  0.0, 1.0,
                ]).transpose()
            )
        );

        
        let scale = Transform::from_scale(Vec3::new(1.0, 2.0, 3.0));
        let matrix = scale.compute_matrix();
        assert!(
            Matrix::approximately_equal(matrix,
                Mat4::from_cols_array(&[
                    1.0, 0.0, 0.0, 0.0,
                    0.0, 2.0, 0.0, 0.0,
                    0.0, 0.0, 3.0, 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ]).transpose()
            )
        );

        let combined = translation * rotation * scale;
        let matrix = combined.compute_matrix();
        assert!(
            Matrix::approximately_equal(matrix,
                Mat4::from_cols_array(&[
                    1.0,    0.0,     0.0, 1.0,
                    0.0, 2.*r_2, -3.*r_2, 2.0,
                    0.0, 2.*r_2,  3.*r_2, 3.0,
                    0.0,    0.0,     0.0, 1.0,
                ]).transpose()
            )
        );

    }
}