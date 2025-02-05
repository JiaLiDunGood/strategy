use core::num;
use std::io::{self, Write};

///折现计算器,需要用户输入金额,折扣率,时间周期
pub fn compute(){
    let mut input = String::new();

    println!("折现计算器");


    //提示用户输入金额
    print!("请输入金额:");
    io::stdout().flush().expect("刷新失败"); //刷新输出流,确保立即显示
    io::stdin().read_line(& mut input).expect("读取失败");
    let principal: f64 = input.trim().parse().expect("无效的输入");

    input.clear(); //清空输入缓冲区,方便下次使用
     
    print!("请输入折现率(以小数形式:)");
    io::stdout().flush().expect("刷新失败");
    io::stdin().read_line(&mut input).expect("读取失败");
    let discount_rate: f64 = input.trim().parse().expect("无效的输入");

    input.clear(); //清空输入缓冲区,方便下次输入

    print!("请输入年限(以整数为单位)");
    io::stdout().flush().expect("刷新失败");
    io::stdin().read_line(&mut input).expect("读取失败");
    let time_period: u32 = input.trim().parse().expect("无效的输入");

    let result = claculate_present_value(principal,discount_rate,time_period);
    println!("现值为: {:.2}",result);

}
fn claculate_present_value(principal: f64,discount: f64, time_period: u32)-> f64{
    if discount < 0.0{
        eprint!("\n错误: 折现率不能为负数");
        eprintln!("\n请提供有效折现率");
        std::process::exit(1);
    }
    if time_period == 0 {
        eprint!("\n错误: 时间周期不能为0");
        eprintln!("\n请提供有效时间周期");
        std::process::exit(1);
    }
    principal / (1.0 +discount).powi(time_period as i32)
}

//投资组合的语气收益和风险率

struct Asset{
    expected_return: f64,
    risk: f64,
}

fn calculate_portfolio_metrics(assets: &[Asset],weights: &[f64]) ->(f64,f64){
    //计算预期收益率,
    let expected_return: f64 = assets
        .iter()
        .zip(weights.iter())
        .map(|(asset,weight)| asset.expected_return * weight)
        .sum::<f64>(); 

    //计算风险
    let portfolio_risk: f64 = assets
        .iter()
        .zip(weights.iter())
        .map(|(asset,weight)| asset.risk * asset.risk * weight * weight)
        .sum::<f64>();

    (expected_return,portfolio_risk)
}
//
fn optimize_with_algorithm<F>(_object_function: F ,initial_weight: Vec<f64>) ->Vec<f64> 
where 
    F: Fn(Vec<f64>) ->f64,
{
    //这里简化为均匀分配权重的实现,实际中需要使用优化算法
    initial_weight
}
//优化投资组合
fn optimize_portfolio(assets: &[Asset],risk_preference: f64) ->Vec<f64>{
    let object_function= |weights: Vec<f64>| -> f64{
        let (expected_return,portfolio_risk) =calculate_portfolio_metrics(&assets, &weights);
        expected_return - risk_preference * portfolio_risk //预期收益 - 风险偏好 * 投资组合风险
    };

    let num_assets = assets.len();
    let initial_weight = vec![1.0 / num_assets as f64; num_assets];
    let optimize_weight = optimize_with_algorithm(object_function, initial_weight);
    optimize_weight
}