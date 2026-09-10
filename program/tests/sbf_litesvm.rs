use std::{path::PathBuf, str::FromStr};

use litesvm::LiteSVM;
use solana_account::Account;
use solana_address::Address;
use solana_instruction::{account_meta::AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signer::Signer;
use solana_transaction::Transaction;

const PROGRAM: &str = "6Uzr4jz1SENxn3DdprQQ24zxaJXr6rThQNB48QxbuJed";

fn config(tag: u8, bump: u8, max_daily_trades: u16) -> Vec<u8> {
    let mut data = vec![0u8; 67];
    data[0] = tag;
    data[1] = bump;
    data[2] = 2;
    data[3..9].copy_from_slice(b"SBF v2");
    data[27..29].copy_from_slice(&6_000u16.to_le_bytes());
    data[29..31].copy_from_slice(&4_000u16.to_le_bytes());
    data[43..45].copy_from_slice(&8_000u16.to_le_bytes());
    data[45..47].copy_from_slice(&9_000u16.to_le_bytes());
    data[59..61].copy_from_slice(&500u16.to_le_bytes());
    data[61..63].copy_from_slice(&50u16.to_le_bytes());
    data[63..65].copy_from_slice(&max_daily_trades.to_le_bytes());
    data[65..67].copy_from_slice(&2_000u16.to_le_bytes());
    data
}

fn execution(id: u64, input: u64, output: u64, fee: u64, nav: u64) -> Vec<u8> {
    let mut data = vec![0u8; 43];
    data[0] = 3;
    data[1..9].copy_from_slice(&id.to_le_bytes());
    data[9..17].copy_from_slice(&input.to_le_bytes());
    data[17..25].copy_from_slice(&output.to_le_bytes());
    data[25..33].copy_from_slice(&fee.to_le_bytes());
    data[33] = 0;
    data[34] = 0;
    data[35..43].copy_from_slice(&nav.to_le_bytes());
    data
}

fn ix(program_id: Address, authority: Address, state: Address, data: Vec<u8>) -> Instruction {
    let is_execution = data.first() == Some(&3);
    let mut accounts = vec![
        AccountMeta::new_readonly(authority, true),
        AccountMeta::new(state, false),
    ];
    if is_execution {
        accounts.push(AccountMeta::new_readonly(
            Address::from_str("SysvarC1ock11111111111111111111111111111111").unwrap(),
            false,
        ));
    }
    Instruction {
        program_id,
        accounts,
        data,
    }
}

fn send(svm: &mut LiteSVM, payer: &Keypair, instruction: Instruction) -> bool {
    svm.expire_blockhash();
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let tx = Transaction::new(&[payer], message, svm.latest_blockhash());
    svm.send_transaction(tx).is_ok()
}

fn state_data(svm: &LiteSVM, state: &Address) -> Vec<u8> {
    svm.get_account(state).unwrap().data
}

#[test]
fn final_sbf_executes_all_instructions_and_rolls_back_attacks() {
    let program_id = Address::from_str(PROGRAM).unwrap();
    let mut so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    so_path.push("../target/deploy/xstocks_autopilot.so");
    assert!(so_path.exists(), "run cargo build-sbf before this test");

    let mut svm = LiteSVM::new();
    svm.add_program_from_file(program_id, &so_path).unwrap();
    let authority = Keypair::new();
    let attacker = Keypair::new();
    svm.airdrop(&authority.pubkey(), 5_000_000_000).unwrap();
    svm.airdrop(&attacker.pubkey(), 5_000_000_000).unwrap();
    let (state, bump) =
        Address::find_program_address(&[b"portfolio", authority.pubkey().as_ref()], &program_id);

    // Initialize rejects an incorrect PDA and accepts the canonical PDA.
    let wrong_pda = Address::new_unique();
    let bad_init = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(authority.pubkey(), true),
            AccountMeta::new(wrong_pda, false),
            AccountMeta::new_readonly(Address::default(), false),
        ],
        data: config(0, bump, 2),
    };
    assert!(!send(&mut svm, &authority, bad_init));
    assert!(svm.get_account(&wrong_pda).is_none());

    let initialize = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(authority.pubkey(), true),
            AccountMeta::new(state, false),
            AccountMeta::new_readonly(Address::default(), false),
        ],
        data: config(0, bump, 2),
    };
    assert!(send(&mut svm, &authority, initialize));
    assert_eq!(&state_data(&svm, &state)[..8], b"XSTOCKS1");

    // A modified replay proves duplicate initialization is rejected rather than
    // being transaction-signature deduplicated.
    let mut duplicate_data = config(0, bump, 2);
    duplicate_data[3] = b'X';
    let duplicate = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(authority.pubkey(), true),
            AccountMeta::new(state, false),
            AccountMeta::new_readonly(Address::default(), false),
        ],
        data: duplicate_data,
    };
    let initialized = state_data(&svm, &state);
    assert!(!send(&mut svm, &authority, duplicate));
    assert_eq!(state_data(&svm, &state), initialized);

    // UpdateStrategy succeeds, while forged authority and malformed accounts fail.
    assert!(send(
        &mut svm,
        &authority,
        ix(program_id, authority.pubkey(), state, config(1, 0, 2)),
    ));
    let after_update = state_data(&svm, &state);
    assert_eq!(
        u64::from_le_bytes(after_update[107..115].try_into().unwrap()),
        2
    );
    assert!(!send(
        &mut svm,
        &attacker,
        ix(program_id, attacker.pubkey(), state, config(1, 0, 2)),
    ));
    assert_eq!(state_data(&svm, &state), after_update);

    for (owner, len) in [(Address::default(), 160usize), (program_id, 159usize)] {
        let malformed = Address::new_unique();
        svm.set_account(
            malformed,
            Account {
                lamports: 2_000_000,
                data: vec![0; len],
                owner,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(!send(
            &mut svm,
            &authority,
            ix(program_id, authority.pubkey(), malformed, config(1, 0, 2)),
        ));
    }

    // Draft cannot jump to Paused, then Draft -> Running is accepted.
    assert!(!send(
        &mut svm,
        &authority,
        ix(program_id, authority.pubkey(), state, vec![2, 2]),
    ));
    assert!(send(
        &mut svm,
        &authority,
        ix(program_id, authority.pubkey(), state, vec![2, 1]),
    ));

    // Successful record, then duplicate id, zero output, excessive fee,
    // excessive slippage, and daily trade overflow all fail atomically.
    assert!(send(
        &mut svm,
        &authority,
        ix(
            program_id,
            authority.pubkey(),
            state,
            execution(1, 1_000_000, 996_000, 1_000, 10_000_000)
        ),
    ));
    let after_first = state_data(&svm, &state);
    for rejected in [
        execution(1, 1_000_000, 996_000, 1_000, 10_000_000),
        execution(2, 1_000_000, 0, 1_000, 10_000_000),
        execution(2, 1_000_000, 996_000, 1_000_001, 10_000_000),
        execution(2, 1_000_000, 900_000, 1_000, 10_000_000),
    ] {
        assert!(!send(
            &mut svm,
            &authority,
            ix(program_id, authority.pubkey(), state, rejected)
        ));
        assert_eq!(state_data(&svm, &state), after_first);
    }
    assert!(send(
        &mut svm,
        &authority,
        ix(
            program_id,
            authority.pubkey(),
            state,
            execution(2, 1_000_000, 996_000, 1_000, 10_000_000)
        ),
    ));
    let after_second = state_data(&svm, &state);
    assert!(!send(
        &mut svm,
        &authority,
        ix(
            program_id,
            authority.pubkey(),
            state,
            execution(3, 1, 1, 0, 10_000_000)
        ),
    ));
    assert_eq!(state_data(&svm, &state), after_second);

    // Running -> Paused -> Stopped succeeds; Stopped is terminal.
    assert!(send(
        &mut svm,
        &authority,
        ix(program_id, authority.pubkey(), state, vec![2, 2])
    ));
    assert!(send(
        &mut svm,
        &authority,
        ix(program_id, authority.pubkey(), state, vec![2, 5])
    ));
    let stopped = state_data(&svm, &state);
    assert!(!send(
        &mut svm,
        &authority,
        ix(program_id, authority.pubkey(), state, vec![2, 1])
    ));
    assert_eq!(state_data(&svm, &state), stopped);
}

#[test]
fn shared_golden_vectors_match_rust_layout() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("../../docs/golden-vectors.json")).unwrap();
    let mut record = execution(42, 1_000_000, 996_000, 1_000, 10_000_000);
    record[33] = 1;
    record[34] = 1;
    assert_eq!(hex::encode(record), vectors["record_execution_hex"]);
    let state = hex::decode(vectors["state_hex"].as_str().unwrap()).unwrap();
    assert_eq!(state.len(), 160);
    assert_eq!(
        u64::from_le_bytes(state[107..115].try_into().unwrap()),
        9_007_199_254_740_993
    );
    assert_eq!(u16::from_le_bytes(state[155..157].try_into().unwrap()), 7);
}
