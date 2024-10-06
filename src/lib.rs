// https://github.com/MONOGRID/gainmap-js
// https://helpx.adobe.com/content/dam/help/en/camera-raw/using/gain-map/jcr_content/root/content/flex/items/position/position-par/table/row-io13dug-column-4a63daf/download_section/download-1/Gain_Map_1_0d14.pdf
// https://developer.android.com/media/platform/hdr-image-format
// https://openexr.com/en/latest/TechnicalIntroduction.html#
// https://stackoverflow.com/questions/45605506/how-are-cie-xyy-luminance-values-for-color-primaries-determined

// http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_xyY.html

pub mod illuminants;
pub mod representations;
pub mod spaces;
pub mod transfer_functions;

// ----- Matrix

use nalgebra::SMatrix;

pub type Matrix3x3f = SMatrix<f32, 3, 3>;
pub type Matrix3x1f = SMatrix<f32, 3, 1>;
