use stockfish::uciengine::{GoJob, Timecontrol, UciEngine};
use tokio::join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let go_job1 = GoJob::new()
        .uci_opt("UCI_Variant", "atomic")
        .uci_opt("Hash", 128)
        .uci_opt("Threads", 40)
        .pos_fen("k7/8/8/8/8/8/R7/7K w - - 0 1")
        .pos_moves("h1h2")
        .tc(Timecontrol {
            wtime: 15000,
            winc: 0,
            btime: 15000,
            binc: 0,
        });

    let go_job2 = GoJob::new()
        .uci_opt("UCI_Variant", "chess")
        .pos_startpos()
        .go_opt("depth", 12);

    let engine = UciEngine::new("stockfish");

    // make two clones of the engine, so that we can move them to async blocks
    let (engine_clone1, engine_clone2) = (engine.clone(), engine.clone());

    // issue two parallel async go commands, to demonstrate that they will be queued and processed one a time
    let job1 = tokio::spawn(async move {
        let go_result1 = engine_clone1.go(go_job1).await;

        println!("go result 1 {:?}", go_result1);
    });

    let job2 = tokio::spawn(async move {
        let go_result2 = engine_clone2.go(go_job2).await;

        println!("go result 2 {:?}", go_result2);
        go_result2
    });

    join!(job1, job2);
    // wait enough for the go commands to complete in the background
    //tokio::time::sleep(tokio::time::Duration::from_millis(20000)).await;

    // quit engine
    engine.quit();

    // wait for engine to quit gracefully
    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;

    Ok(())
}
