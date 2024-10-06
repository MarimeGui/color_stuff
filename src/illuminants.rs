use crate::representations::CIExyCoords;

pub const D50_ILLUMINANT: CIExyCoords = CIExyCoords {
    x: 0.34567,
    y: 0.35850,
};

// https://en.wikipedia.org/wiki/Standard_illuminant#Illuminant_series_D
// There are more precise definitions (Wikipedia), but using official ITU values used in Rec. 709 and 2020
pub const D65_ILLUMINANT: CIExyCoords = CIExyCoords {
    x: 0.3127,
    y: 0.3290,
};

pub const ACES_ILLUMINANT: CIExyCoords = CIExyCoords {
    x: 0.32168,
    y: 0.33767,
};
