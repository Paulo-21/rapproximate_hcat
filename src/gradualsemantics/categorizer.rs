use std::collections::VecDeque;
use crate::{graph::ArgumentationFramework, parser::Task};

const EPSILON : f64 = 0.0001;

pub fn solve(af : ArgumentationFramework, task : &Task) -> f64 {
    //let score = computeFinalScore(&af);
    //let solution = score[task.argument];
	let solution= computeFinalScore2(&af, task.argument);
	//let solution= compute_final_score2_deep(&af, task.argument);
    solution
}

fn computeFinalScore(af : &ArgumentationFramework) -> Vec<f64> {
    let mut res = initScores(af);
    let mut newScores = initScores(af);
    let mut hasChanged = true;
    
		while hasChanged {
			/*(newScores, hasChanged) =*/ computeOneStep(af,&res, &mut newScores);
			if stabilisation(&res,&newScores) {
				hasChanged = false;
			}
            let temp = res;
			res = newScores;
            newScores = temp;
		}
		return res;
}

fn computeOneStep(af : &ArgumentationFramework, scoresArg : &Vec<f64>, res : &mut Vec<f64>) {//-> (Vec<f64>, bool) {
    //let mut res = vec![0.;scoresArg.len()];
    //let mut res = Vec::with_capacity(scoresArg.len());
    //let mut haschanged = true;
		for i in 0..scoresArg.len() {
			let mut sumScoreAttacker = 0.;
			for  attacker in &af.af_attacker[i] {
                unsafe {
                    sumScoreAttacker += scoresArg.get_unchecked(*attacker as usize);
                }
			}
			res[i] =  1. / (1. + sumScoreAttacker);
            //haschanged = (res[i] - scoresArg[i]).abs() > EPSILON;
		}
		//return (res, haschanged);
}
fn computeFinalScore2( af : &ArgumentationFramework, task_argument : usize) -> f64 {
	let mut nb_hit = 0;
	let mut index_to_hit = Vec::with_capacity(af.nb_argument);
	let mut never_hit = vec![true;af.nb_argument];
	let mut scores_arg : Vec<f64> = vec![1.;af.nb_argument]; 
	index_to_hit.push(task_argument);
	let mut old_score_t_arg = 0.;
	loop  {
		while nb_hit < index_to_hit.len() {
			let arg = index_to_hit[nb_hit];
			let mut sum_score_attacker = 0.;
			for new_arg in &af.af_attacker[arg] {
				if never_hit[*new_arg as usize] {
					index_to_hit.push(*new_arg as usize);
					never_hit[*new_arg as usize] = false;
				}
				sum_score_attacker += scores_arg[*new_arg as usize];
			}
			scores_arg[arg] = 1. / (1. + sum_score_attacker);
			nb_hit+=1;
		}
		index_to_hit = Vec::with_capacity(af.nb_argument);
		never_hit = vec![true; af.nb_argument];
		nb_hit = 0;
		index_to_hit.push(task_argument);
		if (old_score_t_arg - scores_arg[task_argument]).abs() <= EPSILON {
			break;
		}
		old_score_t_arg = scores_arg[task_argument];
	}
	old_score_t_arg
}
/*fn compute_final_score2_deep( af : &ArgumentationFramework, task_argument : usize) -> f64 {
	let mut nb_hit = 0;
	let mut index_to_hit = VecDeque::with_capacity(af.nb_argument);
	let mut never_hit = vec![true;af.nb_argument];
	let mut scores_arg : Vec<f64> = vec![1.;af.nb_argument];
	index_to_hit.push_back(task_argument);
	let mut old_score_t_arg = 0.;
	loop  {
		let mut pushed = false;
		while !pushed {
			pushed = false;
			let arg = index_to_hit[0];
			let mut sum_score_attacker = 0.;
			for new_arg in &af.af_attacker[arg] {
				if never_hit[*new_arg as usize] {
					index_to_hit.push_front(*new_arg as usize);
					never_hit[*new_arg as usize] = false;
					pushed = true;
				}
			}
		}
		let arg = index_to_hit.pop_front().unwrap();
		for new_arg in &af.af_attacker[arg] {
			sum_score_attacker += scores_arg[*new_arg as usize];
		}
		scores_arg[arg] = 1. / (1. + sum_score_attacker);
		nb_hit+=1;
		index_to_hit = VecDeque::with_capacity(af.nb_argument);
		never_hit = vec![true; af.nb_argument];
		nb_hit = 0;
		index_to_hit.push_back(task_argument);
		if (old_score_t_arg - scores_arg[task_argument]).abs() <= EPSILON {
			break;
		}
		old_score_t_arg = scores_arg[task_argument];
	}
	old_score_t_arg
}*/
fn compute_final_score_rec (af : &ArgumentationFramework) {

}
fn initScores(af : &ArgumentationFramework) -> Vec<f64> {
    vec![1.0;af.nb_argument]
}

fn stabilisation(tab1 : &Vec<f64>, tab2 : &Vec<f64>) -> bool {
	for (i, x) in tab1.into_iter().enumerate() {
		if (x-tab2[i]).abs() > EPSILON {
			return false;
		}
	}
	return true;
}