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
    fn execute(&self, input_buffers: &Vec<Buffer>, output_buffers: &mut Vec<Buffer>);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;

    #[derive(Debug, Clone)]
    struct TestOperator;

    impl Operator for TestOperator {
        fn execute(&self, input_buffers: &Vec<Buffer>, output_buffers: &mut Vec<Buffer>) {
            // TODO: Add check for input and output buffer sizes
            for (input, output) in input_buffers.iter().zip(output_buffers.iter_mut()) {
                output.write(input.read());
            }
        }

        fn input_count(&self) -> usize {
            2
        }

        fn output_count(&self) -> usize {
            2
        }

        fn compute_output_buffer_sizes(&self, input_buffer_sizes: &[usize]) -> Vec<usize> {
            input_buffer_sizes.to_vec()
        }
    }

    #[test]
    fn test_operator_execute() {
        let operator = TestOperator;
        let input_buffers = vec![Buffer::new_from_data(vec![1 as u8; 10]), Buffer::new_from_data(vec![2 as u8; 20])];
        let mut output_buffers = vec![Buffer::new_from_size(10), Buffer::new_from_size(20)];
        operator.execute(&input_buffers, &mut output_buffers);
        assert_eq!(input_buffers, output_buffers);
    }

    #[test]
    fn test_operator_compute_output_buffer_sizes() {
        let operator = TestOperator;
        let input_buffer_sizes = vec![10, 20];
        let expected_output_buffer_sizes = vec![10, 20];
        assert_eq!(operator.compute_output_buffer_sizes(&input_buffer_sizes), expected_output_buffer_sizes);
    }
}