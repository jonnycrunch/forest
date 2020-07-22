initSidebarItems({"constant":[["CHAIN_FINALITYISH","An approximation to chain state finality (should include message propagation time as well)."],["CRON_EVENT_PRE_COMMIT_EXPIRY",""],["CRON_EVENT_PROVING_PERIOD",""],["CRON_EVENT_WORKER_KEY_CHANGE",""],["ELECTION_LOOKBACK","Lookback from the current epoch for state view for leader elections."],["FAULT_DECLARATION_CUTOFF","Minimum period before a deadline's challenge window opens that a fault must be declared for that deadline. A fault declaration may appear in the challenge epoch, since it must have been posted before the epoch completed, and hence before the challenge was knowable."],["FAULT_MAX_AGE","The maximum age of a fault before the sector is terminated."],["NEW_SECTORS_PER_PERIOD_MAX","The maximum number of new sectors that may be staged by a miner during a single proving period."],["PLEDGE_VESTING_SPEC",""],["PRE_COMMIT_CHALLENGE_DELAY","Number of epochs between publishing the precommit and when the challenge for interactive PoRep is drawn used to ensure it is not predictable by miner."],["SECTORS_MAX",""],["WORKER_KEY_CHANGE_DELAY","Staging period for a miner worker key change."],["WPOST_CHALLENGE_LOOKBACK","Lookback from the deadline's challenge window opening from which to sample chain randomness for the challenge seed."],["WPOST_CHALLENGE_WINDOW","The duration of a deadline's challenge window, the period before a deadline when the challenge is available."],["WPOST_PERIOD_DEADLINES","The number of non-overlapping PoSt deadlines in each proving period."],["WPOST_PROVING_PERIOD","The period over which all a miner's active sectors will be challenged."]],"enum":[["Method","Storage Miner actor methods available"],["SupportedProofTypes","List of proof types which can be used when creating new miner actors"]],"fn":[["active_partitions_max","The maximum number of proving partitions a miner can have simultaneously active."],["assign_new_sectors","Assigns a sequence of sector numbers to deadlines by: - filling any non-full partitions, in round-robin order across the deadlines - repeatedly adding a new partition to the deadline with the fewest partitions When multiple partitions share the minimal sector count, one is chosen at random (from a seed)."],["check_supported_proof_types","List of proof types which can be used when creating new miner actors"],["compute_partitions_sector","Computes a bitfield of the sector numbers included in a sequence of partitions due at some deadline. Fails if any partition is not due at the provided deadline."],["compute_proving_period_deadline","Calculates the deadline at some epoch for a proving period and returns the deadline-related calculations."],["deadline_count","Counts the partitions (including up to one partial) and sectors at a deadline."],["max_seal_duration","Maximum duration to allow for the sealing process for seal algorithms. Dependent on algorithm and sector size"],["partitions_for_deadline","Computes the first partition index and number of sectors for a deadline. Partitions are numbered globally for the miner, not per-deadline. If the deadline has no sectors, the first partition index is the index that a partition at that deadline would have, if non-empty (and sectorCount is zero)."],["pledge_penalty_for_sector_declared_fault","Penalty to locked pledge collateral for a declared or on-going sector fault."],["pledge_penalty_for_sector_termination",""],["pledge_penalty_for_sector_undeclared_fault","Penalty to locked pledge collateral for a \"skipped\" sector or missing PoSt fault."],["precommit_deposit","Deposit per sector required at pre-commitment, refunded after the commitment is proven (else burned)."],["reward_for_consensus_slash_report",""],["to_storage_weight_desc",""],["window_post_message_partitions_max","The maximum number of partitions that may be submitted in a single message. This bounds the size of a list/set of sector numbers that might be instantiated to process a submission."]],"struct":[["Actor","Miner Actor"],["ChangeMultiaddrsParams",""],["ChangePeerIDParams",""],["ChangeWorkerAddressParams",""],["CheckSectorProvenParams",""],["ConfirmSectorProofsParams",""],["CronEventPayload",""],["DeadlineInfo","Deadline calculations with respect to a current epoch. \"Deadline\" refers to the window during which proofs may be submitted. Windows are non-overlapping ranges [Open, Close), but the challenge epoch for a window occurs before the window opens."],["Deadlines",""],["DeclareFaultsParams",""],["DeclareFaultsRecoveredParams",""],["ExtendSectorExpirationParams",""],["FaultDeclaration",""],["GetControlAddressesReturn",""],["MinerConstructorParams","Storage miner actor constructor params are defined here so the power actor can send them to the init actor to instantiate miners."],["MinerInfo","Static information about miner"],["ProveCommitSectorParams",""],["RecoveryDeclaration",""],["ReportConsensusFaultParams",""],["SectorOnChainInfo",""],["SectorOnChainInfo",""],["SectorPreCommitInfo",""],["SectorPreCommitOnChainInfo",""],["State",""],["SubmitWindowedPoStParams","Information submitted by a miner to provide a Window PoSt."],["TerminateSectorsParams",""],["VestSpec","Specification for a linear vesting schedule."],["WithdrawBalanceParams",""],["WorkerKeyChange",""]],"type":[["CronEvent",""]]});