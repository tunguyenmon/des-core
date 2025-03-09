#[derive(Debug)]
pub struct Time{
    max_time_steps: u64,
    current_time_step: u64,
}

impl Time{
    pub fn new() -> Time{
        Time{
            max_time_steps: 1,
            current_time_step: 0,
        }
    }

    pub fn timestep(&mut self){
        self.current_time_step += 1;
    }

    pub fn set_max_time_steps(&mut self, max_time_steps: u64){
        self.max_time_steps = max_time_steps;
    }

    pub fn get_max_time_steps(&self) -> u64{
        self.max_time_steps
    }

    pub fn is_finished(&self) -> bool{
        self.current_time_step >= self.max_time_steps
    }

    pub fn get_current_time_step(&self) -> u64{
        self.current_time_step
    }
}