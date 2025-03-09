mod time;
mod takt;

use crate::{flowagent, flowblock};
use crate::{flowagent::FlowAgent, flowblock::FlowBlock};
use crate::flowblock::station_single::StationSingle;
use rayon::prelude::*;

pub struct SIM{
    flowblocks: Vec<Box<dyn FlowBlock + Send + Sync>>,
    flowblock_names: Vec<String>,
    flowagents: Vec<FlowAgent>,
    time_controller: time::Time,
}

impl SIM{
    pub fn new() -> SIM{
        SIM{
            flowblocks: Vec::new(),
            flowblock_names: Vec::new(),
            flowagents: Vec::new(),
            time_controller: time::Time::new(),
        }
    }

    pub fn add_flowblock(&mut self, flowblock: Box<dyn FlowBlock + Send + Sync>){
        self.flowblocks.push(flowblock);
    }

    pub fn add_flowagent(&mut self, flowagent: FlowAgent){
        self.flowagents.push(flowagent);
    }

    #[cfg(test)]
    pub fn get_flowblocks(&self) -> & Vec<Box<dyn FlowBlock + Send + Sync>>{
        &self.flowblocks
    }

    fn flowblock_exists(&self, name: &str) -> bool{
        if self.flowblock_names.contains(&name.to_string()){
            return true;
        }
        return false
    }

    pub fn add_single_station(&mut self, name: &str) -> bool{
        if !self.flowblock_exists(name){
            let new_single_station = StationSingle::<flowblock::Idle>::new(name);
            let station = Box::new(new_single_station);
            self.add_flowblock(station);
            return true;
        }
        println!("Block with name '{}' already exists. Please choose a different name.", name);
        return false;
    }

    pub fn get_mut_flowblock_by_name(&mut self, name: &str) -> Result<&mut Box<dyn FlowBlock + Send + Sync>, String>{
        for block in self.flowblocks.iter_mut(){
            if block.get_name() == name{
                return Ok(block);
            }
        }
        return Err(format!("SIM::get_mut_flowblock_by_name - Could not find Flowblock {}", name));
    }

    pub fn get_flowbock_by_name(&self, name: &str) -> Result<& Box<dyn FlowBlock + Send + Sync>, String>{
        for block in self.flowblocks.iter(){
            if block.get_name() == name{
                return Ok(block);
            }
        }
        return Err(format!("SIM::get_mut_flowblock_by_name - Could not find Flowblock {}", name));
    }


    pub fn run(&mut self){
        println!("Starting Simulation.");
        let start = std::time::SystemTime::now();
        //FOR NOW set agent in station manually
        if let Some(block) = self.flowblocks.get_mut(0){
            block.take_agent(self.flowagents.remove(0));
        }

        //Process all
        for i in 0..self.time_controller.get_max_time_steps(){
            // Work on the stations and return workload in that timestep
            self.flowblocks.par_iter_mut().for_each(|flowblock| {
                flowblock.process(1.);
                
            });

            
            self.time_controller.timestep();
            if i%10 == 0{
                let time_since_start = start.elapsed().unwrap().as_secs_f32();
                println!("{} - 10 Timesteps.", time_since_start);
            }
        }

        let time_since_start = start.elapsed().unwrap().as_secs_f32();
        println!("{} - Simulation Finished", time_since_start);
    }

    pub fn set_timesteps(&mut self, timesteps: u64){
        self.time_controller.set_max_time_steps(timesteps);
    }

    pub fn show_state(&self){
        println!("{:#?}", self.flowblocks);
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    fn setup() -> SIM{
        let mut sim = SIM::new();
        sim.set_timesteps(11);
        sim.add_single_station("Groot");
        sim.add_single_station("Am");
        //sim.add_single_station("I");

        let block = sim.get_mut_flowblock_by_name("Groot").expect("main::Could not find Station Groot.");
        block.set_process_time(10.);

        let flowagent = FlowAgent::new();
        sim.add_flowagent(flowagent);

        sim.run();

        return sim;
    }

    #[test]
    fn test_takting(){
        let sim = setup();

        sim.show_state();

    }

    #[test]
    fn test_takt(){
        let mut sim = SIM::new();
        sim.set_timesteps(11);
        sim.add_single_station("Groot");
        sim.add_single_station("Am");

        let block = sim.get_mut_flowblock_by_name("Groot").expect("main::Could not find Station Groot.");
        block.set_process_time(10.);
        
        let flowagent = FlowAgent::new();
        sim.add_flowagent(flowagent);
        //sim.add_flowagent(flowagent);

        sim.run();
        sim.show_state();

        assert!(sim.get_flowbock_by_name("Am").expect("Could not find Station Am").get_agent_by_index_read(0).is_some());
    }
}