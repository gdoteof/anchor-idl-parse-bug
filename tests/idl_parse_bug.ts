import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { IdlParseBug } from '../target/types/idl_parse_bug';

describe('idl_parse_bug', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.IdlParseBug as Program<IdlParseBug>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
