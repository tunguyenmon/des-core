pub mod station_single;
mod processor;
mod disruptor;

#[derive(Debug)]
pub struct Working;
#[derive(Debug)]
pub struct Idle;
#[derive(Debug)]
pub struct PartDone;

use std::fmt::Debug;
use crate::entity::Entity;
use crate::flowagent::FlowAgent;
pub trait FlowBlock: Debug + Entity{
    fn move_agent_to(&mut self, agent: FlowAgent, destination: &mut dyn FlowBlock);
    fn set_process_time(&mut self, process_time: f64);
    fn get_agent_by_index_read(&self, index: usize) -> &Option<FlowAgent>;
    fn take_agent(&mut self, agent: FlowAgent);
    fn process(&mut self, given_time: f64);
}