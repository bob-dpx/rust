mod schema;

use std::future::Future;

use async_std::task::block_on;

async fn lesson() -> String { String::from("从0到Go语言微服务架构师") }

fn study1() -> impl Future<Output = String> {
    async {
        let x = lesson().await;
        x+" 学习目标"
    }
}

fn go_study() -> impl Future<Output = String> {
    let r = |x: String| async move {
        let y:String = study1().await;
        y + &*x
    };
    r(String::from(":全面掌握Go语言微服务落地，代码级一次性解决微服务和分布式系统。"))
}
fn main() {
    let result = block_on(go_study());
    println!("{}", result);
}

