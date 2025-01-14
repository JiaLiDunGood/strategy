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