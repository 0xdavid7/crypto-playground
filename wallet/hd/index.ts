import { Address, address } from "@ton/core";
import BigNumber from "bignumber.js";

import {
  TonClient,
  WalletContractV4,
  WalletContractV5R1,
  internal,
} from "@ton/ton";

import {
  deriveEd25519Path,
  keyPairFromSeed,
  mnemonicNew,
  mnemonicToHDSeed,
  mnemonicValidate,
} from "ton-crypto";
import { MemoryStorage, StorageKey } from "./storage";

const password = "123abc";

class DerivationPath {
  static default = "m/44'/607'/0'";
  /**
   *
   * @description Because the `deriveEd25519Path` function of the `ton-crypto` package is used `hardened childs`, so just derive the path to number array.
   */
  static toNumberArray(): number[] {
    return [44, 607, 0];
  }
}

const store = new MemoryStorage();

const randomInt = (min: number, max: number) => {
  return Math.floor(Math.random() * (max - min + 1) + min);
};

const eightIndexes = () => {
  let indexes: number[] = [];
  for (let i = 0; i < 8; i++) {
    let index = randomInt(0, 23);
    while (indexes.includes(index)) {
      index = randomInt(0, 23);
    }
    indexes.push(index);
  }
  return indexes;
};

const generateMnemonic = async () => {
  try {
    let mnemonics: string[] = await mnemonicNew(24, password);
    let valid = await mnemonicValidate(mnemonics, password);
    if (!valid) {
      throw new Error("Invalid Mnemonic");
    }

    store.set({
      [StorageKey.MNEMONIC]: mnemonics,
    });

    console.log("mnemonics:", mnemonics);
  } catch (error) {
    console.log(error);
  }
};

const verifyMnemonic = async (mode: string = "skip") => {
  const mnemonics = (await store.get(StorageKey.MNEMONIC)) as any;
  if (!mnemonics) {
    return false;
  }

  const valid = await mnemonicValidate(mnemonics, password);
  if (!valid) {
    return false;
  }

  if (mode === "skip") {
    return true;
  }

  //   let indexes = eightIndexes();
  let indexes = [0, 1, 2, 3, 4, 5, 6, 7];
  let lines = 8;
  const prompt = `Let's verify your mnemonic.\nEnter the word at index #${
    indexes[8 - lines] + 1
  }: `;
  process.stdout.write(prompt);
  lines--;

  for await (const line of process.stdin) {
    if (lines === 0) {
      break;
    }
    console.log(`You typed: ${line}`);
    if (`${line}`.trim() !== mnemonics[indexes[8 - lines - 1]]) {
      return false;
    }
    process.stdout.write(
      `Enter the word at index #${indexes[8 - lines] + 1}: `
    );
    lines--;
  }

  return true;
};

const run = async () => {
  await generateMnemonic();
  const verified = await verifyMnemonic();
  if (verified) {
    console.log("Mnemonic is valid");
  } else {
    console.log("Mnemonic is not valid");
  }

  // recover steps
  // const mnemonics = (await store.get(StorageKey.MNEMONIC)) as any;

  const raw =
    "vault grant math damage slight live equip turtle taxi prize phrase notice";

  const mnemonics = raw.split(" ");

  const mnemonicHDSeed = await mnemonicToHDSeed(mnemonics, password);

  const derivedSeed = await deriveEd25519Path(
    mnemonicHDSeed,
    DerivationPath.toNumberArray()
  );

  const { publicKey, secretKey } = await keyPairFromSeed(derivedSeed);

  console.log({
    publicKey: Buffer.from(publicKey).toString("hex"),
    secretKey: Buffer.from(secretKey).toString("hex"),
  });
};

// run();

const simulateKeyPair = async () => {
  const friendly = "EQAPvJW6WmXgNd4vTjtgaqgakKpiujezcCGCPpLBOX78EkS5";
  const add = address(friendly);
  console.log({ add: add.toRawString() });

  const pk =
    "0:0fbc95ba5a65e035de2f4e3b606aa81a90aa62ba37b37021823e92c1397efc12";
  const pk2 =
    "0:05eaa5f1bd1ca8844d14dabf0344b85f269625821aa17d251eb7729b5140e860";

  const pk3 =
    "0:0cf416d62d662d844e3fb7b7ea4de53af5a2a0eefe71f9ff0cf7a0f19aa13550";

  // 0:
  // 0:05eaa5f1bd1ca8844d14dabf0344b85f269625821aa17d251eb7729b5140e860

  //encrypt_pk: 0cf416d62d662d844e3fb7b7ea4de53af5a2a0eefe71f9ff0cf7a0f19aa13550
  console.log({
    wallet: Address.parse(pk).toString(),
  });

  console.log({
    wallet2: Address.parseRaw(pk2).toString(),
  });

  console.log({
    wallet3: Address.parse(pk3).toString(),
  });

  const balanceBN = new BigNumber("1000000000");
  const res = balanceBN.dividedBy(1e9).toFixed(4);
  console.log({ res });
};

const simulateCalFee = async () => {
  const publicKey = Buffer.from(
    "0cf416d62d662d844e3fb7b7ea4de53af5a2a0eefe71f9ff0cf7a0f19aa13550"
  );

  const secretKey = Buffer.from(
    "fbd828712bb750ac002c7012696031985c6c72a77d43ae1986652381cea9780c0cf416d62d662d844e3fb7b7ea4de53af5a2a0eefe71f9ff0cf7a0f19aa13550"
  );

  // Create Client
  const client = new TonClient({
    endpoint: "https://testnet.toncenter.com/api/v2/jsonRPC",
  });

  // Create wallet contract
  let workchain = 0; // Usually you need a workchain 0
  let wallet = WalletContractV4.create({
    workchain,
    publicKey,
  });
  let contract = client.open(wallet);

  // Get balance
  let balance: bigint = await contract.getBalance();

  // Create a transfer
  let seqno: number = await contract.getSeqno();
  let transfer = await contract.createTransfer({
    seqno,
    secretKey,
    messages: [
      internal({
        value: "1.5",
        to: "EQCD39VS5jcptHL8vMjEXrzGaRcCVYto7HUn4bpAOg8xqB2N",
        body: "Hello world",
      }),
    ],
  });
};

const main = async () => {
  const source = "EQCio9-JHjka8GWq0hRWsdLM4ctsgZvqhKSx8aObTFlOoazz";
  console.log({
    raw: fromFriendlyToRaw(source),
  });

  const publicKey = Buffer.from(
    "0cf416d62d662d844e3fb7b7ea4de53af5a2a0eefe71f9ff0cf7a0f19aa13550",
    "hex"
  );

  // dbb2393458726b2d0bce9d5968a8e810a0b886099026ab3dd8967203b90525f4

  // dbb2393458726b2d0bce9d5968a8e810a0b886099026ab3dd8967203b90525f4
  console.log("V5R1");
  const wallet = WalletContractV5R1.create({
    workChain: 0,
    walletId: {
      networkGlobalId: -3,
    },
    publicKey,
  });

  const wallet1 = WalletContractV5R1.create({
    walletId: {
      networkGlobalId: -239,
    },
    publicKey,
  });

  const wallet2 = WalletContractV5R1.create({
    workChain: 0,
    publicKey,
  });

  console.log({
    wallet: wallet.address.toRawString(),
    wallet1: wallet1.address.toRawString(),
    wallet2: wallet2.address.toRawString(),
  });

  console.log("V4");

  const wallet3 = WalletContractV4.create({
    workchain: 0,
    publicKey,
  });

  const wallet4 = WalletContractV4.create({
    workchain: 0,
    publicKey,
  });

  const wallet5 = WalletContractV4.create({
    workchain: 0,
    publicKey,
  });

  console.log({
    wallet3: wallet3.address.toRawString(),
    wallet4: wallet4.address.toRawString(),
    wallet5: wallet5.address.toRawString(),
  });
  
  console.log({
    friendly: fromRawToFriendly(wallet1.address.toRawString()),
  })
};

const fromFriendlyToRaw = (source: string) => {
  return address(source).toRawString();
};

const fromRawToFriendly = (source: string) => {
  return address(source).toString();
};

main();
