#[derive(Debug)]
pub struct ProcessorSingle{
    process_time: f64,
    remaining_time: f64,
    on_process_start: Option<fn()>,
    on_process_end: Option<fn()>,
}

impl ProcessorSingle{
    pub fn new() -> ProcessorSingle{
        ProcessorSingle{
            process_time: 0.0,
            remaining_time: 0.0,
            on_process_start: None,
            on_process_end: None,
        }
    }

    pub fn set_process_start_event() -> (){
        todo!();
    }
    pub fn set_process_end_event() -> (){
        todo!();
    }

    pub fn get_remaining_time(&self) -> f64{
        self.remaining_time
    }

    pub fn set_remaining_time(&mut self, time: f64) -> (){
        if time <= self.process_time{
            self.remaining_time = time;
        }else{
            panic!("Time must be less than process time. Change the process time first, if needed.");
        }
    }

    pub fn set_process_time(&mut self, time: f64) -> (){
        self.process_time = time;
    }

    pub fn get_process_time(&self) -> f64{
        self.process_time
    }



}


use std::collections::HashMap;
use crate::flowagent::FlowAgent;
pub struct ProcessorMultiple{
    agent_reference: HashMap<&'static FlowAgent, i16>,
    remaining_time: f64,
    on_process_start: Option<fn()>,
    on_process_end: Option<fn()>,
}

