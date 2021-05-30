mod traffic_light;
mod sum;
mod area;
use traffic_light::{ Trafficlight, TrafficLightTrait };
use area::{ Circle, Triangle, Square, get_area };


fn main() {
    // 具体实现分别写在traffic_light, mod, sum3个模块里面

    // 作业一：为枚举交通信号灯实现一个trait，trait里包含一个返回时间的方法，不同灯持续时间不同
    let red_light = Trafficlight::RED;
    let yellow_light = Trafficlight::YELLOW;
    let green_light = Trafficlight::GREEN;
    let red_light_time = red_light.time();
    let yellow_light_time = yellow_light.time();
    let green_light_time = green_light.time();
    println!("the time of red light, yellow light and green light is: {:?}, {:?}, {:?}", red_light_time,yellow_light_time,green_light_time);
   
    //作业二：实现一个函数，为u32类型的整数集合求和，参数类型为&[u32]，返回类型为Option<u32>，溢出时返回None
    let b1 = sum::sum(&[1,2,3,4,6,5,787,42,35,5634]);
    let b2 = sum::sum(&[1,2,3,4,6,5,787,4294967295]);
    println!("b1: {:?}, b2: {:?}", b1, b2);

    //作业三：实现一个打印图形面积的函数，接收一个可以计算面积的类型作为参数，需要用到泛型和泛型约束
    let a = Circle {
        r: 2.0
    };
    println!("the area of this circle is: {}", get_area(a));
    let b = Triangle {
        b: 4.0,
        h: 3.0,
    };
    println!("the area of this triangle is: {}", get_area(b));

    let c = Square {
        a: 5.0,
    };
    println!("the area of this square is: {}", get_area(c));

}

 


