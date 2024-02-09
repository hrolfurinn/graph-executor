use crate::operator::Operator;
use std::rc::Rc;

/// Represents an individual operation within a computation graph. It encapsulates a specific
/// computation operator and the identifiers for its input and output values.
///
/// The `operator` field holds a dynamically dispatched `Operator`, allowing for different
/// computational behaviors to be executed. The `input_ids` and `output_ids` are vectors of
/// integers that identify the specific inputs and outputs of this operation within the graph's
/// broader context.
#[derive(Debug, Clone)]
pub struct Operation {
    /// A shared reference to an operator, enabling runtime polymorphism and shared ownership.
    /// This allows multiple operations to share the same operator instance if needed.
    operator: Rc<dyn Operator>,
    /// Identifiers for the input values to this operation.
    input_ids: Vec<usize>,
    /// Identifiers for the output values from this operation.
    output_ids: Vec<usize>,
}

/// Represents a computation graph, which is a directed acyclic graph (DAG) of operations.
/// The graph defines a series of operations that process values, with certain values designated
/// as inputs and outputs of the graph. The objective is to compute the output values from the
/// given input values through a sequence of operations.
///
/// Each operation within the graph is capable of performing a specific computation, taking
/// certain values as input and producing output values. The graph orchestrates the execution
/// of these operations, ensuring that inputs and outputs are correctly managed.
///
/// Example graph structure:
///
/// ```ignore
/// Graph {
///     operations: [
///         Operation {
///             operator: &OperatorA,
///             input_ids: [0],
///             output_ids: [1, 2],
///         },
///         Operation {
///             operator: &OperatorB,
///             input_ids: [1, 2],
///             output_ids: [3],
///         },
///     ],
///     input_ids: [0],
///     output_ids: [3],
/// }
/// ```
///
/// Visualization:
///
/// ```plaintext
///                 +---> value 1 ---+
///                 |                |
///                 |                V
/// value 0 ---> operation 0    operation 1 ---> value 3
///                 |                ^
///                 |                |
///                 +---> value 2 ---+
/// ```

#[derive(Debug, Clone)]
pub struct Graph {
    /// A list of operations that constitute the graph. The operations are stored in no
    /// particular execution order; the graph's logic determines the correct order based on
    /// dependencies between operations.
    operations: Vec<Operation>,
    /// Identifiers for the input values to the entire graph. These are the entry points for
    /// data into the graph computation.
    input_ids: Vec<usize>,
    /// Identifiers for the output values from the entire graph. These are the results produced
    /// by the graph computation.
    output_ids: Vec<usize>,
}

impl Graph {
    /// Constructs a new `Graph` instance with the specified operations and input/output identifiers.
    ///
    /// # Arguments
    ///
    /// * `operations` - A vector of `Operation` instances that make up the graph.
    /// * `input_ids` - A vector of usize identifiers for the graph's input values.
    /// * `output_ids` - A vector of usize identifiers for the graph's output values.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Graph`.
    pub fn new(operations: Vec<Operation>, input_ids: Vec<usize>, output_ids: Vec<usize>) -> Self {
        Self {
            operations,
            input_ids,
            output_ids,
        }
    }
}
