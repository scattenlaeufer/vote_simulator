use rand::prelude::*;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Candidate {
    Party(String),
    Independent(String),
    Nothing,
}

impl<'a> Hash for &'a Candidate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(*self, state)
    }
}

#[derive(Debug)]
struct Choice<'a> {
    candidate: &'a Candidate,
    probability: f32,
    preferences: HashMap<&'a Candidate, f32>,
}

#[derive(Debug)]
struct Vote<'a> {
    vote: &'a Candidate,
    next_vote: Option<&'a Self>,
}

pub fn run(voters: usize, positions: usize, constituencies: usize) {
    // Those candidates stand for election
    let party_a = Candidate::Party("A".into());
    let party_b = Candidate::Party("B".into());
    let party_c = Candidate::Party("C".into());
    let indy_z = Candidate::Independent("Z".into());

    // Define the likeliness of a candidate to be elected and the preferred
    // other candidates
    let choices_a_vec = vec![
        Choice {
            candidate: &party_a,
            probability: 0.3,
            preferences: {
                let mut map = HashMap::new();
                map.insert(&party_b, 0.1);
                map.insert(&party_c, 0.2);
                map.insert(&indy_z, 0.5);
                map.insert(&Candidate::Nothing, 0.2);
                map
            },
        },
        Choice {
            candidate: &party_b,
            probability: 0.4,
            preferences: {
                let mut map = HashMap::new();
                map.insert(&party_a, 0.1);
                map.insert(&party_c, 0.6);
                map.insert(&indy_z, 0.2);
                map.insert(&Candidate::Nothing, 0.1);
                map
            },
        },
        Choice {
            candidate: &party_c,
            probability: 0.1,
            preferences: {
                let mut map = HashMap::new();
                map.insert(&party_a, 0.1);
                map.insert(&party_b, 0.7);
                map.insert(&indy_z, 0.1);
                map.insert(&Candidate::Nothing, 0.1);
                map
            },
        },
        Choice {
            candidate: &indy_z,
            probability: 0.2,
            preferences: {
                let mut map = HashMap::new();
                map.insert(&party_a, 0.7);
                map.insert(&party_b, 0.1);
                map.insert(&party_c, 0.1);
                map.insert(&Candidate::Nothing, 0.1);
                map
            },
        },
        Choice {
            candidate: &Candidate::Nothing,
            probability: 0.,
            preferences: HashMap::new(),
        },
    ];

    let mut rng = thread_rng();

    // In this Vector the actual votes will be collected
    let mut election_results = Vec::new();

    let voters_per_constituency = voters / constituencies;

    // Do the actual voting
    for _ in 0..constituencies {
        let mut votes = Vec::new();
        for _ in 0..voters_per_constituency {
            // Select the primary vote
            let primary_choice = choices_a_vec
                .iter()
                .map(|c| (c, c.probability))
                .collect::<Vec<_>>()
                .choose_weighted(&mut rng, |i| i.1)
                .unwrap()
                .0;
            match primary_choice.candidate {
                &Candidate::Nothing => (),
                _ => {
                    let mut ballot = Vec::new();
                    let mut choices = primary_choice.preferences.clone();
                    ballot.push(primary_choice.candidate);
                    // Select subsequent votes
                    loop {
                        // Make a choice
                        match &choices
                            .iter()
                            .map(|c| c)
                            .collect::<Vec<_>>()
                            .choose_weighted(&mut rng, |i| i.1)
                            .unwrap()
                            .0
                        {
                            // If nothing was chosen, break the loop
                            &Candidate::Nothing => break,
                            // Else process the new choice
                            c => {
                                // Add the new choices to the ballot
                                ballot.push(c);
                                // Remove the current choice from the set of further possibilities
                                choices = choices
                                    .iter()
                                    .filter(|o| &o.0 != c)
                                    .map(|o| (o.0.clone(), *o.1))
                                    .collect::<HashMap<_, _>>();
                            }
                        };
                    }
                    votes.push(ballot);
                }
            }
        }
        election_results.push(votes);
    }

    for votes in election_results.iter() {
        let primary_votes = votes.iter().map(|i| i[0]).collect::<Vec<_>>();

        let mut vote_count = HashMap::new();
        for v in primary_votes {
            match vote_count.insert(v, 1) {
                None => (),
                Some(c) => {
                    vote_count.insert(v, c + 1);
                    ()
                }
            };
        }
        println!("{:#?}", vote_count);
    }
}
