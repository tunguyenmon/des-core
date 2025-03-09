pub mod flowagent;
mod flowblock;
mod entity;
mod sim;



pub fn add(left: usize, right: usize) -> usize {
    left + right
}


/// Create the simulation
    /// 
    /// # Arguments
    /// * None
    /// 
    /// # Returns
    /// * Simulation Instance
    /// 
    /// This Instance can be used to create all agents and run the the simulation.
pub fn create_simulation() -> sim::SIM{

    sim::SIM::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
