use crate::{
    Schema, Schema1, Schema2, Schema3, Schema4, View1Full, View2Breakdown2ndCount, View2Full,
    View3Breakdown3rdCount, View3Full, View4Breakdown4thCount, View4Full,
};
#[cfg(any(feature = "primitive_impls", feature = "pointer_impls"))]
use crate::{
    View2Breakdown2nd, View2Regular, View3Breakdown2ndView3rd, View3Breakdown3rd, View3Regular,
    View4Breakdown3rdView4th, View4Breakdown4th, View4Regular,
};
#[cfg(feature = "pointer_impls")]
use std::ops::Deref;
// We use this in the doc strings.
#[allow(unused_imports)]
use super::View;
// We use this in the doc strings.
#[allow(unused_imports)]
use super::Schemas;

/// A dataset in `flat`.
/// The same dataset may be observed through multiple views.
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let builder: Dataset<Schema1<f64>> = DatasetBuilder::new(Schemas::one("dim1")).build();
/// ```
pub struct Dataset<S: Schema> {
    pub(crate) schema: S,
    data: Vec<S::Dimensions>,
}

impl<S: Schema> Dataset<S> {
    /// Get the data held within this `Dataset`.
    pub(crate) fn data(&self) -> &[S::Dimensions] {
        self.data.as_slice()
    }
}

/// Builder for a dataset in `flat`.
///
/// ```
/// # use flat::*;
/// // Note, explicit type annotation included for clarity.
/// // We encourage consumers to allow the compiler to infer the type implicitly.
/// let builder: DatasetBuilder<Schema1<f64>> = DatasetBuilder::new(Schemas::one("dim1"));
/// ```
pub struct DatasetBuilder<S: Schema> {
    schema: S,
    data: Vec<S::Dimensions>,
}

// Would love to be able to do this, but it conflicts with the specific `impl Dataset<Schema1<i32>>` (etc) implementations.
// We need specialization!
// impl<T: Deref<Target = f64>> Dataset<Schema1<T>> {
// impl<T: Into<f64> + Clone> Dataset<Schema1<T>> {

#[cfg(feature = "pointer_impls")]
impl<T: Clone + Into<f64>, Dt: Deref<Target = T>> Dataset<Schema1<Dt>> {
    pub fn reflect_1st(&self) -> View1Full<Schema1<Dt>> {
        let extractor: Box<dyn Fn(&<Schema1<Dt> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.0).clone().into());
        View1Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_0.clone(),
        }
    }
}

#[cfg(feature = "primitive_impls")]
mod primitive_impls1 {
    use super::*;

    macro_rules! impl_schema1_view {
        ($T:ty, $attrs:meta) => {
            #[$attrs]
            #[allow(rustdoc::broken_intra_doc_links)]
            impl Dataset<Schema1<$T>> {
                /// Take a reflective view of this 1-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the final dimension (1st), and use all other dimensions in the frame of the widget.
                /// The term 'reflection' refers to the fact that the value will appear in both in the frame and rendering of the widget.
                /// ```text
                /// r#"
                /// Frame..   | Rendering....
                /// (dim1, )  | aggregate(dim1)"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema1<T>` where `T = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema1<Dt>` where `T: Clone + Into<f64>, Dt: Deref<Target = T>`.
                pub fn reflect_1st(&self) -> View1Full<Schema1<$T>> {
                    let extractor: Box<dyn Fn(&<Schema1<$T> as Schema>::Dimensions) -> f64> =
                        Box::new(|d| d.0 as f64);
                    View1Full {
                        dataset: &self,
                        extractor,
                        value_header: self.schema.dimension_0.clone(),
                    }
                }
            }
        };
    }

    impl_schema1_view!(f64, doc());
    impl_schema1_view!(f32, doc(hidden));
    impl_schema1_view!(isize, doc(hidden));
    impl_schema1_view!(i128, doc(hidden));
    impl_schema1_view!(i64, doc(hidden));
    impl_schema1_view!(i32, doc(hidden));
    impl_schema1_view!(i16, doc(hidden));
    impl_schema1_view!(i8, doc(hidden));
    impl_schema1_view!(usize, doc(hidden));
    impl_schema1_view!(u128, doc(hidden));
    impl_schema1_view!(u64, doc(hidden));
    impl_schema1_view!(u32, doc(hidden));
    impl_schema1_view!(u16, doc(hidden));
    impl_schema1_view!(u8, doc(hidden));
}

impl<T> Dataset<Schema1<T>> {
    /// Take a counting view of this N-dimensional dataset.
    /// Views are rendered differently by different widgets, but
    /// always have a frame on the left and a rendering on the right.
    ///
    /// This view will render the occurrences of each dimensional vector, while using all dimensions in the frame of the widget.
    /// ```text
    /// r#"
    /// Frame..  | Rendering..
    /// (*,)     | aggregate(count())"#
    /// ```
    ///
    /// Implemented for `Schema1<_>`, `Schema2<_, _>`, `Schema3<_, _, _>`.
    pub fn count(&self) -> View1Full<Schema1<T>> {
        let extractor: Box<dyn Fn(&<Schema1<T> as Schema>::Dimensions) -> f64> = Box::new(|_| 1f64);
        View1Full {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }
}

#[cfg(feature = "pointer_impls")]
impl<T, U: Clone + Into<f64>, Du: Deref<Target = U>> Dataset<Schema2<T, Du>> {
    pub fn reflect_2nd(&self) -> View2Full<Schema2<T, Du>> {
        let extractor: Box<dyn Fn(&<Schema2<T, Du> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.1).clone().into());
        View2Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_1.clone(),
        }
    }

    pub fn view_2nd(&self) -> View2Regular<Schema2<T, Du>> {
        let extractor: Box<dyn Fn(&<Schema2<T, Du> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.1).clone().into());
        View2Regular {
            dataset: &self,
            extractor,
        }
    }

    pub fn breakdown_2nd(&self) -> View2Breakdown2nd<Schema2<T, Du>> {
        let extractor: Box<dyn Fn(&<Schema2<T, Du> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.1).clone().into());
        View2Breakdown2nd {
            dataset: &self,
            extractor,
        }
    }
}

#[cfg(feature = "primitive_impls")]
mod primitive_impls2 {
    use super::*;

    macro_rules! impl_schema2_view {
        ($T:ty, $attrs:meta) => {
            #[$attrs]
            #[allow(rustdoc::broken_intra_doc_links)]
            impl<T> Dataset<Schema2<T, $T>> {
                /// Take a reflective view of this 2-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the final dimension (2nd), and use all other dimensions in the frame of the widget.
                /// The term 'reflection' refers to the fact that the value will appear in both in the frame and rendering of the widget.
                /// ```text
                /// r#"
                /// Frame..       | Rendering..
                /// (dim1, dim2)  | aggregate(dim2)"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema2<_, U>` where `U = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema2<_, Du>` where `U: Clone + Into<f64>, Du: Deref<Target = U>`.
                pub fn reflect_2nd(&self) -> View2Full<Schema2<T, $T>> {
                    let extractor: Box<dyn Fn(&<Schema2<T, $T> as Schema>::Dimensions) -> f64> =
                        Box::new(|d| d.1 as f64);
                    View2Full {
                        dataset: &self,
                        extractor,
                        value_header: self.schema.dimension_1.clone(),
                    }
                }

                /// Take a regular view of this 2-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the final dimension (2nd), and use all other dimensions in the frame of the widget.
                /// ```text
                /// r#"
                /// Frame..   | Rendering..
                /// (dim1, )  | aggregate(dim2)"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema2<_, U>` where `U = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema2<_, Du>` where `U: Clone + Into<f64>, Du: Deref<Target = U>`.
                pub fn view_2nd(&self) -> View2Regular<Schema2<T, $T>> {
                    let extractor: Box<dyn Fn(&<Schema2<T, $T> as Schema>::Dimensions) -> f64> =
                        Box::new(|d| d.1 as f64);
                    View2Regular {
                        dataset: &self,
                        extractor,
                    }
                }

                /// Take a breakdown view of this 2-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the breakdown of the final dimension (2nd), and use all other dimensions in the frame of the widget.
                /// ```text
                /// r#"
                /// Frame..   | Breakdown Rendering..              |
                /// (dim1, )  | breakdown(dim2, aggregate(dim2)).. |"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema2<_, U>` where `U = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema2<_, Du>` where `U: Clone + Into<f64>, Du: Deref<Target = U>`.
                pub fn breakdown_2nd(&self) -> View2Breakdown2nd<Schema2<T, $T>> {
                    let extractor: Box<dyn Fn(&<Schema2<T, $T> as Schema>::Dimensions) -> f64> =
                        Box::new(|d| d.1 as f64);
                    View2Breakdown2nd {
                        dataset: &self,
                        extractor,
                    }
                }
            }
        };
    }

    impl_schema2_view!(f64, doc());
    impl_schema2_view!(f32, doc(hidden));
    impl_schema2_view!(isize, doc(hidden));
    impl_schema2_view!(i128, doc(hidden));
    impl_schema2_view!(i64, doc(hidden));
    impl_schema2_view!(i32, doc(hidden));
    impl_schema2_view!(i16, doc(hidden));
    impl_schema2_view!(i8, doc(hidden));
    impl_schema2_view!(usize, doc(hidden));
    impl_schema2_view!(u128, doc(hidden));
    impl_schema2_view!(u64, doc(hidden));
    impl_schema2_view!(u32, doc(hidden));
    impl_schema2_view!(u16, doc(hidden));
    impl_schema2_view!(u8, doc(hidden));
}

impl<T, U> Dataset<Schema2<T, U>> {
    #[doc(hidden)]
    pub fn count(&self) -> View2Full<Schema2<T, U>> {
        let extractor: Box<dyn Fn(&<Schema2<T, U> as Schema>::Dimensions) -> f64> =
            Box::new(|_| 1.0);
        View2Full {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }

    /// Take a view of this 2-dimensional dataset breaking down the 2nd column.
    /// Views are rendered differently by different widgets, but
    /// always have a frame on the left and a rendering on the right.
    ///
    /// This view will render the breakdown of the final dimension (2nd), and use all other dimensions in the frame of the widget.
    /// Rather than displaying the value of the final dimension, the occurrences of each dimensional vector are counted.
    /// ```text
    /// r#"
    /// Frame..   | Breakdown Rendering..                 |
    /// (dim1, )  | breakdown(dim2, aggregate(count())).. |"#
    /// ```
    ///
    /// Implemented for `Schema2<_, _>`.
    pub fn count_breakdown_2nd(&self) -> View2Breakdown2ndCount<Schema2<T, U>> {
        View2Breakdown2ndCount { dataset: &self }
    }
}

#[cfg(feature = "primitive_impls")]
mod primitive_impls3 {
    use super::*;

    macro_rules! impl_schema3_view {
        ($T:ty, $attrs:meta) => {
            #[$attrs]
            #[allow(rustdoc::broken_intra_doc_links)]
            impl<T, U> Dataset<Schema3<T, U, $T>> {
                /// Take a reflective view of this 3-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the final dimension (3rd), and use all other dimensions in the frame of the widget.
                /// The term 'reflection' refers to the fact that the value will appear in both in the frame and rendering of the widget.
                /// ```text
                /// r#"
                /// Frame..             | Rendering..
                /// (dim1, dim2, dim3)  | aggregate(dim3)"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema3<_, _, V>` where `V = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema3<_, _, Dv>` where `V: Clone + Into<f64>, Dv: Deref<Target = V>`.
                pub fn reflect_3rd(&self) -> View3Full<Schema3<T, U, $T>> {
                    let extractor: Box<dyn Fn(&<Schema3<T, U, $T> as Schema>::Dimensions) -> f64> =
                        Box::new(|d| d.2 as f64);
                    View3Full {
                        dataset: &self,
                        extractor,
                        value_header: self.schema.dimension_2.to_string(),
                    }
                }

                /// Take a regular view of this 3-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the final dimension (3rd), and use all other dimensions in the frame of the widget.
                /// ```text
                /// r#"
                /// Frame..       | Rendering..
                /// (dim1, dim2)  | aggregate(dim3)"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema3<_, _, V>` where `V = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema3<_, _, Dv>` where `V: Clone + Into<f64>, Dv: Deref<Target = V>`.
                pub fn view_3rd(&self) -> View3Regular<Schema3<T, U, $T>> {
                    let extractor: Box<dyn Fn(&<Schema3<T, U, $T> as Schema>::Dimensions) -> f64> =
                        Box::new(|d| d.2 as f64);
                    View3Regular {
                        dataset: &self,
                        extractor,
                    }
                }

                /// Take a breakdown view of this 3-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the breakdown of the final dimension (3rd), and use all other dimensions in the frame of the widget.
                /// ```text
                /// r#"
                /// Frame..       | Breakdown Rendering..              |
                /// (dim1, dim2)  | breakdown(dim3, aggregate(dim3)).. |"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema3<_, _, V>` where `V = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema3<_, _, Dv>` where `V: Clone + Into<f64>, Dv: Deref<Target = V>`.
                pub fn breakdown_3rd(&self) -> View3Breakdown3rd<Schema3<T, U, $T>> {
                    let extractor: Box<dyn Fn(&<Schema3<T, U, $T> as Schema>::Dimensions) -> f64> =
                        Box::new(|d| d.2 as f64);
                    View3Breakdown3rd {
                        dataset: &self,
                        extractor,
                    }
                }

                /// Take a breakdown+view view of this 3-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the final dimension (3rd) under a breakdown of the next dimension (2nd), and use all other dimensions in the frame of the widget.
                /// ```text
                /// r#"
                /// Frame..   | Breakdown Rendering..              |
                /// (dim1, )  | breakdown(dim2, aggregate(dim3)).. |"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema3<_, _, V>` where `V = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema3<_, _, Dv>` where `V: Clone + Into<f64>, Dv: Deref<Target = V>`.
                pub fn breakdown_2nd_view_3rd(
                    &self,
                ) -> View3Breakdown2ndView3rd<Schema3<T, U, $T>> {
                    let extractor: Box<dyn Fn(&<Schema3<T, U, $T> as Schema>::Dimensions) -> f64> =
                        Box::new(|d| d.2 as f64);
                    View3Breakdown2ndView3rd {
                        dataset: &self,
                        extractor,
                    }
                }
            }
        };
    }

    impl_schema3_view!(f64, doc());
    impl_schema3_view!(f32, doc(hidden));
    impl_schema3_view!(isize, doc(hidden));
    impl_schema3_view!(i128, doc(hidden));
    impl_schema3_view!(i64, doc(hidden));
    impl_schema3_view!(i32, doc(hidden));
    impl_schema3_view!(i16, doc(hidden));
    impl_schema3_view!(i8, doc(hidden));
    impl_schema3_view!(usize, doc(hidden));
    impl_schema3_view!(u128, doc(hidden));
    impl_schema3_view!(u64, doc(hidden));
    impl_schema3_view!(u32, doc(hidden));
    impl_schema3_view!(u16, doc(hidden));
    impl_schema3_view!(u8, doc(hidden));
}

#[cfg(feature = "pointer_impls")]
impl<T, U, V: Clone + Into<f64>, Dv: Deref<Target = V>> Dataset<Schema3<T, U, Dv>> {
    pub fn reflect_3rd(&self) -> View3Full<Schema3<T, U, Dv>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, Dv> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.2).clone().into());
        View3Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_2.to_string(),
        }
    }

    pub fn view_3rd(&self) -> View3Regular<Schema3<T, U, Dv>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, Dv> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.2).clone().into());
        View3Regular {
            dataset: &self,
            extractor,
        }
    }

    pub fn breakdown_3rd(&self) -> View3Breakdown3rd<Schema3<T, U, Dv>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, Dv> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.2).clone().into());
        View3Breakdown3rd {
            dataset: &self,
            extractor,
        }
    }

    pub fn breakdown_2nd_view_3rd(&self) -> View3Breakdown2ndView3rd<Schema3<T, U, Dv>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, Dv> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.2).clone().into());
        View3Breakdown2ndView3rd {
            dataset: &self,
            extractor,
        }
    }
}

impl<T, U, V> Dataset<Schema3<T, U, V>> {
    #[doc(hidden)]
    pub fn count(&self) -> View3Full<Schema3<T, U, V>> {
        let extractor: Box<dyn Fn(&<Schema3<T, U, V> as Schema>::Dimensions) -> f64> =
            Box::new(|_| 1.0);
        View3Full {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }

    /// Take a view of this 3-dimensional dataset breaking down the 3rd column.
    /// Views are rendered differently by different widgets, but
    /// always have a frame on the left and a rendering on the right.
    ///
    /// This view will render the breakdown of the final dimension (3rd), and use all other dimensions in the frame of the widget.
    /// ```text
    /// r#"
    /// Frame..       | Breakdown Rendering..                 |
    /// (dim1, dim2)  | breakdown(dim3, aggregate(count())).. |"#
    /// ```
    ///
    /// Implemented for `Schema3<_, _, _>`.
    pub fn count_breakdown_3rd(&self) -> View3Breakdown3rdCount<Schema3<T, U, V>> {
        View3Breakdown3rdCount { dataset: &self }
    }
}

#[cfg(feature = "primitive_impls")]
mod primitive_impls4 {
    use super::*;

    macro_rules! impl_schema4_view {
        ($T:ty, $attrs:meta) => {
            #[$attrs]
            #[allow(rustdoc::broken_intra_doc_links)]
            impl<T, U, V> Dataset<Schema4<T, U, V, $T>> {
                /// Take a reflective view of this 4-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the final dimension (4th), and use all other dimensions in the frame of the widget.
                /// The term 'reflection' refers to the fact that the value will appear in both in the frame and rendering of the widget.
                /// ```text
                /// r#"
                /// Frame..                   | Rendering..
                /// (dim1, dim2, dim3, dim4)  | aggregate(dim4)"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema4<_, _, _, W>` where `W = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema4<_, _, _, Dw>` where `W: Clone + Into<f64>, Dw: Deref<Target = W>`.
                pub fn reflect_4th(&self) -> View4Full<Schema4<T, U, V, $T>> {
                    let extractor: Box<
                        dyn Fn(&<Schema4<T, U, V, $T> as Schema>::Dimensions) -> f64,
                    > = Box::new(|d| d.3 as f64);
                    View4Full {
                        dataset: &self,
                        extractor,
                        value_header: self.schema.dimension_3.to_string(),
                    }
                }

                /// Take a regular view of this 4-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the final dimension (4th), and use all other dimensions in the frame of the widget.
                /// ```text
                /// r#"
                /// Frame..             | Rendering..
                /// (dim1, dim2, dim3)  | aggregate(dim4)"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema4<_, _, _, W>` where `W = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema4<_, _, _, Dw>` where `W: Clone + Into<f64>, Dw: Deref<Target = W>`.
                pub fn view_4th(&self) -> View4Regular<Schema4<T, U, V, $T>> {
                    let extractor: Box<
                        dyn Fn(&<Schema4<T, U, V, $T> as Schema>::Dimensions) -> f64,
                    > = Box::new(|d| d.3 as f64);
                    View4Regular {
                        dataset: &self,
                        extractor,
                    }
                }

                /// Take a breakdown view of this 4-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the breakdown of the final dimension (4th), and use all other dimensions in the frame of the widget.
                /// ```text
                /// r#"
                /// Frame..             | Breakdown Rendering..              |
                /// (dim1, dim2, dim3)  | breakdown(dim4, aggregate(dim4)).. |"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema4<_, _, _, W>` where `W = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema4<_, _, _, Dw>` where `W: Clone + Into<f64>, Dw: Deref<Target = W>`.
                pub fn breakdown_4th(&self) -> View4Breakdown4th<Schema4<T, U, V, $T>> {
                    let extractor: Box<
                        dyn Fn(&<Schema4<T, U, V, $T> as Schema>::Dimensions) -> f64,
                    > = Box::new(|d| d.3 as f64);
                    View4Breakdown4th {
                        dataset: &self,
                        extractor,
                    }
                }

                /// Take a breakdown+view view of this 4-dimensional dataset.
                /// Views are rendered differently by different widgets, but
                /// always have a frame on the left and a rendering on the right.
                ///
                /// This view will render the final dimension (4th) under a breakdown of the next dimension (3rd), and use all other dimensions in the frame of the widget.
                /// ```text
                /// r#"
                /// Frame..       | Breakdown Rendering..              |
                /// (dim1, dim2)  | breakdown(dim3, aggregate(dim4)).. |"#
                /// ```
                ///
                /// Requires feature `primitives_impl` or `pointers_impl`.
                /// * `primitives_impl`: implemented for `Schema4<_, _, _, W>` where `W = {f64, .., u8}`.
                /// * `pointers_impl`: implemented for `Schema4<_, _, _, Dw>` where `W: Clone + Into<f64>, Dw: Deref<Target = W>`.
                pub fn breakdown_3rd_view_4th(
                    &self,
                ) -> View4Breakdown3rdView4th<Schema4<T, U, V, $T>> {
                    let extractor: Box<
                        dyn Fn(&<Schema4<T, U, V, $T> as Schema>::Dimensions) -> f64,
                    > = Box::new(|d| d.3 as f64);
                    View4Breakdown3rdView4th {
                        dataset: &self,
                        extractor,
                    }
                }
            }
        };
    }

    impl_schema4_view!(f64, doc());
    impl_schema4_view!(f32, doc(hidden));
    impl_schema4_view!(isize, doc(hidden));
    impl_schema4_view!(i128, doc(hidden));
    impl_schema4_view!(i64, doc(hidden));
    impl_schema4_view!(i32, doc(hidden));
    impl_schema4_view!(i16, doc(hidden));
    impl_schema4_view!(i8, doc(hidden));
    impl_schema4_view!(usize, doc(hidden));
    impl_schema4_view!(u128, doc(hidden));
    impl_schema4_view!(u64, doc(hidden));
    impl_schema4_view!(u32, doc(hidden));
    impl_schema4_view!(u16, doc(hidden));
    impl_schema4_view!(u8, doc(hidden));
}

#[cfg(feature = "pointer_impls")]
impl<T, U, V, W: Clone + Into<f64>, Dw: Deref<Target = W>> Dataset<Schema4<T, U, V, Dw>> {
    pub fn reflect_4th(&self) -> View4Full<Schema4<T, U, V, Dw>> {
        let extractor: Box<dyn Fn(&<Schema4<T, U, V, Dw> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.3).clone().into());
        View4Full {
            dataset: &self,
            extractor,
            value_header: self.schema.dimension_3.to_string(),
        }
    }

    pub fn view_4th(&self) -> View4Regular<Schema4<T, U, V, Dw>> {
        let extractor: Box<dyn Fn(&<Schema4<T, U, V, Dw> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.3).clone().into());
        View4Regular {
            dataset: &self,
            extractor,
        }
    }

    pub fn breakdown_4th(&self) -> View4Breakdown4th<Schema4<T, U, V, Dw>> {
        let extractor: Box<dyn Fn(&<Schema4<T, U, V, Dw> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.3).clone().into());
        View4Breakdown4th {
            dataset: &self,
            extractor,
        }
    }

    pub fn breakdown_3rd_view_4th(&self) -> View4Breakdown3rdView4th<Schema4<T, U, V, Dw>> {
        let extractor: Box<dyn Fn(&<Schema4<T, U, V, Dw> as Schema>::Dimensions) -> f64> =
            Box::new(|d| (*d.3).clone().into());
        View4Breakdown3rdView4th {
            dataset: &self,
            extractor,
        }
    }
}

impl<T, U, V, W> Dataset<Schema4<T, U, V, W>> {
    #[doc(hidden)]
    pub fn count(&self) -> View4Full<Schema4<T, U, V, W>> {
        let extractor: Box<dyn Fn(&<Schema4<T, U, V, W> as Schema>::Dimensions) -> f64> =
            Box::new(|_| 1.0);
        View4Full {
            dataset: &self,
            extractor,
            value_header: "Count".to_string(),
        }
    }

    /// Take a view of this 4-dimensional dataset breaking down the 4th column.
    /// Views are rendered differently by different widgets, but
    /// always have a frame on the left and a rendering on the right.
    ///
    /// This view will render the breakdown of the final dimension (4th), and use all other dimensions in the frame of the widget.
    /// ```text
    /// r#"
    /// Frame..             | Breakdown Rendering..                 |
    /// (dim1, dim2, dim3)  | breakdown(dim4, aggregate(count())).. |"#
    /// ```
    ///
    /// Implemented for `Schema4<_, _, _, _>`.
    pub fn count_breakdown_4th(&self) -> View4Breakdown4thCount<Schema4<T, U, V, W>> {
        View4Breakdown4thCount { dataset: &self }
    }
}

impl<S: Schema> DatasetBuilder<S> {
    /// Build a dataset based for the provided schema.
    pub fn new(schema: S) -> DatasetBuilder<S> {
        Self {
            schema,
            data: Vec::default(),
        }
    }

    /// Update this dataset with a data point `vector`.
    /// Use this method to add data via mutation.
    ///
    /// See also: [`DatasetBuilder::add`].
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// let schema = Schemas::one("Things");
    /// let mut builder = DatasetBuilder::new(schema);
    /// builder.update((0, ));
    /// builder.update((0, ));
    /// builder.update((1, ));
    /// let dataset = builder.build();
    /// let view = dataset.count();
    ///
    /// let flat = DagChart::new(&view)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Count)
    /// 0       |**
    /// 1       |*"#);
    ///
    /// let flat = Histogram::new(&view, 2)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Count)
    /// [0, 1)  |**
    /// [1, 2]  |*"#);
    /// ```
    pub fn update(&mut self, vector: S::Dimensions) {
        self.data.push(vector);
    }

    /// Add a data point `vector` to this dataset.
    /// Use this method to add via method chaining.
    ///
    /// See also: [`DatasetBuilder::update`].
    ///
    /// ### Example
    /// ```
    /// use flat::*;
    ///
    /// let schema = Schemas::one("Things");
    /// let dataset = DatasetBuilder::new(schema)
    ///     .add((0, ))
    ///     .add((0, ))
    ///     .add((1, ))
    ///     .build();
    /// let view = dataset.count();
    ///
    /// let flat = DagChart::new(&view)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Count)
    /// 0       |**
    /// 1       |*"#);
    ///
    /// let flat = Histogram::new(&view, 2)
    ///     .render(Render::default());
    /// assert_eq!(
    ///     format!("\n{}", flat.to_string()),
    ///     r#"
    /// Things  |Sum(Count)
    /// [0, 1)  |**
    /// [1, 2]  |*"#);
    /// ```
    pub fn add(mut self, vector: S::Dimensions) -> Self {
        self.update(vector);
        self
    }

    /// Finalize the builder into a [`Dataset`].
    pub fn build(self) -> Dataset<S> {
        let DatasetBuilder { schema, data } = self;
        Dataset { schema, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Schemas;

    #[test]
    fn dataset_add() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let dataset = DatasetBuilder::new(schema).add((1,)).add((2,)).add((3,));
        assert_eq!(dataset.data.len(), 3);
    }

    #[test]
    fn dataset_update() {
        let schema: Schema1<i64> = Schemas::one("abc");
        let mut dataset = DatasetBuilder::new(schema);
        dataset.update((1,));
        dataset.update((2,));
        dataset.update((3,));
        assert_eq!(dataset.data.len(), 3);
    }
}
