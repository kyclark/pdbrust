//! CRYST1 record structure and implementations

/// Represents a CRYST1 record from a PDB file.
///
/// Gives the unit cell and, for a crystal structure, its space group. For a
/// molecular dynamics structure this is the only record that carries the
/// periodic box, so a file that loses it can no longer be re-wrapped, measured
/// under the minimum image convention, or used to continue the simulation.
#[derive(Debug, Clone, PartialEq)]
pub struct Cryst1 {
    /// Unit cell length a, in angstroms.
    pub a: f64,
    /// Unit cell length b, in angstroms.
    pub b: f64,
    /// Unit cell length c, in angstroms.
    pub c: f64,
    /// Unit cell angle alpha, in degrees.
    pub alpha: f64,
    /// Unit cell angle beta, in degrees.
    pub beta: f64,
    /// Unit cell angle gamma, in degrees.
    pub gamma: f64,
    /// Space group, as written. Empty when the field is blank, which is usual
    /// for simulation output.
    pub space_group: String,
    /// Z value, or `None` when the field is blank.
    pub z: Option<i32>,
}
