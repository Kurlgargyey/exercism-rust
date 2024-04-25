use std::collections::HashMap;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(u32);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```

struct InputCell<T> {
    value: T,
    callbacks: Vec<CallbackId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(u32);

struct ComputeCell<T> {
    dependencies: Vec<CellId>,
    function: Box<dyn Fn(&[T]) -> T>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(u32);

struct Callback<T: Clone> {
    compute_cell: ComputeCellId,
    function: Box<dyn FnMut(T)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<T: Copy> {
    input_cells: HashMap<InputCellId, InputCell<T>>,
    compute_cells: HashMap<ComputeCellId, ComputeCell<T>>,
    callbacks: HashMap<CallbackId, Callback<T>>,
    next_input: u32,
    next_compute: u32,
    next_callback: u32,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: HashMap::<InputCellId, InputCell<T>>::new(),
            compute_cells: HashMap::<ComputeCellId, ComputeCell<T>>::new(),
            callbacks: HashMap::<CallbackId, Callback<T>>::new(),
            next_input: 0,
            next_compute: 0,
            next_callback: 0,
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let input_cell = InputCellId(self.next_input);
        self.next_input += 1;
        self.input_cells.insert(input_cell, InputCell {
            value: initial,
            callbacks: Vec::<CallbackId>::new(),
        });
        input_cell
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F
    ) -> Result<ComputeCellId, CellId> {
        for id in dependencies {
            if (
                match id {
                    CellId::Input(id) => id.0 >= self.next_input,
                    CellId::Compute(id) => id.0 >= self.next_compute,
                }
            ) {
                return Err(*id);
            }
        }
        let compute_cell = ComputeCellId(self.next_compute);
        self.next_compute += 1;

        self.compute_cells.insert(compute_cell, ComputeCell {
            dependencies: dependencies.to_vec(),
            function: Box::new(compute_func),
        });
        Ok(compute_cell)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(input_cell) => {
                if let Some(input_cell) = self.input_cells.get(&input_cell) {
                    Some(input_cell.value)
                } else {
                    None
                }
            }
            CellId::Compute(compute_cell) => {
                if let Some(compute_cell) = self.compute_cells.get(&compute_cell) {
                    Some(
                        (compute_cell.function)(
                            &self.resolve_dependencies(&compute_cell.dependencies)
                        )
                    )
                } else {
                    None
                }
            }
        }
    }

    fn resolve_dependencies(&self, dependencies: &Vec<CellId>) -> Vec<T> {
        dependencies
            .iter()
            .map(|cell| self.value(*cell).unwrap())
            .collect()
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if let Some(input_cell) = self.input_cells.get_mut(&id) {
            input_cell.value = new_value;
            self.run_callbacks(id);
            true
        } else {
            false
        }
    }

    fn run_callbacks(&mut self, id: InputCellId) {
        for callback_id in &self.input_cells.get(&id).unwrap().callbacks {
            let callback = self.callbacks.get_mut(&callback_id).unwrap();
            let cell_value = self.value(CellId::Compute(callback.compute_cell)).unwrap().clone();
            let function = &mut callback.function;
            function(cell_value);
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'static>(
        &mut self,
        id: ComputeCellId,
        callback_function: F
    ) -> Option<CallbackId> {
        if id.0 >= self.next_compute {
            return None;
        }
        let callback = Callback {
            compute_cell: id,
            function: Box::new(callback_function),
        };
        let callback_id = CallbackId(self.next_callback);
        self.next_callback += 1;
        self.callbacks.insert(callback_id, callback);
        for cell_id in &self.compute_cells.get(&id).unwrap().dependencies {
            if let CellId::Input(input_cell) = cell_id {
                self.input_cells.get_mut(input_cell).unwrap().callbacks.push(callback_id);
            }
        }
        Some(callback_id)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId
    ) -> Result<(), RemoveCallbackError> {
        if let Some(_) = self.callbacks.get(&callback) {
            self.callbacks.remove(&callback);
        } else {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        if let Some(cell) = self.compute_cells.get(&cell) {
            for dependency in &cell.dependencies {
                if let CellId::Input(input_cell) = dependency {
                    let input_callbacks = &mut self.input_cells
                        .get_mut(&input_cell)
                        .unwrap().callbacks;
                    input_callbacks.retain(|cb_id| cb_id != &callback);
                }
            }
        } else {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        Ok(())
    }
}
