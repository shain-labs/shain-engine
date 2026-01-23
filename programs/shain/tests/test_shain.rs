use {
    anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas},
    shain::{
        constants::{SHAIN_CONFIG_SEED, SHAIN_SESSION_SEED, SHAIN_TREASURY_SEED},
        instructions::InitializeParams,
        state::{ShainConfig, ShainSession},
    },
    litesvm::LiteSVM,
    litesvm_token::{CreateAssociatedTokenAccount, CreateMint, MintTo},
    solana_address::Address,
    solana_instruction::Instruction,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

const SESSION_DURATION: i64 = 60 * 60 * 24;
const SESSION_FEE: u64 = 1_000_000;
const MIN_HOLDING: u64 = 10_000_000;
const MINT_AMOUNT: u64 = 100_000_000;

const ATA_PROGRAM_ID_BYTES: [u8; 32] = [
    140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131, 11, 90, 19, 153, 218,
    255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
];

fn ata_program_pubkey() -> Pubkey {
    Pubkey::new_from_array(ATA_PROGRAM_ID_BYTES)
}

fn spl_token_pubkey() -> Pubkey {
    Pubkey::new_from_array(spl_token::ID.to_bytes())
}

fn a2p(address: &Address) -> Pubkey {
    Pubkey::new_from_array(address.to_bytes())
}

fn p2a(pubkey: &Pubkey) -> Address {
    Address::new_from_array(pubkey.to_bytes())
}

fn load_program(svm: &mut LiteSVM, program_id: &Pubkey) {
    let bytes = include_bytes!("../../../target/deploy/shain.so");
    svm.add_program(p2a(program_id), bytes).unwrap();
}

fn shain_config_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[SHAIN_CONFIG_SEED], program_id)
}

fn shain_treasury_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[SHAIN_TREASURY_SEED], program_id)
}

fn shain_session_pda(program_id: &Pubkey, user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[SHAIN_SESSION_SEED, user.as_ref()], program_id)
}

fn ata_address(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    let ata_program = ata_program_pubkey();
    let token_program = spl_token_pubkey();
    let (ata, _) = Pubkey::find_program_address(
        &[owner.as_ref(), token_program.as_ref(), mint.as_ref()],
        &ata_program,
    );
    ata
}

fn send_ix(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
    extra_signers: &[&Keypair],
) -> Result<(), String> {
    svm.expire_blockhash();
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let mut signers: Vec<&Keypair> = Vec::with_capacity(extra_signers.len() + 1);
    signers.push(payer);
    for s in extra_signers {
        signers.push(*s);
    }
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &signers)
        .map_err(|e| format!("build tx: {e}"))?;
    svm.send_transaction(tx)
        .map(|_| ())
        .map_err(|e| format!("{:?}", e.err))
}

fn warp(svm: &mut LiteSVM, seconds: i64) {
    let mut clock = svm.get_sysvar::<solana_clock::Clock>();
    clock.unix_timestamp = clock.unix_timestamp.saturating_add(seconds);
    clock.slot = clock.slot.saturating_add((seconds.max(1) as u64) * 2);
    svm.set_sysvar(&clock);
}

struct Bootstrap {
    svm: LiteSVM,
    program_id: Pubkey,
    authority: Keypair,
    user: Keypair,
    mint: Pubkey,
    user_ata: Pubkey,
    shain_config: Pubkey,
    treasury_authority: Pubkey,
    treasury_ata: Pubkey,
}

fn bootstrap(min_holding: u64) -> Bootstrap {
    let program_id = shain::id();
    let mut svm = LiteSVM::new();
    load_program(&mut svm, &program_id);

    let payer = Keypair::new();
    let authority = Keypair::new();
    let user = Keypair::new();

    svm.airdrop(&p2a(&payer.pubkey()), 10_000_000_000).unwrap();
    svm.airdrop(&p2a(&authority.pubkey()), 10_000_000_000).unwrap();
    svm.airdrop(&p2a(&user.pubkey()), 10_000_000_000).unwrap();

    let mint_addr = CreateMint::new(&mut svm, &payer)
        .authority(&p2a(&payer.pubkey()))
        .decimals(6)
        .send()
        .unwrap();
    let mint = a2p(&mint_addr);

    let user_addr = p2a(&user.pubkey());
    let user_ata_addr = CreateAssociatedTokenAccount::new(&mut svm, &payer, &mint_addr)
        .owner(&user_addr)
        .send()
        .unwrap();
    let user_ata = a2p(&user_ata_addr);

    MintTo::new(&mut svm, &payer, &mint_addr, &user_ata_addr, MINT_AMOUNT)
        .send()
        .unwrap();

    let (shain_config, _) = shain_config_pda(&program_id);
    let (treasury_authority, _) = shain_treasury_pda(&program_id);
    let treasury_ata = ata_address(&treasury_authority, &mint);

    let init_accounts = shain::accounts::Initialize {
        authority: authority.pubkey(),
        shain_mint: mint,
        shain_config,
        treasury_authority,
        treasury_ata,
        system_program: Pubkey::new_from_array(solana_sdk_ids::system_program::ID.to_bytes()),
        token_program: spl_token_pubkey(),
        associated_token_program: ata_program_pubkey(),
        rent: Pubkey::new_from_array(solana_sdk_ids::sysvar::rent::ID.to_bytes()),
    };
    let init_data = shain::instruction::Initialize {
        params: InitializeParams {
            session_duration: Some(SESSION_DURATION),
            session_fee: Some(SESSION_FEE),
            min_holding: Some(min_holding),
        },
    }
    .data();
    let init_ix = Instruction {
        program_id,
        accounts: init_accounts.to_account_metas(None),
        data: init_data,
    };
    send_ix(&mut svm, &authority, init_ix, &[]).expect("initialize");

    Bootstrap {
        svm,
        program_id,
        authority,
        user,
        mint,
        user_ata,
        shain_config,
        treasury_authority,
        treasury_ata,
    }
}

fn start_session_ix(ctx: &Bootstrap, user: &Pubkey, user_ata: &Pubkey) -> Instruction {
    let (shain_session, _) = shain_session_pda(&ctx.program_id, user);
    let accounts = shain::accounts::StartSession {
        user: *user,
        shain_config: ctx.shain_config,
        shain_mint: ctx.mint,
        user_token_account: *user_ata,
        treasury_ata: ctx.treasury_ata,
        shain_session,
        token_program: spl_token_pubkey(),
        system_program: Pubkey::new_from_array(solana_sdk_ids::system_program::ID.to_bytes()),
    };
    Instruction {
        program_id: ctx.program_id,
        accounts: accounts.to_account_metas(None),
        data: shain::instruction::StartSession {}.data(),
    }
}

fn gated_action_ix(ctx: &Bootstrap, user: &Pubkey, tag: u64) -> Instruction {
    let (shain_session, _) = shain_session_pda(&ctx.program_id, user);
    let accounts = shain::accounts::GatedAction {
        user: *user,
        shain_session,
    };
    Instruction {
        program_id: ctx.program_id,
        accounts: accounts.to_account_metas(None),
        data: shain::instruction::GatedAction { tag }.data(),
    }
}

fn close_session_ix(ctx: &Bootstrap, user: &Pubkey) -> Instruction {
    let (shain_session, _) = shain_session_pda(&ctx.program_id, user);
    let accounts = shain::accounts::CloseSession {
        user: *user,
        shain_session,
    };
    Instruction {
        program_id: ctx.program_id,
        accounts: accounts.to_account_metas(None),
        data: shain::instruction::CloseSession {}.data(),
    }
}

fn read_config(svm: &LiteSVM, pda: &Pubkey) -> ShainConfig {
    let account = svm.get_account(&p2a(pda)).expect("config account");
    ShainConfig::try_deserialize(&mut &account.data[..]).expect("deserialize config")
}

fn read_session(svm: &LiteSVM, pda: &Pubkey) -> ShainSession {
    let account = svm.get_account(&p2a(pda)).expect("session account");
    ShainSession::try_deserialize(&mut &account.data[..]).expect("deserialize session")
}

#[test]
fn initialize_sets_config() {
    let ctx = bootstrap(MIN_HOLDING);
    let cfg = read_config(&ctx.svm, &ctx.shain_config);
    assert_eq!(cfg.authority, ctx.authority.pubkey());
    assert_eq!(cfg.shain_mint, ctx.mint);
    assert_eq!(cfg.treasury_ata, ctx.treasury_ata);
    assert_eq!(cfg.session_duration, SESSION_DURATION);
    assert_eq!(cfg.session_fee, SESSION_FEE);
    assert_eq!(cfg.min_holding, MIN_HOLDING);
    assert_eq!(cfg.total_sessions, 0);
    assert_eq!(cfg.total_fees_collected, 0);
    let _ = ctx.treasury_authority;
}

#[test]
fn start_session_happy_path() {
    let mut ctx = bootstrap(MIN_HOLDING);
    let user_pk = ctx.user.pubkey();
    let ix = start_session_ix(&ctx, &user_pk, &ctx.user_ata);
    send_ix(&mut ctx.svm, &ctx.user, ix, &[]).expect("start_session");

    let (session_pda, _) = shain_session_pda(&ctx.program_id, &user_pk);
    let session = read_session(&ctx.svm, &session_pda);
    assert_eq!(session.owner, user_pk);
    assert!(session.expires_at > session.started_at);
    assert_eq!(session.expires_at - session.started_at, SESSION_DURATION);
    assert_eq!(session.actions_count, 0);
    assert_eq!(session.total_sessions, 1);

    let cfg = read_config(&ctx.svm, &ctx.shain_config);
    assert_eq!(cfg.total_sessions, 1);
    assert_eq!(cfg.total_fees_collected, SESSION_FEE);

    let treasury_addr = p2a(&ctx.treasury_ata);
    let treasury =
        litesvm_token::get_spl_account::<spl_token::state::Account>(&ctx.svm, &treasury_addr)
            .expect("treasury ata");
    assert_eq!(treasury.amount, SESSION_FEE);
}

#[test]
fn start_session_fails_when_holding_below_min() {
    let mut ctx = bootstrap(MINT_AMOUNT + 1);
    let user_pk = ctx.user.pubkey();
    let ix = start_session_ix(&ctx, &user_pk, &ctx.user_ata);
    let err = send_ix(&mut ctx.svm, &ctx.user, ix, &[]).expect_err("should fail");
    assert!(
        err.contains("6001") || err.to_lowercase().contains("insufficient"),
        "err: {err}"
    );
}

#[test]
fn start_session_fails_when_already_active() {
    let mut ctx = bootstrap(MIN_HOLDING);
    let user_pk = ctx.user.pubkey();
    let ix1 = start_session_ix(&ctx, &user_pk, &ctx.user_ata);
    send_ix(&mut ctx.svm, &ctx.user, ix1, &[]).expect("first start");
    let ix2 = start_session_ix(&ctx, &user_pk, &ctx.user_ata);
    let err = send_ix(&mut ctx.svm, &ctx.user, ix2, &[]).expect_err("second should fail");
    assert!(
        err.contains("6002") || err.to_lowercase().contains("still active"),
        "err: {err}"
    );
}

#[test]
fn gated_action_increments_counter_while_active() {
    let mut ctx = bootstrap(MIN_HOLDING);
    let user_pk = ctx.user.pubkey();
    let ix = start_session_ix(&ctx, &user_pk, &ctx.user_ata);
    send_ix(&mut ctx.svm, &ctx.user, ix, &[]).expect("start");

    for i in 0..3u64 {
        let ix = gated_action_ix(&ctx, &user_pk, i);
        send_ix(&mut ctx.svm, &ctx.user, ix, &[]).expect("gated_action");
    }

    let (session_pda, _) = shain_session_pda(&ctx.program_id, &user_pk);
    let session = read_session(&ctx.svm, &session_pda);
    assert_eq!(session.actions_count, 3);
}

#[test]
fn gated_action_fails_after_expiry() {
    let mut ctx = bootstrap(MIN_HOLDING);
    let user_pk = ctx.user.pubkey();
    let start = start_session_ix(&ctx, &user_pk, &ctx.user_ata);
    send_ix(&mut ctx.svm, &ctx.user, start, &[]).expect("start");

    warp(&mut ctx.svm, SESSION_DURATION + 1);

    let ix = gated_action_ix(&ctx, &user_pk, 42);
    let err = send_ix(&mut ctx.svm, &ctx.user, ix, &[]).expect_err("should fail");
    assert!(
        err.contains("6003") || err.to_lowercase().contains("expired"),
        "err: {err}"
    );
}

#[test]
fn close_session_after_expiry_refunds_rent() {
    let mut ctx = bootstrap(MIN_HOLDING);
    let user_pk = ctx.user.pubkey();
    let start = start_session_ix(&ctx, &user_pk, &ctx.user_ata);
    send_ix(&mut ctx.svm, &ctx.user, start, &[]).expect("start");

    let (session_pda, _) = shain_session_pda(&ctx.program_id, &user_pk);
    assert!(ctx.svm.get_account(&p2a(&session_pda)).is_some());

    warp(&mut ctx.svm, SESSION_DURATION + 1);

    let close = close_session_ix(&ctx, &user_pk);
    send_ix(&mut ctx.svm, &ctx.user, close, &[]).expect("close");

    let acct = ctx.svm.get_account(&p2a(&session_pda));
    assert!(
        acct.map(|a| a.lamports == 0 || a.data.is_empty()).unwrap_or(true),
        "session account should be closed",
    );
}

#[test]
fn close_session_fails_while_active() {
    let mut ctx = bootstrap(MIN_HOLDING);
    let user_pk = ctx.user.pubkey();
    let start = start_session_ix(&ctx, &user_pk, &ctx.user_ata);
    send_ix(&mut ctx.svm, &ctx.user, start, &[]).expect("start");

    let close = close_session_ix(&ctx, &user_pk);
    let err = send_ix(&mut ctx.svm, &ctx.user, close, &[]).expect_err("close should fail while active");
    assert!(
        err.contains("6002") || err.to_lowercase().contains("still active"),
        "err: {err}"
    );
}

#[test]
fn restart_after_expiry_succeeds_and_counts() {
    let mut ctx = bootstrap(MIN_HOLDING);
    let user_pk = ctx.user.pubkey();

    let ix1 = start_session_ix(&ctx, &user_pk, &ctx.user_ata);
    send_ix(&mut ctx.svm, &ctx.user, ix1, &[]).expect("first");

    warp(&mut ctx.svm, SESSION_DURATION + 1);

    let ix2 = start_session_ix(&ctx, &user_pk, &ctx.user_ata);
    send_ix(&mut ctx.svm, &ctx.user, ix2, &[]).expect("second");

    let (session_pda, _) = shain_session_pda(&ctx.program_id, &user_pk);
    let session = read_session(&ctx.svm, &session_pda);
    assert_eq!(session.total_sessions, 2);

    let cfg = read_config(&ctx.svm, &ctx.shain_config);
    assert_eq!(cfg.total_sessions, 2);
    assert_eq!(cfg.total_fees_collected, SESSION_FEE * 2);
}
