use libc::{c_float, c_int};

use crate::color;
use crate::encoder::{Comparison, Decision};
use crate::Rational;

use super::{State, VideoEncoder};

impl<S: State> VideoEncoder<S> {
    pub fn set_gop(&mut self, value: u32) {
        unsafe {
            (*self.as_mut_ptr()).gop_size = value as c_int;
        }
    }

    pub fn set_max_b_frames(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).max_b_frames = value as c_int;
        }
    }

    pub fn set_b_quant_factor(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).b_quant_factor = value as c_float;
        }
    }

    pub fn set_b_quant_offset(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).b_quant_offset = value as c_float;
        }
    }

    pub fn set_i_quant_factor(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).i_quant_factor = value as c_float;
        }
    }

    pub fn set_i_quant_offset(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).i_quant_offset = value as c_float;
        }
    }

    pub fn set_lumi_masking(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).lumi_masking = value as c_float;
        }
    }

    pub fn set_temporal_cplx_masking(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).temporal_cplx_masking = value as c_float;
        }
    }

    pub fn set_spatial_cplx_masking(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).spatial_cplx_masking = value as c_float;
        }
    }

    pub fn set_p_masking(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).p_masking = value as c_float;
        }
    }

    pub fn set_dark_masking(&mut self, value: f32) {
        unsafe {
            (*self.as_mut_ptr()).dark_masking = value as c_float;
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn set_prediction(&mut self, value: Prediction) {
        unsafe {
            (*self.as_mut_ptr()).prediction_method = value.into();
        }
    }

    pub fn set_aspect_ratio<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).sample_aspect_ratio = value.into().into();
        }
    }

    pub fn set_me_comparison(&mut self, value: Comparison) {
        unsafe {
            (*self.as_mut_ptr()).me_cmp = value.into();
        }
    }

    pub fn set_me_sub_comparison(&mut self, value: Comparison) {
        unsafe {
            (*self.as_mut_ptr()).me_sub_cmp = value.into();
        }
    }

    pub fn set_mb_comparison(&mut self, value: Comparison) {
        unsafe {
            (*self.as_mut_ptr()).mb_cmp = value.into();
        }
    }

    pub fn set_ildct_comparison(&mut self, value: Comparison) {
        unsafe {
            (*self.as_mut_ptr()).ildct_cmp = value.into();
        }
    }

    pub fn set_dia_size(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).dia_size = value as c_int;
        }
    }

    pub fn set_last_predictors(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).last_predictor_count = value as c_int;
        }
    }

    #[cfg(not(feature = "ffmpeg_5_0"))]
    pub fn set_pre_me(&mut self, value: MotionEstimation) {
        unsafe {
            (*self.as_mut_ptr()).pre_me = value.into();
        }
    }

    pub fn set_me_pre_comparison(&mut self, value: Comparison) {
        unsafe {
            (*self.as_mut_ptr()).me_pre_cmp = value.into();
        }
    }

    pub fn set_pre_dia_size(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).pre_dia_size = value as c_int;
        }
    }

    pub fn set_me_subpel_quality(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).me_subpel_quality = value as c_int;
        }
    }

    pub fn set_me_range(&mut self, value: usize) {
        unsafe {
            (*self.as_mut_ptr()).me_range = value as c_int;
        }
    }

    pub fn set_mb_decision(&mut self, value: Decision) {
        unsafe {
            (*self.as_mut_ptr()).mb_decision = value.into();
        }
    }

    pub fn set_mb_lmin(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).mb_lmin = value as c_int;
        }
    }

    pub fn set_mb_lmax(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).mb_lmax = value as c_int;
        }
    }

    pub fn set_intra_dc_precision(&mut self, value: u8) {
        unsafe {
            (*self.as_mut_ptr()).intra_dc_precision = i32::from(value);
        }
    }

    pub fn set_qmin(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).qmin = value as c_int;
        }
    }

    pub fn set_qmax(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).qmax = value as c_int;
        }
    }

    pub fn set_global_quality(&mut self, value: i32) {
        unsafe {
            (*self.as_mut_ptr()).global_quality = value as c_int;
        }
    }

    pub fn set_colorspace(&mut self, value: color::Space) {
        unsafe {
            (*self.as_mut_ptr()).colorspace = value.into();
        }
    }
}
