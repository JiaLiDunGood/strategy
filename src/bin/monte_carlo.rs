use std::env;

use rand::Rng;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() !=2 {
        eprintln!("Usage: cargo run  --bin monte_carlo <num_simulations>");
    }
    let num_simulation:usize = args[1].parse().expect("Invalid number of simulations");
    let portfolio_value = 1000000.0;  //初始投资额
    let expected_return = 0.08;     //预期收益率
    let risk = 0.15;                //年华风险(基差率)

    let mut rng = rand::thread_rng();
    let mut total_return = Vec::new();

    for _ in 0..num_simulation {
        // 使用蒙特卡洛模拟生成投资组合的未来收益率
        let random_return = rng.gen_range(-risk..risk);
        let portfolio_return = expected_return + random_return;
        let new_portfolio_value = portfolio_value * (1.0 + portfolio_return);
        total_return.push(new_portfolio_value);
    }
    //这里执行风险评估,生成报告或者其他任务
    let average_return: f64 = total_return.iter().sum::<f64>() / num_simulation as f64;
    println!("Average Portfolio Return: {:.2}%", (average_return - 1.0) * 100.0);
}