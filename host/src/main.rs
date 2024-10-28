use hex;
use inputs::{pages::Log, Inputs};
use methods::{METHOD_ELF, METHOD_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

const STEPS_ENV_VAR: &str = "VERIFY_STEPS";
const START_HASH_ENV_VAR: &str = "VERIFY_START_HASH";
const END_HASH_ENV_VAR: &str = "VERIFY_END_HASH";
const LOG_PATH_ENV_VAR: &str = "VERIFY_LOG_PATH";

fn read_inputs() -> Inputs {
    let steps = std::env::var(STEPS_ENV_VAR)
        .expect("VERIFY_STEPS environment variable not set")
        .parse()
        .expect("VERIFY_STEPS environment variable is not a number");

    let start_hash = hex::decode(
        std::env::var(START_HASH_ENV_VAR).expect("VERIFY_START_HASH environment variable not set"),
    )
    .expect("VERIFY_START_HASH environment variable is not a valid hex string");

    let end_hash = hex::decode(
        std::env::var(END_HASH_ENV_VAR).expect("VERIFY_END_HASH environment variable not set"),
    )
    .expect("VERIFY_END_HASH environment variable is not a valid hex string");

    let log_path =
        std::env::var(LOG_PATH_ENV_VAR).expect("VERIFY_LOG_PATH environment variable not set");

    let log = Log::read(log_path);

    Inputs {
        steps,
        start_hash,
        end_hash,
        log,
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let inputs = read_inputs();

    let env = ExecutorEnv::builder()
        .write(&inputs)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    let prove_info = prover.prove(env, METHOD_ELF).unwrap();
    let receipt = prove_info.receipt;
    dbg!(&receipt);

    // TODO: Implement code for retrieving receipt journal here.

    // For example:
    //let _output: u32 = receipt.journal.decode().unwrap();

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    //receipt.verify(METHOD_ID).unwrap();
}
