use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Pulse {
    High = 1,
    Low = 0,
}

struct Communication {
    source: String,
    destination: String,
    state: Pulse,
}

trait HandleCommunication {
    fn handle_communication(&mut self, communication: &Communication) -> Vec<Communication>;
}

// impl FromStr for Module {
//     type Err = ();
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//     }
// }
//

#[derive(Debug, PartialEq, Eq)]
enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq, Eq)]
struct FlipFlopModule {
    id: String,
    state: State,
    destinations: Vec<String>,
}
impl FlipFlopModule {
    fn new(id: String, destinations: Vec<String>) -> Self {
        Self {
            id,
            state: State::Off,
            destinations,
        }
    }
}

impl HandleCommunication for FlipFlopModule {
    fn handle_communication(&mut self, communication: &Communication) -> Vec<Communication> {
        match communication.state {
            // If we get a high pulse, we don't do anything.
            Pulse::High => {
                vec![]
            }
            // If we get a low pulse, we flip our state and send a
            // pulse to our destinations.
            Pulse::Low => match self.state {
                State::On => {
                    self.state = State::Off;
                    self.destinations
                        .iter()
                        .map(|d| Communication {
                            source: self.id.to_owned(),
                            destination: d.to_owned(),
                            state: Pulse::Low,
                        })
                        .collect()
                }
                State::Off => {
                    self.state = State::On;
                    self.destinations
                        .iter()
                        .map(|d| Communication {
                            source: self.id.to_owned(),
                            destination: d.to_owned(),
                            state: Pulse::High,
                        })
                        .collect()
                }
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ConjunctionModule {
    id: String,
    inputs: HashMap<String, Pulse>,
    destinations: Vec<String>,
}

impl ConjunctionModule {
    fn new(id: String, inputs: Vec<String>, destinations: Vec<String>) -> Self {
        // Default all inputs to low.
        let inputs = inputs.into_iter().map(|i| (i, Pulse::Low)).collect();
        ConjunctionModule {
            id,
            inputs,
            destinations,
        }
    }
}

impl HandleCommunication for ConjunctionModule {
    fn handle_communication(&mut self, communication: &Communication) -> Vec<Communication> {
        // Update our inputs.
        self.inputs
            .insert(communication.source.clone(), communication.state);

        // Figure out what pulse to send. It will be low if all the
        // inputs are high.
        let pulse_to_send = if self.inputs.values().all(|p| *p == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };

        // Send the pulse to our destinations.
        self.destinations
            .iter()
            .map(|d| Communication {
                source: self.id.to_owned(),
                destination: d.to_owned(),
                state: pulse_to_send,
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BroadcastModule {
    id: String,
    destinations: Vec<String>,
}
impl BroadcastModule {
    fn new(name: String, destinations: Vec<String>) -> Self {
        Self {
            id: name,
            destinations,
        }
    }
}

impl HandleCommunication for BroadcastModule {
    fn handle_communication(&mut self, communication: &Communication) -> Vec<Communication> {
        self.destinations
            .iter()
            .map(|d| Communication {
                source: self.id.to_owned(),
                destination: d.to_owned(),
                state: communication.state,
            })
            .collect()
    }
}

struct Configuration {
    // Map of module name to module. The all implement
    // HandleCommunication which allows us to collect them like this.
    map: HashMap<String, Box<dyn HandleCommunication>>,

    // This is tracked for the mermaid output. Otherwise, not helpful
    // to the algorithm.
    modules: HashMap<String, Vec<String>>,
}

impl Configuration {
    fn parse_module(input: &str) -> (char, String, Vec<String>) {
        // The first part is the name, the second part is the
        // destinations.
        let parts = input.split(" -> ").collect::<Vec<_>>();

        // Determine the type from the first character. We'll remove
        // the first character from the name if it's not the
        // broadcaster.
        let (type_, name) = match parts[0].chars().next().unwrap() {
            'b' => ('b', parts[0].to_owned()),
            '%' => ('%', parts[0][1..].to_owned()),
            '&' => ('&', parts[0][1..].to_owned()),
            _ => panic!("Unknown type"),
        };

        // The destinations are just comma separated.
        let destinations = parts[1]
            .split(", ")
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        (type_, name, destinations)
    }

    fn parse(input: &str) -> Configuration {
        let mut map: HashMap<String, Box<dyn HandleCommunication>> = HashMap::new();

        // Get all of our modules.
        let modules = input.lines().map(Self::parse_module).collect::<Vec<_>>();

        // We also want a list of inputs per destination, for the
        // conjunctions.
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
        for (_, name, destinations) in &modules {
            for destination in destinations {
                inputs
                    .entry(destination.to_owned())
                    .or_default()
                    .push(name.to_owned());
            }
        }

        // For each module, create the appropriate type and add it to
        // our map.
        for (type_, name, destinations) in modules.clone() {
            let module: Box<dyn HandleCommunication> = match type_ {
                'b' => Box::new(BroadcastModule::new(name.to_owned(), destinations)),
                '%' => Box::new(FlipFlopModule::new(name.to_owned(), destinations)),
                '&' => Box::new(ConjunctionModule::new(
                    name.to_owned(),
                    inputs.get(&name.to_owned()).unwrap().clone(),
                    destinations,
                )),
                _ => panic!("Unknown type {}", type_),
            };
            map.insert(name, module);
        }

        // We also want a list of destinations per module, for the
        // mermaid output.
        let modules = modules
            .into_iter()
            .map(|(_, name, destinations)| (name, destinations))
            .collect::<HashMap<_, _>>();

        Configuration { map, modules }
    }

    fn push<F>(&mut self, mut helper_fn: F) -> (usize, usize)
    where
        F: FnMut(&Communication),
    {
        // We are tracking lows and highs for part 1.
        let mut low = 0;
        let mut high = 0;

        // Keep track of all the work we have to do, starting with the
        // button being pushed. We use a VecDeque because the problem
        // states we need to handle communications in order. So we
        // want grab from the front and push to the back.
        let mut queue: VecDeque<Communication> = VecDeque::new();
        queue.push_back(Communication {
            source: "button".to_owned(),
            destination: "broadcaster".to_owned(),
            state: Pulse::Low,
        });

        // Loop through all the work until we are done.
        while let Some(communication) = queue.pop_front() {
            // Update our trackers.
            match communication.state {
                Pulse::High => high += 1,
                Pulse::Low => low += 1,
            }

            // If we don't have a module for the destination, we can move on.
            let module = match self.map.get_mut(&communication.destination) {
                Some(m) => m,
                None => {
                    continue;
                }
            };

            // Call our helper function for part 2.
            helper_fn(&communication);

            // Handle the communication and extend our queue with any
            // new communications.
            let new_communications = module.handle_communication(&communication);
            queue.extend(new_communications);
        }

        // We are done with all the signal handling, return our highs
        // and lows.
        (low, high)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut configuration = Configuration::parse(input);

    let (lows, highs) = (0..1_000)
        .map(|_| configuration.push(|_| ()))
        .fold((0, 0), |(l1, h1), (l2, h2)| (l1 + l2, h1 + h2));

    Some(lows * highs)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(20);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 20));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 20));
        assert_eq!(result, None);
    }
}
