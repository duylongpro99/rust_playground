use std::time::Duration;

fn main() {
    trpl::run(async {
        // Spawn tasks
        println!(">>>>>Spawn tasks<<<<<<<");
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hello task {}", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
        for i in 1..5 {
            println!("hello {}", i);
            trpl::sleep(Duration::from_millis(500)).await;
        }
        // Join tasks
        println!(">>>>>Join tasks<<<<<<<");
        let fut1 = async {
            for i in 1..5 {
                println!("fut1 {}", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let fut2 = async {
            for i in 1..5 {
                println!("fut2 {}", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        trpl::join(fut1, fut2).await;

        // Channel - Message Passing
        println!(">>>>>Channel - Message Passing<<<<<<<");
        let (tx, mut rx) = trpl::channel();
        let val = String::from("Hi");
        tx.send(val).unwrap();
        let received = rx.recv().await.unwrap();
        println!("Got: {}", received);

        println!(">>>>>Channel - Multiple Messages Passing<<<<<<<");
        // let tx_fut = async {
        let vals = vec![
            String::from("Hi"),
            String::from("From"),
            String::from("The"),
            String::from("Future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
        // };

        // let rx_fut = async {
        while let Some(received) = rx.recv().await {
            println!("{}", received);
        }
        // };

        // trpl::join(tx_fut, rx_fut).await;
    });

    println!("End blocking");
}
