use crate::{graph::ArgumentationFramework, parser::Task};

const EPSILON : f32 = 0.0001;

pub fn solve(af : ArgumentationFramework, task : &Task) -> f32 {
    let score = computeFinalScore(&af);
    let solution = score[task.argument];
    solution
}

fn computeFinalScore(af : &ArgumentationFramework) -> Vec<f32> {
    let mut res = initScores(af);
    let mut hasChanged = true;
		while hasChanged {
			let newScores = computeOneStep(af,&res);
			if stabilisation(&res,&newScores) {
				hasChanged = false;
			}
			res = newScores;
		}
		return res;
}

fn computeOneStep(af : &ArgumentationFramework, scoresArg : &Vec<f32>) -> Vec<f32> {
    let mut res = vec![0.;scoresArg.len()];
		for i in 0..scoresArg.len() {
			let mut sumScoreAttacker = 0.;
			for  attacker in &af.af_attacker[i] {
				sumScoreAttacker += scoresArg[*attacker as usize];
			}
			res[i] = 1. / (1. + sumScoreAttacker);
		}
		return res;
}

fn initScores(af : &ArgumentationFramework) -> Vec<f32> {
    vec![1.0;af.nb_argument]
}

fn stabilisation(tab1 : &Vec<f32>, tab2 : &Vec<f32>) -> bool {
	for (i, x) in tab1.into_iter().enumerate() {
		if (x-tab2[i]).abs() > EPSILON {
			return false;
		}
	}
	return true;
}