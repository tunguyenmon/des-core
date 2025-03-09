use crate::entity::Entity;
use crate::flowblock::processor::ProcessorSingle;
use crate::flowblock::FlowBlock;
use crate::flowagent::FlowAgent;
use crate::flowblock::disruptor::Disruptor;
use crate::flowblock::*;

#[derive(Debug)]
pub struct StationSingle<State: std::fmt::Debug>{
    name: String,
    id: u32,
    processor: ProcessorSingle,
    disruptor: Disruptor,
    agent: Option<FlowAgent>,
    _state: std::marker::PhantomData<State>,
}



trait StationState: Entity{
    fn get_name(&self) -> String;
    fn get_id(&self) -> u32;
}

impl<State: std::fmt::Debug> Entity for StationSingle<State>{
    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn get_id(&self) -> u32 {
        return self.id;
    }
}

impl StationSingle<Idle>{
    /// Processes the part currently in the station. Includes checks for whether
    /// station has an agent and also whether the part is already done.
    /// # Arguments
    /// 
    /// * `given_time` - The given time to process
    /// 
    /// # Returns
    /// 
    /// The remaining time after processing with the given time. 
    pub fn process_part(&mut self, given_time: f64) -> f64{
        if self.agent.is_none(){
            //println!("flowblock::station_single::processPart() - {} has no agent.", self.get_name());
            return given_time;
        }
        if self.part_done() {
            return given_time;
        }

        let task_remaining_time = self.processor.get_remaining_time();
        let remaining_time = task_remaining_time - given_time;

        if remaining_time <= 0.0 {
            self.processor.set_remaining_time(0.0);
            return -remaining_time;
        } else {
            self.processor.set_remaining_time(remaining_time);
            return 0.0;
        }
    }

    pub fn set_process_time(&mut self, time: f64){
        self.processor.set_process_time(time);
    }

    pub fn end_process(&mut self) -> () {
        self.processor.set_remaining_time(0.0);
    }

    pub fn take(&mut self, agent: FlowAgent) -> () {
        self.agent = Some(agent);
    }

    pub fn part_done(&self) -> bool{
        //If part is inside and processor has 0 time remaining
        self.processor.get_remaining_time() == 0.0 && self.agent.is_some()
    }

    pub fn remove_agent(&mut self){
        self.agent = None;
        self.reset_process_time();

    }

    pub fn reset_process_time(&mut self){
        self.processor.set_remaining_time(self.processor.get_process_time());
    }

    pub fn move_first_agent_to(&mut self, destination: &mut dyn FlowBlock){
        if let Some(agent) = self.agent.take(){
            destination.take_agent(agent);  
        }
        else{
            println!("No Agent in station {}", self.name);
        }
    }
}

impl<State: std::fmt::Debug> FlowBlock for StationSingle<State>{
    fn move_agent_to(&mut self, agent: FlowAgent, destination: &mut dyn FlowBlock) {
        destination.take_agent(agent);
    }

    fn set_process_time(&mut self, process_time: f64) {
        self.processor.set_process_time(process_time);
        self.processor.set_remaining_time(process_time);
    }

    fn get_agent_by_index_read(&self, _index: usize) -> &Option<FlowAgent> {
        &self.agent
    }

    fn take_agent(&mut self, agent:FlowAgent){
        self.agent = Some(agent);
    }

    fn process(&mut self, given_time: f64){
        //Need to check for Takt
        if self.processor.get_remaining_time()-given_time <= 0.{
        }
        self.process_part(given_time);
    }
}

impl<State: std::fmt::Debug> StationSingle<State>{
    pub fn new(name: &str) -> StationSingle<Idle>{
        StationSingle::<Idle>{
            name: name.to_string(),
            id: 0,
            processor: ProcessorSingle::new(),
            disruptor: Disruptor::new(),
            agent: None,
            _state: std::marker::PhantomData
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn process_one_part_successfully() -> () {
        let mut station: StationSingle<Idle> = StationSingle::<Idle>::new("station");
        let agent = FlowAgent::new();

        station.take(agent);
        station.set_process_time(3.);
        station.process(3.);

        assert!(station.part_done());
    }

    #[test]
    fn process_one_part_unsuccessfully() -> () {
        let mut station = StationSingle::<Idle>::new("station");
        let agent = FlowAgent::new();

        station.take(agent);
        station.set_process_time(3.);
        station.reset_process_time();
        station.process(2.);

        assert!(!station.part_done());
    }

    #[test]
    fn multiple_process_one_part_completely() -> () {
        let mut station = StationSingle::<Idle>::new("station");
        let agent = FlowAgent::new();

        station.take(agent);
        station.set_process_time(3.);
        station.process(1.);
        station.process(1.);
        station.process(1.);
    
        assert!(station.part_done());
    }

    #[test]
    fn multiple_process_one_part_not_completely() -> () {
        let mut station = StationSingle::<Idle>::new("station");
        let agent = FlowAgent::new();

        station.take(agent);
        station.set_process_time(3.);
        station.reset_process_time();
        station.process(1.);
        station.process(1.);
    
        assert!(!station.part_done());
    }

    #[test]
    fn process_multiple_parts() -> (){
        let mut station = StationSingle::<Idle>::new("station");
        let agent1 = FlowAgent::new();
        let agent2 = FlowAgent::new();
        let agent3 = FlowAgent::new();

        station.take(agent1);
        station.set_process_time(3.);
        station.process(3.);
        
        assert!(station.part_done());
        station.remove_agent();

        station.set_process_time(1.);
        station.take(agent2);
        station.reset_process_time();
        station.process(1.);

        assert!(station.part_done());

        station.remove_agent();
        station.take(agent3);
        assert!(!station.part_done());
        station.reset_process_time();
        station.process(1.);
        assert!(station.part_done());


    }

}

