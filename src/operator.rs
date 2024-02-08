use crate::buffer::Buffer;
use std::vec::Vec;
use std::fmt::Debug;


pub trait Operator: Debug {
    /// Executes the operation using the provided input buffers and writes the results into the provided output buffers.
    ///
    /// # Arguments
    ///
    /// * `input_buffers` - A slice of `Buffer` instances containing the inputs to the operation.
    /// * `output_buffers` - A mutable slice of `Buffer` instances where the operation's results should be stored.
    fn execute(&self, input_buffers: &[Buffer], output_buffers: &mut [Buffer]);

    /// Returns the expected number of input buffers for this operation.
    ///
    /// # Returns
    ///
    /// The number of input buffers required.
    fn input_count(&self) -> usize;

    /// Returns the expected number of output buffers that this operation will produce.
    ///
    /// # Returns
    ///
    /// The number of output buffers produced.
    fn output_count(&self) -> usize;

    /// Calculates and returns the sizes of the output buffers based on the sizes of the input buffers.
    ///
    /// # Arguments
    ///
    /// * `input_buffer_sizes` - A slice of sizes corresponding to each input buffer.
    ///
    /// # Returns
    ///
    /// A vector of sizes for each expected output buffer.
    fn compute_output_buffer_sizes(&self, input_buffer_sizes: &[usize]) -> Vec<usize>;
}

