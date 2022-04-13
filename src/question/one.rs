// 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同

struct Light {
    color: String,
}

trait TrafficRules {
    fn wait_time(&self) -> Option<u32>;
}

impl TrafficRules for Light {
    fn wait_time(&self) -> Option<u32> {
        match &self.color as &str {
            "Green" => Some(30),
            "Yellow" => Some(30),
            "Red" => Some(60),
            _ => None,
        }
    }
}

pub fn enum_traffic_light(color: String) {
    let light = Light { color };

    let time = light.wait_time().unwrap();

    println!("{} 交通灯时间是 {}秒", light.color, time);
}