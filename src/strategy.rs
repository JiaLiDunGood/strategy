//如果设置窗口为Windows= 5, 那么前Windows天没有数据显示. 只有从第Windows+1天开始才有数据显示. 公式为前Windows天价格之和 / Windows 
pub fn sma(){
    let stock_prices = [45.2,46.1,54.2,47.2,54.1,45.4,44.1,46.4,41.7,48.3];

    //移动平均窗口大小
    let windows_size = 5;
    let mut sma_values:Vec<f64> = Vec::new();

    //计算sma容器大小,根据设置的windows窗口计算
    for i in 0..stock_prices.len() - windows_size +1{
        let window = &stock_prices[i..i+windows_size]; //第一次取值为0..Windows天的范围数据, 第二天就是1..windows+1,第ntian就是 n..windows+n
        let sum:f64 = window.iter().sum();
        let sma = sum / windows_size as f64;
        sma_values.push(sma);
    }

    //打印sma平均移动数据
    for (i,sma) in sma_values.iter().enumerate()  {
        println!("Day:{} : {:.2}",i+windows_size,sma);
    }
}
//ema计算公式为  当前时刻的ema值 = (当前时刻的价格 * 平滑因子) + (前一刻的ema值 * (1 - 平滑因子))  或者 (当前价格 - 前一刻ema) * 平滑因子 + 前一刻ema值
pub fn ema(){
    let stock_prices = [45.2,46.1,54.2,47.2,54.1,45.4,44.1,46.4,41.7,48.3];

    //时间窗口长度
    let windows_size = 5;
    let mut ema_values:Vec<f64> = Vec::new();

    //计算平滑因子,  alpha = 2 / (n +1)  n= 时间窗口长度
    let alpha = 2.0 / (windows_size as f64 + 1.0);

    //初始ema值就是第一个价格
    let mut ema = stock_prices[0];

    for price in &stock_prices{
        ema = (price -ema) * alpha + ema;  //这里记录当前的ema值,下一次计算就是前次的ema值
        ema_values.push(ema);
    }

    //打印e ma平均移动数据
    for (i,ema) in ema_values.iter().enumerate()  {
        println!("Day:{} : {:.2}",i+1,ema);

    }
}
/// rsi是分析市场超买的超卖的指标,公式是 100 - (100 / (1.0 + rs)), 其中rs = 周期范围内上涨价格幅度之和的平均值  / 周期范围内下跌价格幅度之和的平均值
pub fn rsi(up_days: Vec<f64>, down_days: Vec<f64>) -> f64{
    let up_sum = up_days.iter().sum::<f64>();
    let down_sum = down_days.iter().sum::<f64>();
    let rs = up_sum / down_sum;
    let rsi = 100.0 - (100.0 / (1.0 + rs));
    rsi
}