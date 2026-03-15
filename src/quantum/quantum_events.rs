
use crate::misc::omega;
use crate::misc::ecc;

// Accepted quantum event
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QEvent {
    pub omega: OmegaStep,   					// Ω contribution
    pub code:  ECCWord,     					// ECC-validated measurement
    pub T:     Interval,    					// control scalar T_n
    pub xyz:   (Interval, Interval, Interval), 	// spatial bins
}
