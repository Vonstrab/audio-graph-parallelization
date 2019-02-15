use scheduling::timeslot::TimeSlot;

#[derive(Clone)]
pub struct Processor {
    pub time_slots: Vec<TimeSlot>,
    completion_time: f64,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            time_slots: Vec::new(),
            completion_time: 0.0,
        }
    }

    pub fn add_timeslot(&mut self, node: usize, start_time: f64, completion_time: f64) -> bool {
        //this condition expect that appending TimeSlot is incremental
        if self.completion_time <= start_time {
            self.time_slots
                .push(TimeSlot::new(node, start_time, completion_time));
            self.completion_time = completion_time;
            return true;
        }
        return false;
    }

    pub fn get_completion_time(&self) -> f64 {
        self.completion_time
    }
}
