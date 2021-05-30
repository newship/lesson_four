// 作业一：为枚举交通信号灯实现一个trait，trait里包含一个返回时间的方法，不同灯持续时间不同

pub trait TrafficLightTrait {
    fn time(&self) -> u8;
  }

pub enum Trafficlight {
	RED, 
    YELLOW, 
    GREEN,
}

impl TrafficLightTrait for Trafficlight {
    fn time(&self) -> u8 {
        match self {
            Trafficlight::RED  => 30,
            Trafficlight::YELLOW  => 5,
            Trafficlight::GREEN  => 60,
        }
    }
}